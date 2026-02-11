use crate::error::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub heartbeat_interval: u64,
    pub worker_interval: u64,
    pub inactivity_threshold: u64,
    pub movement_delta: i32,
    pub max_errors: u32,
    pub auto_start: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            heartbeat_interval: 60,
            worker_interval: 10,
            inactivity_threshold: 60,
            movement_delta: 10,
            max_errors: 10,
            auto_start: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        ProjectDirs::from("com", "rmm", "rmm")
            .map(|dirs| dirs.config_dir().join("config.json"))
            .ok_or_else(|| crate::error::RmmError::Config("Cannot find config directory".into()))
    }
}