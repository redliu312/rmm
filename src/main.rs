mod activity;
mod config;
mod error;
mod mouse;
mod state;
mod tray;

use error::Result;
use std::fs::{self, OpenOptions};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tracing::info;
use tracing_subscriber::fmt::writer::MakeWriterExt;

fn main() -> Result<()> {
    // Create log directory and file
    let log_dir = directories::ProjectDirs::from("com", "rmm", "rmm")
        .map(|dirs| dirs.data_local_dir().to_path_buf())
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            std::path::PathBuf::from(home).join("Library/Logs")
        });

    fs::create_dir_all(&log_dir)?;
    let log_path = log_dir.join("rmm.log");

    // Open log file in append mode
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .expect("Failed to open log file");

    // Clone for the startup message
    let log_path_display = log_path.clone();

    // Initialize logging to both stdout and file
    let file_writer = log_file.with_max_level(tracing::Level::INFO);
    let stdout_writer = std::io::stdout.with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_writer(file_writer.and(stdout_writer))
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_ansi(false) // Disable ANSI colors in log file
        .init();

    // Log startup with file location
    info!("Starting RMM 2");
    info!("Log file: {}", log_path_display.display());

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
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(heartbeat_interval));
        if let Err(e) = mouse::check_and_move(Arc::clone(&heartbeat_state), inactivity_threshold) {
            tracing::error!("Error in heartbeat: {:?}", e);
        }
    });
    info!("Heartbeat started ({}s interval)", heartbeat_interval);

    // Create system tray icon (must be on main thread for macOS)
    // This will block the main thread and keep the tray alive
    let _tray = tray::create_tray();

    // Keep the main thread alive to maintain the tray icon
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
