use std::sync::{Arc, Mutex};

// Import modules from the main crate
// Note: These tests verify the integration between different components

#[test]
fn test_config_default_values() {
    // Test that default configuration has expected values
    use rmm::config::Config;

    let config = Config::default();

    assert_eq!(config.heartbeat_interval, 10);
    assert_eq!(config.worker_interval, 10);
    assert_eq!(config.inactivity_threshold, 10);
    assert_eq!(config.movement_delta, 10);
    assert_eq!(config.max_errors, 10);
    assert_eq!(config.auto_start, false);
}

#[test]
fn test_config_serialization() {
    // Test that configuration can be serialized and deserialized
    use rmm::config::Config;

    let config = Config::default();
    let json = serde_json::to_string(&config).expect("Failed to serialize config");
    let deserialized: Config = serde_json::from_str(&json).expect("Failed to deserialize config");

    assert_eq!(config.heartbeat_interval, deserialized.heartbeat_interval);
    assert_eq!(config.worker_interval, deserialized.worker_interval);
    assert_eq!(
        config.inactivity_threshold,
        deserialized.inactivity_threshold
    );
}

#[test]
fn test_state_initialization() {
    // Test that AppState initializes correctly
    use rmm::state::AppState;

    let state = AppState::new();

    assert_eq!(state.is_running, false);
    assert_eq!(state.move_direction, 1);
    assert_eq!(state.error_count, 0);
}

#[test]
fn test_state_thread_safety() {
    // Test that AppState can be safely shared between threads
    use rmm::state::AppState;
    use std::thread;

    let state = Arc::new(Mutex::new(AppState::new()));
    let state_clone = Arc::clone(&state);

    let handle = thread::spawn(move || {
        let mut s = state_clone.lock().unwrap();
        s.is_running = true;
        s.error_count = 5;
    });

    handle.join().unwrap();

    let s = state.lock().unwrap();
    assert_eq!(s.is_running, true);
    assert_eq!(s.error_count, 5);
}

#[test]
fn test_error_types() {
    // Test that different error types can be created
    use rmm::error::RmmError;

    let mouse_error = RmmError::MouseControl("Test error".to_string());
    assert!(mouse_error.to_string().contains("Mouse control error"));

    let config_error = RmmError::Config("Test config error".to_string());
    assert!(config_error.to_string().contains("Configuration error"));

    let platform_error = RmmError::Platform("Test platform error".to_string());
    assert!(platform_error
        .to_string()
        .contains("Platform-specific error"));
}

#[test]
fn test_state_activity_tracking() {
    // Test that state can track activity timing
    use rmm::state::AppState;
    use std::thread;
    use std::time::Duration;

    let state = AppState::new();

    thread::sleep(Duration::from_millis(10));

    // In a real scenario, activity would update last_activity
    // Here we just verify the initial state is set
    assert!(state.last_activity.elapsed() >= Duration::from_millis(10));
}

#[test]
fn test_config_custom_values() {
    // Test creating config with custom values
    use rmm::config::Config;

    let config = Config {
        heartbeat_interval: 60,
        worker_interval: 30,
        inactivity_threshold: 300,
        movement_delta: 5,
        max_errors: 3,
        auto_start: true,
    };

    assert_eq!(config.heartbeat_interval, 60);
    assert_eq!(config.worker_interval, 30);
    assert_eq!(config.inactivity_threshold, 300);
    assert_eq!(config.movement_delta, 5);
    assert_eq!(config.max_errors, 3);
    assert_eq!(config.auto_start, true);
}

#[test]
fn test_state_move_direction_toggle() {
    // Test that move direction can be toggled
    use rmm::state::AppState;

    let mut state = AppState::new();
    assert_eq!(state.move_direction, 1);

    state.move_direction = -1;
    assert_eq!(state.move_direction, -1);

    state.move_direction = 1;
    assert_eq!(state.move_direction, 1);
}

#[test]
fn test_error_count_increment() {
    // Test error count can be incremented
    use rmm::state::AppState;

    let mut state = AppState::new();
    assert_eq!(state.error_count, 0);

    state.error_count += 1;
    assert_eq!(state.error_count, 1);

    state.error_count += 1;
    assert_eq!(state.error_count, 2);
}
