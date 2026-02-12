// Library exports for testing and external use

pub mod config;
pub mod error;
pub mod state;

// Re-export commonly used types
pub use config::Config;
pub use error::{Result, RmmError};
pub use state::{AppState, SharedState};
