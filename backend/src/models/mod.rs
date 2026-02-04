mod quadlet;
mod response;
mod paginable;

pub use quadlet::{Quadlet, QuadletType};
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub struct AppState {
    pub secret: String,
    pub static_dir: String,
}
