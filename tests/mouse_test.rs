use rmm::config::Config;
use rmm::state::AppState;
use std::sync::{Arc, Mutex};

#[test]
fn test_mouse_movement_direction() {
    // Test that mouse movement direction alternates
    let mut state = AppState::new();

    assert_eq!(state.move_direction, 1);

    // Simulate direction toggle
    state.move_direction *= -1;
    assert_eq!(state.move_direction, -1);

    state.move_direction *= -1;
    assert_eq!(state.move_direction, 1);
}

#[test]
fn test_movement_delta_configuration() {
    // Test that movement delta can be configured
    let config = Config {
        movement_delta: 5,
        ..Default::default()
    };

    assert_eq!(config.movement_delta, 5);

    let config2 = Config {
        movement_delta: 20,
        ..Default::default()
    };

    assert_eq!(config2.movement_delta, 20);
}

#[test]
fn test_inactivity_threshold() {
    // Test inactivity threshold configuration
    let config = Config {
        inactivity_threshold: 300,
        ..Default::default()
    };

    assert_eq!(config.inactivity_threshold, 300);
}

#[test]
fn test_error_count_tracking() {
    // Test that error count can be tracked and reset
    let mut state = AppState::new();

    assert_eq!(state.error_count, 0);

    // Simulate errors
    state.error_count += 1;
    assert_eq!(state.error_count, 1);

    state.error_count += 1;
    assert_eq!(state.error_count, 2);

    // Reset errors
    state.error_count = 0;
    assert_eq!(state.error_count, 0);
}

#[test]
fn test_max_errors_configuration() {
    // Test max errors configuration
    let config = Config {
        max_errors: 5,
        ..Default::default()
    };

    assert_eq!(config.max_errors, 5);

    let mut state = AppState::new();

    // Simulate reaching max errors
    for _ in 0..config.max_errors {
        state.error_count += 1;
    }

    assert_eq!(state.error_count, config.max_errors);
}

#[test]
fn test_state_running_flag() {
    // Test that running flag can be toggled
    let state = Arc::new(Mutex::new(AppState::new()));

    {
        let s = state.lock().unwrap();
        assert_eq!(s.is_running, false);
    }

    {
        let mut s = state.lock().unwrap();
        s.is_running = true;
    }

    {
        let s = state.lock().unwrap();
        assert_eq!(s.is_running, true);
    }
}

#[test]
fn test_heartbeat_interval_configuration() {
    // Test heartbeat interval configuration
    let config = Config {
        heartbeat_interval: 60,
        ..Default::default()
    };

    assert_eq!(config.heartbeat_interval, 60);
}
