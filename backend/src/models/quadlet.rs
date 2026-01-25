use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Tipo de archivo Quadlet soportado
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuadletType {
    Container,
    Network,
    Volume,
    Kube,
    Pod,
    Image,
}

impl QuadletType {
    /// Devuelve la extensión de archivo asociada a este tipo
    pub fn extension(&self) -> &'static str {
        match self {
            QuadletType::Container => ".container",
            QuadletType::Network => ".network",
            QuadletType::Pod => ".pod",
            QuadletType::Image => ".image",
            QuadletType::Volume => ".volume",
            QuadletType::Kube => ".kube",
        }
    }

    /// Intenta determinar el tipo de Quadlet desde una extensión de archivo
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            ".container" => Some(QuadletType::Container),
            ".network" => Some(QuadletType::Network),
            ".pod" => Some(QuadletType::Pod),
            ".image" => Some(QuadletType::Image),
            ".volume" => Some(QuadletType::Volume),
            ".kube" => Some(QuadletType::Kube),
            _ => None,
        }
    }

    /// Devuelve una representación en string del tipo
    pub fn as_str(&self) -> &'static str {
        match self {
            QuadletType::Container => "container",
            QuadletType::Network => "network",
            QuadletType::Pod => "pod",
            QuadletType::Image => "image",
            QuadletType::Volume => "volume",
            QuadletType::Kube => "kube",
        }
    }
}

/// Representa un archivo Quadlet de Podman
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quadlet {
    /// Nombre del archivo (sin extensión)
    pub name: String,
    
    /// Tipo de Quadlet
    pub kind: QuadletType,
    
    /// Contenido del archivo
    pub content: String,
    
    /// Ruta completa al archivo en el sistema de archivos
    pub path: PathBuf,
}

impl Quadlet {
    /// Crea una nueva instancia de Quadlet
    pub fn new(name: String, kind: QuadletType, content: String, path: PathBuf) -> Self {
        Self {
            name,
            kind,
            content,
            path,
        }
    }

    /// Devuelve el nombre completo del archivo (con extensión)
    pub fn full_name(&self) -> String {
        format!("{}{}", self.name, self.kind.extension())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadlet_type_extension() {
        assert_eq!(QuadletType::Container.extension(), ".container");
        assert_eq!(QuadletType::Network.extension(), ".network");
        assert_eq!(QuadletType::Volume.extension(), ".volume");
        assert_eq!(QuadletType::Kube.extension(), ".kube");
        assert_eq!(QuadletType::Pod.extension(), ".pod");
        assert_eq!(QuadletType::Image.extension(), ".image");
    }

    #[test]
    fn test_quadlet_type_from_extension() {
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
        assert_eq!(
            QuadletType::from_extension(".pod"),
            Some(QuadletType::Pod)
        );
        assert_eq!(
            QuadletType::from_extension(".image"),
            Some(QuadletType::Image)
        );
        assert_eq!(QuadletType::from_extension(".txt"), None);
    }

    #[test]
    fn test_quadlet_type_as_str() {
        assert_eq!(QuadletType::Container.as_str(), "container");
        assert_eq!(QuadletType::Network.as_str(), "network");
        assert_eq!(QuadletType::Volume.as_str(), "volume");
        assert_eq!(QuadletType::Kube.as_str(), "kube");
        assert_eq!(QuadletType::Pod.as_str(), "pod");
        assert_eq!(QuadletType::Image.as_str(), "image");
    }

    #[test]
    fn test_quadlet_creation() {
        let quadlet = Quadlet::new(
            "my-app".to_string(),
            QuadletType::Container,
            "[Container]\nImage=alpine\n".to_string(),
            PathBuf::from("/home/user/.config/containers/systemd/my-app.container"),
        );

        assert_eq!(quadlet.name, "my-app");
        assert_eq!(quadlet.kind, QuadletType::Container);
        assert_eq!(quadlet.full_name(), "my-app.container");
    }

    #[test]
    fn test_quadlet_serialization() {
        let quadlet = Quadlet::new(
            "test".to_string(),
            QuadletType::Network,
            "[Network]\n".to_string(),
            PathBuf::from("/test.network"),
        );

        let json = serde_json::to_string(&quadlet).unwrap();
        assert!(json.contains("\"name\":\"test\""));
        assert!(json.contains("\"kind\":\"network\""));
    }
}
