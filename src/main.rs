mod error;
mod state;
mod config;
mod activity;

use error::Result;
use tracing::info;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
    
    // Start activity monitoring
    activity::start_monitoring(Arc::clone(&state));
    info!("Activity monitoring started");
    
    // Keep the application running
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}