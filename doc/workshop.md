# Workshop: Rust Mouse Mover 初始化步驟

## Phase 1: 專案初始化

### 1. 初始化 Cargo 專案

```bash
cargo init --name rmm
```

### 2. 設定 Cargo.toml

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

### 3. 建立專案結構

```bash
mkdir -p src/platform tests resources/icons
touch src/{state,config,activity,mouse,tray,error}.rs
touch src/platform/{mod,macos,windows,linux}.rs
touch tests/{integration,mouse_test}.rs
```

### 4. 建立 src/error.rs

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

### 5. 更新 src/main.rs

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

### 6. 驗證

```bash
cargo build
cargo run
cargo test
cargo clippy
```

### 7. 更新 .gitignore

```gitignore
/target/
Cargo.lock
**/*.rs.bk
*.pdb
.DS_Store
*.log
```

---

## Phase 2: 狀態管理

### 1. 實作 src/state.rs

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

### 2. 實作 src/config.rs

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

### 3. 更新 src/main.rs

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

### 4. 驗證

```bash
cargo build
cargo run
```

---

## Phase 3: 活動監控

### 1. 實作 src/activity.rs

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

### 2. 更新 src/main.rs

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

### 3. 驗證

```bash
cargo build
cargo run
```

執行後，移動滑鼠或按鍵盤，應該會看到活動被監控（state.last_activity 會更新）。

---

## Phase 4: 滑鼠移動控制器

### 1. 實作 src/mouse.rs

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

### 2. 更新 src/main.rs

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

### 3. 驗證

```bash
cargo build
cargo run
```

執行後：
1. 程式會每 60 秒檢查一次
2. 如果 60 秒內沒有活動，會自動移動滑鼠 ±10 像素
3. 移動方向會交替（+10, -10, +10, -10...）
4. 移動後會驗證位置是否正確
5. 如果失敗 10 次會顯示錯誤訊息

---

## Phase 5: System Tray Integration

### 1. Create src/tray.rs

Add a system tray icon with menu controls.

```rust
use crate::error::{Result, RmmError};
use crate::state::SharedState;
use tray_item::{IconSource, TrayItem};
use std::sync::mpsc;
use tracing::info;

pub enum TrayMessage {
    Toggle,
    Quit,
}

pub fn create_tray(state: SharedState) -> Result<(TrayItem, mpsc::Receiver<TrayMessage>)> {
    let mut tray = TrayItem::new("RMM", IconSource::Resource("icon-name"))
        .map_err(|e| RmmError::SystemTray(format!("Failed to create tray: {:?}", e)))?;
    
    let (tx, rx) = mpsc::channel();
    
    // Start/Stop menu item
    let tx_toggle = tx.clone();
    tray.add_menu_item("Start/Stop", move || {
        let _ = tx_toggle.send(TrayMessage::Toggle);
    })
    .map_err(|e| RmmError::SystemTray(format!("Failed to add menu item: {:?}", e)))?;
    
    // Quit menu item
    let tx_quit = tx.clone();
    tray.add_menu_item("Quit", move || {
        let _ = tx_quit.send(TrayMessage::Quit);
    })
    .map_err(|e| RmmError::SystemTray(format!("Failed to add menu item: {:?}", e)))?;
    
    info!("System tray created");
    Ok((tray, rx))
}

pub fn handle_tray_events(rx: mpsc::Receiver<TrayMessage>, state: SharedState) {
    std::thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                TrayMessage::Toggle => {
                    if let Ok(mut state_guard) = state.lock() {
                        state_guard.is_running = !state_guard.is_running;
                        let status = if state_guard.is_running { "started" } else { "stopped" };
                        info!("Mouse movement {}", status);
                    }
                }
                TrayMessage::Quit => {
                    info!("Quit requested");
                    std::process::exit(0);
                }
            }
        }
    });
}
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

## 完成檢查

### Phase 1
- [ ] `cargo build` 成功
- [ ] `cargo run` 執行無誤
- [ ] 所有模組檔案已建立
- [ ] 目錄結構正確

### Phase 2
- [ ] state.rs 實作完成
- [ ] config.rs 實作完成
- [ ] main.rs 更新完成
- [ ] 程式可以編譯執行

### Phase 3
- [ ] activity.rs 實作完成
- [ ] main.rs 整合活動監控
- [ ] 程式可以監控鍵盤滑鼠活動
- [ ] 背景執行緒正常運作

### Phase 4
- [ ] mouse.rs 實作完成
- [ ] MouseController 結構體建立
- [ ] check_and_move 函數實作
- [ ] main.rs 整合 heartbeat 迴圈
- [ ] 程式可以自動移動滑鼠
- [ ] 移動驗證功能正常
- [ ] 錯誤計數功能正常

### Phase 5
- [ ] tray.rs 實作完成
- [ ] TrayMessage enum 建立
- [ ] create_tray 函數實作
- [ ] handle_tray_events 函數實作
- [ ] main.rs 整合 system tray
- [ ] 系統托盤圖示顯示
- [ ] Start/Stop 功能正常
- [ ] Quit 功能正常

---

## 常用指令

```bash
cargo check          # 快速檢查
cargo build          # 建置 debug
cargo build --release # 建置 release
cargo run            # 執行
cargo test           # 測試
cargo clippy         # Lint
cargo fmt            # 格式化
```
