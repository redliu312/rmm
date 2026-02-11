# Automatic Mouse Mover - Rust Implementation Plan

## Project Overview

Port the Automatic Mouse Mover from Go to Rust, creating a cross-platform system tray application that prevents laptop sleep by simulating mouse movement during user inactivity.

## Architecture

### Core Components

1. **System Tray Interface**
   - Native system tray integration across macOS, Windows, and Linux
   - Menu items: Start/Stop, Settings, Quit
   - Status indicator (running/paused)

2. **Activity Monitor**
   - Track keyboard and mouse events
   - Detect user inactivity periods
   - Platform-specific event hooks

3. **Mouse Controller**
   - Simulate mouse movements
   - Verify movement success
   - Implement alternating movement pattern (±10 pixels)

4. **Sleep Event Handler**
   - Detect system sleep/wake events
   - Pause operations during sleep
   - Resume after wake

5. **Configuration Manager**
   - Store user preferences
   - Manage heartbeat intervals
   - Track application state

## Implementation Phases

### Phase 1: Project Setup & Core Infrastructure

**Tasks:**
Dependencies (Cargo.toml):

```
enigo = "0.6" – Mouse/keyboard control (current 0.6.1).

rdev = "0.5.3" – Event listening.
​

tray-item = "0.10" – System tray.

tokio = { version = "1", features = ["full"] } – Async runtime (current ~1.49.0).
​

tracing = "0.1.44" – Logging (latest 0.1.x).
​

tracing-subscriber = "0.3" – Log formatting (latest 0.3.x, MSRV ≥ 1.65).
​

directories = "6" – Config dirs (current 6.0.0).

serde = { version = "1", features = ["derive"] }

serde_json = "1"

Rust version / edition:

MSRV: Rust 1.75+ to be safe with enigo and newer Tokio + tracing family.

Edition: edition = "2021" (or "2024" later if you want, but 2021 is safer with tooling today).
```


**Deliverables:**
- `Cargo.toml` with all dependencies
- Basic project structure
- Logging initialized

### Phase 2: State Management

**Tasks:**
- Define application state structure:
  ```rust
  struct AppState {
      is_running: bool,
      last_activity: Instant,
      last_moved: Instant,
      move_direction: i32,
      error_count: u32,
  }
  ```
- Implement thread-safe state sharing with `Arc<Mutex<AppState>>`
- Create state update methods
- Add configuration persistence

**Deliverables:**
- `src/state.rs` - State management module
- `src/config.rs` - Configuration handling

### Phase 3: Activity Monitoring

**Tasks:**
- Implement activity tracker using `rdev`:
  ```rust
  fn callback(event: Event) {
      match event.event_type {
          EventType::KeyPress(_) | 
          EventType::MouseMove { .. } | 
          EventType::ButtonPress(_) => {
              update_last_activity();
          }
          _ => {}
      }
  }
  ```
- Set up event listener in separate thread
- Update state on activity detection
- Implement 10-second worker check interval

**Deliverables:**
- `src/activity.rs` - Activity monitoring module
- Background thread for event listening

### Phase 4: Mouse Movement Controller

**Tasks:**
- Implement mouse position retrieval
- Create movement logic with `enigo`:
  ```rust
  fn move_mouse(direction: i32) -> Result<(), Error> {
      let (current_x, current_y) = get_mouse_pos()?;
      let new_x = current_x + (10 * direction);
      let new_y = current_y + (10 * direction);
      enigo.move_mouse(new_x, new_y, Coordinate::Abs)?;
      verify_movement(new_x, new_y)
  }
  ```
- Add movement verification
- Implement alternating direction (±10 pixels)
- Add error counting and alerting (10 failures threshold)
- Set up 60-second heartbeat interval

**Deliverables:**
- `src/mouse.rs` - Mouse control module
- Movement verification logic
- Error handling and alerts

### Phase 5: System Tray Integration

**Tasks:**
- Initialize system tray with `tray-item`
- Create menu structure:
  - Start/Stop toggle
  - Settings (future)
  - Quit
- Implement menu callbacks
- Add status icon updates
- Handle tray events

**Deliverables:**
- `src/tray.rs` - System tray module
- Menu item handlers
- Icon resources

### Phase 6: Platform-Specific Sleep Handling

**Tasks:**
- **macOS**: Implement IOKit sleep notifications
  ```rust
  #[cfg(target_os = "macos")]
  fn setup_sleep_notifications() {
      // Use core-foundation + io-kit-sys
      // Register for kIOMessageSystemWillSleep
      // Handle kIOMessageSystemHasPoweredOn
  }
  ```
- **Windows**: Implement Win32 API power notifications
  ```rust
  #[cfg(target_os = "windows")]
  fn setup_sleep_notifications() {
      // Use windows-rs
      // Register for WM_POWERBROADCAST
  }
  ```
- **Linux**: Implement D-Bus sleep notifications
  ```rust
  #[cfg(target_os = "linux")]
  fn setup_sleep_notifications() {
      // Use dbus crate
      // Listen to org.freedesktop.login1
  }
  ```
- Pause mouse movement during sleep
- Resume after wake

**Deliverables:**
- `src/platform/macos.rs` - macOS-specific code
- `src/platform/windows.rs` - Windows-specific code
- `src/platform/linux.rs` - Linux-specific code
- `src/platform/mod.rs` - Platform abstraction

### Phase 7: Main Event Loop

**Tasks:**
- Implement async main loop with `tokio`:
  ```rust
  #[tokio::main]
  async fn main() {
      let mut heartbeat = interval(Duration::from_secs(60));
      let mut worker = interval(Duration::from_secs(10));
      
      loop {
          tokio::select! {
              _ = heartbeat.tick() => check_and_move(),
              _ = worker.tick() => check_activity(),
              _ = quit_rx.recv() => break,
          }
      }
  }
  ```
- Integrate all components
- Handle shutdown gracefully
- Add signal handling (Ctrl+C)

**Deliverables:**
- `src/main.rs` - Main application entry point
- Event loop with all timers
- Graceful shutdown

### Phase 8: Testing & Refinement

**Tasks:**
- Unit tests for core logic
- Integration tests for mouse movement
- Test on all platforms (macOS, Windows, Linux)
- Verify sleep prevention works
- Test error handling and recovery
- Performance profiling
- Memory leak detection

**Deliverables:**
- `tests/` directory with test suites
- Platform-specific test results
- Performance benchmarks

### Phase 9: Build & Distribution

**Tasks:**
- Configure release builds with optimizations
- Set up cross-compilation for all platforms
- Create platform-specific installers/packages:
  - macOS: `.app` bundle or `.dmg`
  - Windows: `.exe` installer
  - Linux: `.deb`, `.rpm`, or AppImage
- Add application icons
- Create README and documentation

**Deliverables:**
- Release binaries for all platforms
- Installation instructions
- User documentation

## Project Structure

```
rmm/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── src/
│   ├── main.rs           # Entry point, event loop
│   ├── state.rs          # Application state management
│   ├── config.rs         # Configuration handling
│   ├── activity.rs       # Activity monitoring
│   ├── mouse.rs          # Mouse control
│   ├── tray.rs           # System tray integration
│   └── platform/
│       ├── mod.rs        # Platform abstraction
│       ├── macos.rs      # macOS sleep notifications
│       ├── windows.rs    # Windows sleep notifications
│       └── linux.rs      # Linux sleep notifications
├── tests/
│   ├── integration.rs    # Integration tests
│   └── mouse_test.rs     # Mouse movement tests
├── resources/
│   └── icons/            # Application icons
└── doc/
    ├── specs.md
    ├── to_rust.md
    └── plan.md
```

## Key Technical Decisions

### Async Runtime
- **Choice**: Tokio
- **Rationale**: Industry standard, excellent performance, comprehensive ecosystem

### Mouse Control
- **Choice**: `enigo` (primary), `rdev` (fallback)
- **Rationale**: `enigo` v0.2+ has better cross-platform support and simpler API

### System Tray
- **Choice**: `tray-item`
- **Rationale**: Simple, cross-platform, minimal dependencies
- **Alternative**: Consider `tao` + `muda` for more features if needed

### State Management
- **Choice**: `Arc<Mutex<T>>` for shared state
- **Rationale**: Simple, safe, sufficient for this use case
- **Alternative**: Could use channels for message-passing if complexity grows

### Error Handling
- **Strategy**: Use `Result<T, E>` throughout, custom error types with `thiserror`
- **Logging**: `tracing` for structured logging with different levels

## Configuration Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `heartbeat_interval` | 60s | Time between activity checks |
| `worker_interval` | 10s | Activity monitoring frequency |
| `inactivity_threshold` | 60s | Idle time before mouse movement |
| `movement_delta` | 10 pixels | Distance to move mouse |
| `max_errors` | 10 | Failures before alert |
| `auto_start` | false | Start on application launch |

## Risk Mitigation

### Platform Compatibility
- **Risk**: Different behavior across platforms
- **Mitigation**: Extensive testing on all platforms, platform-specific code isolation

### Permission Issues
- **Risk**: Accessibility permissions required on macOS
- **Mitigation**: Clear documentation, permission request handling

### Mouse Control Failures
- **Risk**: Mouse movement might fail in certain scenarios
- **Mitigation**: Verification after movement, error counting, user alerts

### Resource Usage
- **Risk**: Background process consuming too much CPU/memory
- **Mitigation**: Efficient event handling, proper sleep intervals, profiling

## Success Criteria

1. ✅ Application runs on macOS, Windows, and Linux
2. ✅ Successfully prevents system sleep during inactivity
3. ✅ Mouse movement is imperceptible to user (±10 pixels)
4. ✅ Properly detects and handles system sleep/wake
5. ✅ System tray integration works natively on all platforms
6. ✅ CPU usage < 1% when idle
7. ✅ Memory usage < 50MB
8. ✅ No crashes or panics during normal operation
9. ✅ Graceful error handling and recovery
10. ✅ User-friendly installation and setup

## Timeline Estimate

- **Phase 1**: 1 day - Project setup
- **Phase 2**: 1 day - State management
- **Phase 3**: 2 days - Activity monitoring
- **Phase 4**: 2 days - Mouse controller
- **Phase 5**: 2 days - System tray
- **Phase 6**: 3 days - Platform-specific sleep handling
- **Phase 7**: 1 day - Main event loop
- **Phase 8**: 3 days - Testing
- **Phase 9**: 2 days - Build & distribution

**Total**: ~17 days (3.5 weeks)

## Next Steps

1. Create initial Rust project structure
2. Set up `Cargo.toml` with all dependencies
3. Implement basic logging and state management
4. Begin Phase 3: Activity monitoring implementation
