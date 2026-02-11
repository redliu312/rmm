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
