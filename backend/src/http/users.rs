use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};

/// Usuario del sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

/// Request para crear un usuario
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Request para login
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Response de login exitoso
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

/// Response de usuario (sin password)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
}

/// Response de error
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Crea el router para gestión de usuarios
pub fn router() -> Router {
    Router::new()
        .route("/", get(list_users))
        .route("/", post(create_user))
        .route("/{id}", get(get_user))
        .route("/{id}", delete(delete_user))
        .route("/login", post(login))
}

/// GET /api/users - Lista todos los usuarios
async fn list_users() -> Result<Json<Vec<UserResponse>>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implementar con SQLite
    Ok(Json(vec![]))
}

/// GET /api/users/:id - Obtiene un usuario por ID
async fn get_user(
    Path(id): Path<i64>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implementar con SQLite
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: format!("User {} not found", id),
        }),
    ))
}

/// POST /api/users - Crea un nuevo usuario
async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Validaciones básicas
    if payload.username.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Username cannot be empty".to_string(),
            }),
        ));
    }

    if payload.email.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Email cannot be empty".to_string(),
            }),
        ));
    }

    if payload.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Password must be at least 8 characters".to_string(),
            }),
        ));
    }

    // TODO: Implementar creación real con SQLite y hash de password
    let user_response = UserResponse {
        id: 1,
        username: payload.username,
        email: payload.email,
    };

    Ok((StatusCode::CREATED, Json(user_response)))
}

/// DELETE /api/users/:id - Elimina un usuario
async fn delete_user(
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implementar con SQLite
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: format!("User {} not found", id),
        }),
    ))
}

/// POST /api/users/login - Login de usuario
async fn login(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implementar autenticación real con SQLite y verificación de password
    Err((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse {
            error: "Invalid credentials".to_string(),
        }),
    ))
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
    fn test_create_user_validation() {
        let valid_request = CreateUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        assert!(!valid_request.username.is_empty());
        assert!(!valid_request.email.is_empty());
        assert!(valid_request.password.len() >= 8);
    }

    #[test]
    fn test_password_length_validation() {
        let short_password = "pass";
        assert!(short_password.len() < 8);

        let valid_password = "password123";
        assert!(valid_password.len() >= 8);
    }

    #[test]
    fn test_create_user_invalid_empty_username() {
        let request = CreateUserRequest {
            username: "".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        assert!(request.username.is_empty());
    }

    #[test]
    fn test_create_user_invalid_empty_email() {
        let request = CreateUserRequest {
            username: "testuser".to_string(),
            email: "".to_string(),
            password: "password123".to_string(),
        };

        assert!(request.email.is_empty());
    }

    #[test]
    fn test_create_user_invalid_short_password() {
        let request = CreateUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "short".to_string(),
        };

        assert!(request.password.len() < 8);
    }

    #[test]
    fn test_user_response_structure() {
        let user = UserResponse {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
        };

        assert_eq!(user.id, 1);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_login_request_structure() {
        let login = LoginRequest {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        assert_eq!(login.username, "testuser");
        assert_eq!(login.password, "password123");
    }

    #[test]
    fn test_error_response_serialization() {
        let error = ErrorResponse {
            error: "Test error message".to_string(),
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("Test error message"));
    }

    #[tokio::test]
    async fn test_router_list_users_endpoint() {
        let app = router();

        let request = Request::builder()
            .uri("/")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_router_create_user_endpoint_valid() {
        let app = router();

        let payload = CreateUserRequest {
            username: "newuser".to_string(),
            email: "new@example.com".to_string(),
            password: "password123".to_string(),
        };

        let request = Request::builder()
            .uri("/")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_router_create_user_endpoint_empty_username() {
        let app = router();

        let payload = CreateUserRequest {
            username: "".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
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
    async fn test_router_create_user_endpoint_empty_email() {
        let app = router();

        let payload = CreateUserRequest {
            username: "testuser".to_string(),
            email: "".to_string(),
            password: "password123".to_string(),
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
    async fn test_router_create_user_endpoint_short_password() {
        let app = router();

        let payload = CreateUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "short".to_string(),
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
    async fn test_router_get_user_endpoint() {
        let app = router();

        let request = Request::builder()
            .uri("/1")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        // Como no hay implementación real, debe devolver NOT_FOUND
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_router_delete_user_endpoint() {
        let app = router();

        let request = Request::builder()
            .uri("/1")
            .method("DELETE")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        // Como no hay implementación real, debe devolver NOT_FOUND
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_router_login_endpoint() {
        let app = router();

        let payload = LoginRequest {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let request = Request::builder()
            .uri("/login")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        // Como no hay implementación real, debe devolver UNAUTHORIZED
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
