use crate::error::{Result, RmmError};
use crate::state::SharedState;
use enigo::{Enigo, Mouse, Settings};
use std::time::Instant;
use tracing::{error, info, warn};

pub struct MouseController {
    enigo: Enigo,
}

impl MouseController {
    pub fn new() -> Result<Self> {
        let enigo = Enigo::new(&Settings::default())
            .map_err(|e| RmmError::MouseControl(format!("Failed to initialize Enigo: {:?}", e)))?;
        Ok(Self { enigo })
    }

    pub fn get_position(&mut self) -> Result<(i32, i32)> {
        self.enigo
            .location()
            .map_err(|e| RmmError::MouseControl(format!("Failed to get mouse position: {:?}", e)))
    }

    pub fn move_mouse(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo
            .move_mouse(x, y, enigo::Coordinate::Abs)
            .map_err(|e| RmmError::MouseControl(format!("Failed to move mouse: {:?}", e)))
    }

    pub fn verify_position(&mut self, expected_x: i32, expected_y: i32) -> Result<bool> {
        let (actual_x, actual_y) = self.get_position()?;
        let tolerance = 5; // Allow 5 pixel tolerance
        let x_match = (actual_x - expected_x).abs() <= tolerance;
        let y_match = (actual_y - expected_y).abs() <= tolerance;
        Ok(x_match && y_match)
    }
}

pub fn check_and_move(state: SharedState, inactivity_threshold: u64) -> Result<()> {
    let mut controller = MouseController::new()?;
    
    let (should_move, direction) = {
        let state_guard = state.lock().map_err(|e| {
            RmmError::MouseControl(format!("Failed to lock state: {}", e))
        })?;
        
        if !state_guard.is_running {
            return Ok(());
        }
        
        let inactive_duration = state_guard.last_activity.elapsed().as_secs();
        let should_move = inactive_duration >= inactivity_threshold;
        
        (should_move, state_guard.move_direction)
    };
    
    if !should_move {
        return Ok(());
    }
    
    // Get current position
    let (current_x, current_y) = controller.get_position()?;
    info!("Current mouse position: ({}, {})", current_x, current_y);
    
    // Calculate new position
    let delta = 10 * direction;
    let new_x = current_x + delta;
    let new_y = current_y + delta;
    
    info!("Moving mouse by {} pixels to ({}, {})", delta, new_x, new_y);
    
    // Move mouse
    controller.move_mouse(new_x, new_y)?;
    
    // Verify movement
    std::thread::sleep(std::time::Duration::from_millis(100));
    let verified = controller.verify_position(new_x, new_y)?;
    
    let mut state_guard = state.lock().map_err(|e| {
        RmmError::MouseControl(format!("Failed to lock state: {}", e))
    })?;
    
    if verified {
        info!("Mouse movement verified successfully");
        state_guard.last_moved = Instant::now();
        state_guard.move_direction *= -1; // Alternate direction
        state_guard.error_count = 0;
    } else {
        state_guard.error_count += 1;
        warn!("Mouse movement verification failed (error count: {})", state_guard.error_count);
        
        if state_guard.error_count >= 10 {
            error!("Mouse movement failed 10 times! Please check system permissions.");
        }
    }
    
    Ok(())
}
