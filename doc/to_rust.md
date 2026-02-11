# Porting to Rust - Crate Equivalents

## Core Dependencies Mapping

### Mouse/Keyboard Control
**Go**: `robotgo` (github.com/go-vgo/robotgo)
- CGo bindings to native APIs
- Cross-platform mouse/keyboard control

**Rust**: `enigo` or `rdev`
- `enigo` (v0.2+): Simulates mouse/keyboard input
  ```rust
  use enigo::{Enigo, Mouse, Settings};
  let mut enigo = Enigo::new(&Settings::default()).unwrap();
  enigo.move_mouse(x, y, enigo::Coordinate::Abs).unwrap();
  ```
- `rdev`: Lower-level event simulation
  ```rust
  use rdev::{simulate, EventType, Button};
  simulate(&EventType::MouseMove { x: 100.0, y: 100.0 });
  ```

### System Tray
**Go**: `systray` (github.com/getlantern/systray)
- Native system tray integration
- Platform-specific implementations

**Rust**: `tray-item` or `ksni`
- `tray-item`: Simple cross-platform tray
  ```rust
  use tray_item::TrayItem;
  let mut tray = TrayItem::new("AMM", "icon").unwrap();
  tray.add_menu_item("Start", || {});
  ```
- `ksni`: Linux StatusNotifier, more feature-rich
- `tao` + `muda`: Modern alternative (part of Tauri ecosystem)

### Activity Monitoring
**Go**: `activity-tracker` (custom library)
- Monitors keyboard/mouse events
- Platform-specific hooks

**Rust**: `rdev` for event listening
```rust
use rdev::{listen, Event};

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(_) | EventType::MouseMove { .. } => {
            // Activity detected
        }
        _ => {}
    }
}

listen(callback).unwrap();
```

### macOS Sleep Notifications
**Go**: `mac-sleep-notifier` (github.com/prashantgupta24/mac-sleep-notifier)
- IOKit notifications for sleep/wake

**Rust**: `core-foundation` + `io-kit-sys`
```rust
use core_foundation::runloop::*;
use io_kit_sys::*;

// Register for IORegisterForSystemPower notifications
// Handle kIOMessageSystemWillSleep / kIOMessageSystemHasPoweredOn
```

### Logging
**Go**: `logrus` (github.com/sirupsen/logrus)

**Rust**: `tracing` or `env_logger`
```rust
use tracing::{info, error};
use tracing_subscriber;

tracing_subscriber::fmt::init();
info!("Mouse moved at {:?}", timestamp);
```

### Configuration Storage
**Go**: `configdir` (github.com/kirsle/configdir)
- Cross-platform config directory

**Rust**: `directories` crate
```rust
use directories::ProjectDirs;

let proj_dirs = ProjectDirs::from("com", "amm", "AutoMouseMover").unwrap();
let config_dir = proj_dirs.config_dir();
```

## Implementation Strategy

### 1. Core Loop
Replace Go channels with Rust async:
```rust
use tokio::time::{interval, Duration};

let mut heartbeat = interval(Duration::from_secs(60));
loop {
    tokio::select! {
        _ = heartbeat.tick() => {
            // Check activity and move mouse
        }
        _ = quit_rx.recv() => break,
    }
}
```

### 2. State Management
Replace Go mutexes with Rust:
```rust
use std::sync::{Arc, Mutex};

struct State {
    is_running: bool,
    last_moved: std::time::Instant,
}

let state = Arc::new(Mutex::new(State::default()));
```

### 3. Platform-Specific Code
Use conditional compilation:
```rust
#[cfg(target_os = "macos")]
fn setup_sleep_notifications() { /* IOKit */ }

#[cfg(target_os = "windows")]
fn setup_sleep_notifications() { /* Win32 API */ }

#[cfg(target_os = "linux")]
fn setup_sleep_notifications() { /* D-Bus */ }
```

## Recommended Crates

| Functionality | Crate | Version |
|--------------|-------|---------|
| Mouse control | `enigo` | 0.2+ |
| Event listening | `rdev` | 0.5+ |
| System tray | `tray-item` | 0.10+ |
| Async runtime | `tokio` | 1.0+ |
| Logging | `tracing` | 0.1+ |
| Config dirs | `directories` | 5.0+ |
| Serialization | `serde` + `serde_json` | 1.0+ |

## Key Differences

### Memory Safety
- Rust eliminates data races at compile time
- No need for explicit mutex locks in many cases (use channels)

### Error Handling
- Replace Go's `panic` with `Result<T, E>`
- Use `?` operator for propagation

### Concurrency
- Replace goroutines with `tokio::spawn`
- Replace channels with `tokio::sync::mpsc`

### Build System
- Replace `go.mod` with `Cargo.toml`
- Native cross-compilation support
