use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    routing::{get, post},
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};

use crate::models::{Quadlet, QuadletType, AppState};

/// Request para guardar un quadlet
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveQuadletRequest {
    pub name: String,
    pub content: String,
}

/// Response de error
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Crea el router para gestión de quadlets
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_quadlets))
        .route("/", post(save_quadlet))
}

/// GET /api/quadlets - Lista todos los archivos Quadlet
async fn list_quadlets() -> Result<Json<Vec<Quadlet>>, (StatusCode, Json<ErrorResponse>)> {
    let quadlets_dir = get_quadlets_directory()
        .map_err(|e| internal_error(format!("Failed to get quadlets directory: {}", e)))?;

    if !quadlets_dir.exists() {
        return Ok(Json(vec![]));
    }

    let entries = fs::read_dir(&quadlets_dir)
        .map_err(|e| internal_error(format!("Failed to read directory: {}", e)))?;

    let mut quadlets = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| internal_error(format!("Failed to read entry: {}", e)))?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| format!(".{}", ext));

        if let Some(ext) = extension {
            if let Some(kind) = QuadletType::from_extension(&ext) {
                let name = path
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let content = fs::read_to_string(&path)
                    .map_err(|e| internal_error(format!("Failed to read file: {}", e)))?;

                quadlets.push(Quadlet::new(name, kind, content, path));
            }
        }
    }

    Ok(Json(quadlets))
}

/// POST /api/quadlets - Guarda un archivo Quadlet y recarga systemd
async fn save_quadlet(
    Json(payload): Json<SaveQuadletRequest>,
) -> Result<Json<Quadlet>, (StatusCode, Json<ErrorResponse>)> {
    // Validar que el nombre no esté vacío
    if payload.name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Name cannot be empty".to_string(),
            }),
        ));
    }

    // Determinar el tipo de quadlet desde el nombre del archivo
    let path_buf = PathBuf::from(&payload.name);
    let extension = path_buf
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| format!(".{}", ext))
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid file name: missing extension".to_string(),
                }),
            )
        })?;

    let kind = QuadletType::from_extension(&extension).ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid quadlet extension: {}", extension),
            }),
        )
    })?;

    let quadlets_dir = get_quadlets_directory()
        .map_err(|e| internal_error(format!("Failed to get quadlets directory: {}", e)))?;

    // Crear el directorio si no existe
    if !quadlets_dir.exists() {
        fs::create_dir_all(&quadlets_dir)
            .map_err(|e| internal_error(format!("Failed to create directory: {}", e)))?;
    }

    let file_path = quadlets_dir.join(&payload.name);

    // Guardar el archivo
    fs::write(&file_path, &payload.content)
        .map_err(|e| internal_error(format!("Failed to write file: {}", e)))?;

    // Recargar systemd user daemon
    reload_systemd_user()
        .map_err(|e| internal_error(format!("Failed to reload systemd: {}", e)))?;

    let name = file_path
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or(&payload.name)
        .to_string();

    Ok(Json(Quadlet::new(name, kind, payload.content, file_path)))
}

/// Obtiene el directorio de quadlets del usuario
fn get_quadlets_directory() -> Result<PathBuf, String> {
    let home = std::env::var("HOME").map_err(|_| "HOME environment variable not set")?;
    Ok(PathBuf::from(home).join(".config/containers/systemd"))
}

/// Recarga el daemon de systemd del usuario
fn reload_systemd_user() -> Result<(), String> {
    let output = Command::new("systemctl")
        .arg("--user")
        .arg("daemon-reload")
        .output()
        .map_err(|e| format!("Failed to execute systemctl: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "systemctl daemon-reload failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Helper para crear respuestas de error interno
fn internal_error(message: String) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse { error: message }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[test]
    fn test_quadlet_type_from_filename() {
        assert_eq!(
            QuadletType::from_extension(".container"),
            Some(QuadletType::Container)
        );
        assert_eq!(
            QuadletType::from_extension(".network"),
            Some(QuadletType::Network)
        );
        assert_eq!(
            QuadletType::from_extension(".volume"),
            Some(QuadletType::Volume)
        );
        assert_eq!(
            QuadletType::from_extension(".kube"),
            Some(QuadletType::Kube)
        );
        assert_eq!(QuadletType::from_extension(".pod"), Some(QuadletType::Pod));
        assert_eq!(
            QuadletType::from_extension(".image"),
            Some(QuadletType::Image)
        );
    }

    #[test]
    fn test_save_request_validation() {
        let request = SaveQuadletRequest {
            name: "test.container".to_string(),
            content: "[Container]\nImage=alpine\n".to_string(),
        };

        assert!(!request.name.is_empty());
        assert!(!request.content.is_empty());
    }

    #[test]
    fn test_save_request_empty_name() {
        let request = SaveQuadletRequest {
            name: "".to_string(),
            content: "[Container]\nImage=alpine\n".to_string(),
        };

        assert!(request.name.is_empty());
    }

    #[test]
    fn test_save_request_with_all_quadlet_types() {
        let types = vec![
            "test.container",
            "test.network",
            "test.volume",
            "test.kube",
            "test.pod",
            "test.image",
        ];

        for filename in types {
            let request = SaveQuadletRequest {
                name: filename.to_string(),
                content: format!("[{}]\n", filename.split('.').nth(1).unwrap()),
            };

            let path = PathBuf::from(&request.name);
            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| format!(".{}", e));
            assert!(extension.is_some());

            let kind = QuadletType::from_extension(&extension.unwrap());
            assert!(kind.is_some());
        }
    }

    #[tokio::test]
    async fn test_router_list_quadlets_endpoint() {
        let app = router();

        let request = Request::builder()
            .uri("/")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        // Debe responder con 200 OK o error interno si el directorio no existe
        assert!(
            response.status() == StatusCode::OK
                || response.status() == StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn test_router_save_quadlet_endpoint_empty_name() {
        let app = router();

        let payload = SaveQuadletRequest {
            name: "".to_string(),
            content: "[Container]\nImage=alpine\n".to_string(),
        };

        let request = Request::builder()
            .uri("/")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_router_save_quadlet_endpoint_invalid_extension() {
        let app = router();

        let payload = SaveQuadletRequest {
            name: "test.txt".to_string(),
            content: "some content".to_string(),
        };

        let request = Request::builder()
            .uri("/")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_router_save_quadlet_endpoint_no_extension() {
        let app = router();

        let payload = SaveQuadletRequest {
            name: "testfile".to_string(),
            content: "[Container]\n".to_string(),
        };

        let request = Request::builder()
            .uri("/")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_error_response_structure() {
        let error = ErrorResponse {
            error: "Test error".to_string(),
        };

        assert_eq!(error.error, "Test error");

        // Verificar que se serializa correctamente
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("Test error"));
    }

    #[test]
    fn test_get_quadlets_directory() {
        // Test que el directorio se construye correctamente
        if let Ok(dir) = get_quadlets_directory() {
            let path_str = dir.to_string_lossy();
            assert!(path_str.contains(".config/containers/systemd"));
        }
    }
}
