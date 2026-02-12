use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct AppState {
    pub is_running: bool,
    pub last_activity: Instant,
    pub last_moved: Instant,
    pub move_direction: i32,
    pub error_count: u32,
}

impl AppState {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            is_running: false,
            last_activity: now,
            last_moved: now,
            move_direction: 1,
            error_count: 0,
        }
    }
}

pub type SharedState = Arc<Mutex<AppState>>;
