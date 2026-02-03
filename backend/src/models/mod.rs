pub mod quadlet;

pub use quadlet::{Quadlet, QuadletType};
pub type Error = Box<dyn std::error::Error + Send + Sync>;
