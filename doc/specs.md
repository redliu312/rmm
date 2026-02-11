# Automatic Mouse Mover - Technical Specifications

## Architecture

System tray application that prevents laptop sleep by simulating mouse movement during inactivity.

## Cross-Platform OS Control

### Activity Tracking
- Uses [`activity-tracker`](https://github.com/prashantgupta24/activity-tracker) library
- Monitors keyboard/mouse events via OS-specific APIs:
  - **macOS**: `mac-sleep-notifier` for system sleep/wake events
  - **Windows**: Win32 API hooks
  - **Linux**: X11/XGB event monitoring

### System Tray Integration
- [`systray`](https://github.com/getlantern/systray) library provides native system tray across platforms
- Platform-specific implementations:
  - **macOS**: NSStatusBar
  - **Windows**: Shell_NotifyIcon
  - **Linux**: libappindicator/GTK

## Mouse Movement Mechanism

### Implementation
Uses [`robotgo`](https://github.com/go-vgo/robotgo) library for cross-platform mouse control:

```go
currentX, currentY := robotgo.GetMousePos()
robotgo.Move(currentX + 10, currentY + 10)
```

### Movement Logic
1. **Heartbeat interval**: 60 seconds
2. **Worker check**: Every 10 seconds
3. **Movement pattern**: ±10 pixels (alternates direction)
4. **Trigger**: Only when no user activity detected

### Verification
After movement, app verifies mouse position changed:
- Success: Updates timestamp, reverses direction
- Failure: Increments error counter, shows alert after 10 failures

## Sleep Prevention

### How It Works

Operating systems use idle timers to trigger sleep:
- **Input monitoring**: OS tracks last keyboard/mouse event
- **Idle threshold**: Typically 5-30 minutes without input
- **Sleep trigger**: When threshold exceeded

### Prevention Mechanism

1. **Activity detection**: App monitors user input every 10s
2. **Idle detection**: When no activity for 60s
3. **Simulated input**: Moves mouse ±10 pixels
4. **OS response**: Resets idle timer, preventing sleep

### System Sleep Handling

App detects actual system sleep events:
- Pauses mouse movement during sleep
- Resumes after wake
- Prevents unnecessary operations while suspended

## Key Dependencies

- `robotgo`: Low-level mouse/keyboard control
- `activity-tracker`: Cross-platform activity monitoring
- `systray`: Native system tray integration
- `mac-sleep-notifier`: macOS sleep/wake notifications
