mod error;
mod state;
mod config;
mod activity;
mod mouse;

use error::Result;
use tracing::info;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    // Initialize logging (use RUST_LOG env or default to INFO)
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    // Log startup
    info!("Starting RMM 2");
    
    // Load configuration (returns error on failure)
    let config = config::Config::load()?;
    // Create shared, thread-safe application state
    let state = Arc::new(Mutex::new(state::AppState::new()));
    
    // Set running to true
    {
        let mut state_guard = state.lock().unwrap();
        state_guard.is_running = true;
    }
    
    info!("Configuration loaded");
    info!("State initialized");
    
    // Start activity monitoring in background (uses shared `state`)
    activity::start_monitoring(Arc::clone(&state));
    info!("Activity monitoring started");
    
    // Heartbeat loop - check every heartbeat_interval seconds
    let heartbeat_state = Arc::clone(&state);
    let inactivity_threshold = config.inactivity_threshold;
    let heartbeat_interval = config.heartbeat_interval;
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(heartbeat_interval));
            if let Err(e) = mouse::check_and_move(Arc::clone(&heartbeat_state), inactivity_threshold) {
                tracing::error!("Error in heartbeat: {:?}", e);
            }
        }
    });
    info!("Heartbeat started ({}s interval)", heartbeat_interval);
    
    // Keep the main thread alive while background monitoring runs
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}