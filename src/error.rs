use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum RmmError {
    #[error("Mouse control error: {0}")]
    MouseControl(String),
    #[error("Activity monitoring error: {0}")]
    ActivityMonitor(String),
    #[error("System tray error: {0}")]
    SystemTray(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Platform-specific error: {0}")]
    Platform(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, RmmError>;