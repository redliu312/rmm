mod error;
mod state;
mod config;

use error::Result;
use tracing::info;
use std::sync::{Arc, Mutex};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    info!("Starting RMM 2");
    
    let config = config::Config::load()?;
    let state = Arc::new(Mutex::new(state::AppState::new()));
    
    info!("Configuration loaded");
    info!("State initialized");
    
    Ok(())
}