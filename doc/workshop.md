# Workshop: Rust Mouse Mover Initialization Steps

## Phase 1: Project Initialization

### 1. Initialize Cargo Project

```bash
cargo init --name rmm
```

### 2. Configure Cargo.toml

```toml
[package]
name = "rmm"
version = "0.1.0"
edition = "2021"

[dependencies]
enigo = "0.2"
rdev = "0.5"
tray-item = "0.10"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
directories = "5.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"

[target.'cfg(target_os = "macos")'.dependencies]
core-foundation = "0.9"
io-kit-sys = "0.4"

[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.52", features = [
    "Win32_Foundation",
    "Win32_System_Power",
    "Win32_UI_WindowsAndMessaging",
] }

[target.'cfg(target_os = "linux")'.dependencies]
dbus = "0.9"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### 3. Create Project Structure

```bash
mkdir -p src/platform tests resources/icons
touch src/{state,config,activity,mouse,tray,error}.rs
touch src/platform/{mod,macos,windows,linux}.rs
touch tests/{integration,mouse_test}.rs
```

### 4. Create src/error.rs

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RmmError {
    #[error("Mouse control error: {0}")]
    MouseControl(String),
    #[error("Activity monitoring error: {0}")]
    ActivityMonitor(String),
    #[error("System tray error: {0}")]
    SystemTray(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Platform-specific error: {0}")]
    Platform(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, RmmError>;
```

### 5. Update src/main.rs

```rust
mod error;

use error::Result;
use tracing::info;
use tracing_subscriber;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    info!("Starting RMM");
    
    // TODO: Initialize application
    
    Ok(())
}
```

### 6. Verify

```bash
cargo build
cargo run
cargo test
cargo clippy
```

### 7. Update .gitignore

```gitignore
/target/
Cargo.lock
**/*.rs.bk
*.pdb
.DS_Store
*.log
```

---

## Phase 2: State Management

### 1. Implement src/state.rs

```rust
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
```

### 2. Implement src/config.rs

```rust
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
```

### 3. Update src/main.rs

```rust
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
```

### 4. Verify

```bash
cargo build
cargo run
```

---

## Phase 3: Activity Monitoring

### 1. Implement src/activity.rs

```rust
use crate::state::SharedState;
use rdev::{listen, Event, EventType};
use std::time::Instant;
use tracing::{error, info};

pub fn start_monitoring(state: SharedState) {
    std::thread::spawn(move || {
        info!("Starting activity monitoring");
        
        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(_) |
                EventType::MouseMove { .. } |
                EventType::ButtonPress(_) => {
                    if let Ok(mut state) = state.lock() {
                        state.last_activity = Instant::now();
                    }
                }
                _ => {}
            }
        };

        if let Err(e) = listen(callback) {
            error!("Error in activity monitoring: {:?}", e);
        }
    });
}
```

### 2. Update src/main.rs

```rust
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
```

### 3. Verify

```bash
cargo build
cargo run
```

After running, move the mouse or press keyboard keys. You should see activity being monitored (state.last_activity will update).

---

## Phase 4: Mouse Movement Controller

### 1. Implement src/mouse.rs

```rust
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
```

### 2. Update src/main.rs

```rust
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
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    info!("Starting RMM 2");
    
    let config = config::Config::load()?;
    let state = Arc::new(Mutex::new(state::AppState::new()));
    
    // Set running to true
    {
        let mut state_guard = state.lock().unwrap();
        state_guard.is_running = true;
    }
    
    info!("Configuration loaded");
    info!("State initialized");
    
    // Start activity monitoring
    activity::start_monitoring(Arc::clone(&state));
    info!("Activity monitoring started");
    
    // Heartbeat loop - check every 60 seconds
    let heartbeat_state = Arc::clone(&state);
    let inactivity_threshold = config.inactivity_threshold;
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60));
            if let Err(e) = mouse::check_and_move(Arc::clone(&heartbeat_state), inactivity_threshold) {
                tracing::error!("Error in heartbeat: {:?}", e);
            }
        }
    });
    info!("Heartbeat started (60s interval)");
    
    // Keep the application running
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
```

### 3. Verify

```bash
cargo build
cargo run
```

After running:
1. The program checks every 60 seconds
2. If there's no activity within 60 seconds, it will automatically move the mouse by ±10 pixels
3. Movement direction alternates (+10, -10, +10, -10...)
4. After moving, it verifies the position is correct
5. If it fails 10 times, it displays an error message

---

## Phase 5: System Tray Integration

### Platform-Specific Considerations

The tray-item crate has different APIs on different platforms. Here's what you need to know:

#### Platform API Differences

| Feature | macOS | Linux (ksni) | Windows |
|---------|-------|--------------|---------|
| IconSource::Data | Yes | Yes | No |
| IconSource::Resource | Yes | Yes | Yes |
| add_quit_item() | Yes | No | No |
| display() | Yes | No (auto-spawns) | No (auto-runs) |

#### Icon Source Selection

- **macOS and Linux**: Use `IconSource::Data` with PNG bytes
- **Windows**: Use `IconSource::Resource` with resource name

#### Quit Menu Handling

- **macOS**: Use special `add_quit_item()` method and call `display()`
- **Linux and Windows**: Use regular `add_menu_item()` for quit functionality

### 1. Create src/tray.rs

Add a system tray icon with menu controls that works across all platforms.

```rust
use native_dialog::{MessageDialog, MessageType};
use std::process;
use tracing::info;
use tray_item::{IconSource, TrayItem};

pub fn create_tray() -> () {
    // Platform-specific icon creation
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let icon = {
        let png_data = include_bytes!("../resources/mouse.png").to_vec();
        IconSource::Data {
            data: png_data,
            height: 16,
            width: 16,
        }
    };

    #[cfg(target_os = "windows")]
    let icon = IconSource::Resource("mouse-icon");

    // Create tray icon
    let mut tray = TrayItem::new("RMM - Rust Mouse Monitor", icon).unwrap();

    // Add About menu item with native dialog
    tray.add_menu_item("About", || {
        let _ = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("About RMM")
            .set_text("RMM - Rust Mouse Monitor\n\nAuthor: Red\n\nCreated with LLM help for learning Rust concepts")
            .show_alert();
    }).unwrap();

    tray.add_label("---").unwrap();

    // Add Stop menu item
    tray.add_menu_item("Stop", || {
        info!("Stopping RMM application...");
        println!("RMM stopped by user");
        process::exit(0);
    })
    .unwrap();

    // Platform-specific quit handling
    #[cfg(target_os = "macos")]
    {
        let inner = tray.inner_mut();
        inner.add_quit_item("Quit");
        inner.display();
    }

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        // On Linux (ksni) and Windows, add Quit as a regular menu item
        tray.add_menu_item("Quit", || {
            info!("Quitting RMM application...");
            process::exit(0);
        })
        .unwrap();
    }
}
```

### Platform-Specific Dependencies

Update Cargo.toml to include platform-specific tray-item features:

```toml
[target.'cfg(target_os = "macos")'.dependencies]
tray-item = "0.10"

[target.'cfg(target_os = "windows")'.dependencies]
tray-item = "0.10"

[target.'cfg(target_os = "linux")'.dependencies]
tray-item = { version = "0.10", features = ["ksni"] }
```

### 2. Update src/main.rs

Add tray module and integrate it.

```rust
mod error;
mod state;
mod config;
mod activity;
mod mouse;
mod tray;

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
    
    // Set running to true
    {
        let mut state_guard = state.lock().unwrap();
        state_guard.is_running = true;
    }
    
    info!("Configuration loaded");
    info!("State initialized");
    
    // Create system tray
    let (_tray, tray_rx) = tray::create_tray(Arc::clone(&state))?;
    tray::handle_tray_events(tray_rx, Arc::clone(&state));
    info!("System tray initialized");
    
    // Start activity monitoring
    activity::start_monitoring(Arc::clone(&state));
    info!("Activity monitoring started");
    
    // Heartbeat loop
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
    
    // Keep running
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
```

### 3. Test

```bash
cargo build
cargo run
```

After running:
1. You'll see a system tray icon
2. Click it to see the menu
3. Use "Start/Stop" to toggle mouse movement
4. Use "Quit" to exit the app

**Note**: On macOS, you may need to grant accessibility permissions for the tray to work properly.

---

## Completion Checklist

### Phase 1
- [ ] `cargo build` succeeds
- [ ] `cargo run` executes without errors
- [ ] All module files created
- [ ] Directory structure correct

### Phase 2
- [ ] state.rs implementation complete
- [ ] config.rs implementation complete
- [ ] main.rs updated
- [ ] Program compiles and runs

### Phase 3
- [ ] activity.rs implementation complete
- [ ] main.rs integrates activity monitoring
- [ ] Program can monitor keyboard and mouse activity
- [ ] Background thread runs normally

### Phase 4
- [ ] mouse.rs implementation complete
- [ ] MouseController struct created
- [ ] check_and_move function implemented
- [ ] main.rs integrates heartbeat loop
- [ ] Program can automatically move mouse
- [ ] Movement verification works
- [ ] Error counting works

### Phase 5
- [ ] tray.rs implementation complete
- [ ] TrayMessage enum created
- [ ] create_tray function implemented
- [ ] handle_tray_events function implemented
- [ ] main.rs integrates system tray
- [ ] System tray icon displays
- [ ] Start/Stop functionality works
- [ ] Quit functionality works

---

## Common Commands

```bash
cargo check          # Quick check
cargo build          # Build debug
cargo build --release # Build release
cargo run            # Run
cargo test           # Test
cargo clippy         # Lint
cargo fmt            # Format
```
