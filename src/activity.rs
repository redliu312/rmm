use crate::state::SharedState;
use rdev::{listen, Event, EventType};
use std::time::Instant;
use tracing::{debug, error, info};

pub fn start_monitoring(state: SharedState) {
    std::thread::spawn(move || {
        info!("Starting activity monitoring");

        let callback = move |event: Event| match event.event_type {
            EventType::KeyPress(key) => {
                info!("Key pressed: {:?}", key);
                if let Ok(mut state) = state.lock() {
                    state.last_activity = Instant::now();
                }
            }
            EventType::MouseMove { x, y } => {
                debug!("Mouse moved to: ({}, {})", x, y);
                if let Ok(mut state) = state.lock() {
                    state.last_activity = Instant::now();
                }
            }
            EventType::ButtonPress(button) => {
                info!("Mouse button pressed: {:?}", button);
                if let Ok(mut state) = state.lock() {
                    state.last_activity = Instant::now();
                }
            }
            _ => {}
        };

        if let Err(e) = listen(callback) {
            error!("Error in activity monitoring: {:?}", e);
        }
    });
}
