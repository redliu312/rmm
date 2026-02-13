# RMM - Rust Mouse Monitor

[![CI](https://github.com/redliu312/rmm/workflows/CI/badge.svg)](https://github.com/redliu312/rmm/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)


Mainly focus on the macos app, 
if the mouse auto move fail (log file located at /Users/simon_liu/Library/Application Support/com.rmm.rmm)
grant the accessibility permission for this app in macos system setting




A learning project for Rust, created with LLM assistance.

RMM is an automatic mouse mover that prevents your computer from going to sleep by simulating mouse activity when you're inactive. It runs as a system tray application on macOS, Windows, and Linux.

## Inspiration

This project is inspired by [automatic-mouse-mover](https://github.com/prashantgupta24/automatic-mouse-mover), which is a Go implementation. RMM is a Rust reimplementation created as a learning exercise.

## Features

- Automatic Mouse Movement - Moves mouse when inactive
- System Tray Icon - Runs quietly in the background
- Configurable - Customize inactivity threshold and check intervals
- Activity Monitoring - Tracks keyboard and mouse activity
- Cross-platform - Works on macOS, Windows, and Linux

## Installation

### Prerequisites

- Rust 1.70 or later
- Platform-specific dependencies:
  - Linux: `libx11-dev`, `libxtst-dev`, `libevdev-dev`, `libdbus-1-dev`
  - macOS: Xcode Command Line Tools
  - Windows: No additional dependencies

### Download Pre-built Binaries

Download the latest release from the [Releases page](https://github.com/redliu312/rmm/releases).

#### macOS (.app Bundle - Recommended)

For macOS users, download the `.dmg` file for your architecture:
- **Intel Macs**: `RMM-x86_64-apple-darwin.dmg`
- **Apple Silicon (M1/M2/M3)**: `RMM-aarch64-apple-darwin.dmg`

Installation:
1. Download and open the `.dmg` file
2. Drag `RMM.app` to your Applications folder
3. Double-click `RMM.app` to run
4. The app will appear in your menu bar

The .app bundle provides a native macOS experience with proper icon and menu bar integration.

#### Other Platforms

- **Linux (x86_64)**: `rmm-linux-x86_64.tar.gz`
- **Windows (x86_64)**: `rmm-windows-x86_64.exe.zip`
- **macOS (command-line binary)**: `rmm-macos-x86_64.tar.gz` or `rmm-macos-aarch64.tar.gz`

### Build from Source

#### Standard Build

```bash
# Clone the repository
git clone https://github.com/redliu312/rmm.git
cd rmm

# Build the project
cargo build --release

# The binary will be in target/release/rmm (or rmm.exe on Windows)
```

#### Build macOS .app Bundle (macOS only)

```bash
# Build the .app bundle
make app

# The app will be in target/release/RMM.app
# You can double-click it or drag it to /Applications

# Optional: Create a DMG installer
make dmg

# Optional: Install directly to /Applications
make install
```

## Usage

### macOS (.app Bundle)

Simply double-click `RMM.app` from your Applications folder or Launchpad. The application will:
1. Start monitoring your keyboard and mouse activity
2. Display a menu bar icon (top-right of your screen)
3. Automatically move the mouse when you've been inactive for the configured threshold

### Command-line (All Platforms)

Run the application from the terminal:

```bash
# If built from source
cargo run --release

# Or run the binary directly
./rmm  # macOS/Linux
rmm.exe  # Windows
```

The application will:
1. Start monitoring your keyboard and mouse activity
2. Display a system tray icon
3. Automatically move the mouse when you've been inactive for the configured threshold

### System Tray Menu

- About - Shows application information
- Stop - Stops the application
- Quit - Exits the application

## Configuration

Configuration is stored in:
- macOS: `~/Library/Application Support/rmm/config.json`
- Linux: `~/.config/rmm/config.json`
- Windows: `%APPDATA%\rmm\config.json`

Default configuration:

```json
{
  "inactivity_threshold": 10,
  "heartbeat_interval": 10,
  "worker_interval": 10,
  "movement_delta": 10,
  "max_errors": 10,
  "auto_start": false
}
```

Configuration options:
- `inactivity_threshold`: Seconds of inactivity before moving mouse (default: 10)
- `heartbeat_interval`: Seconds between activity checks (default: 10)
- `worker_interval`: Seconds between worker thread checks (default: 10)
- `movement_delta`: Pixels to move the mouse (default: 10)
- `max_errors`: Maximum errors before stopping (default: 10)
- `auto_start`: Start monitoring automatically on launch (default: false)

### macOS Permissions

**Important:** On macOS, RMM requires **Accessibility permission** to monitor keyboard/mouse activity and move the cursor.

1. Launch RMM.app
2. Go to **System Settings → Privacy & Security → Accessibility**
3. Add RMM.app and toggle it ON
4. Restart RMM.app

Without this permission, the app will run but won't detect inactivity or move the mouse.

See [`macos/PERMISSIONS.md`](macos/PERMISSIONS.md) for detailed setup instructions.

## Troubleshooting

### macOS: App Runs But Mouse Doesn't Move

**Symptom:** Menu bar icon appears, but mouse doesn't move after inactivity.

**Solution:** Grant Accessibility permission (see Configuration → macOS Permissions above).

**Verify the app is running:**
```bash
# Check if RMM process is running
ps aux | grep -i rmm | grep -v grep

# View recent app logs (macOS)
log show --predicate 'process == "rmm"' --last 1m --style compact 2>/dev/null | tail -20
```

**Command explanation:**
- `log show` - macOS unified logging system
- `--predicate 'process == "rmm"'` - Filter logs for RMM process only
- `--last 1m` - Show logs from the last 1 minute
- `--style compact` - Compact output format
- `2>/dev/null` - Suppress error messages
- `| tail -20` - Show only the last 20 lines

**Check for permission errors:**
```bash
# Look for permission-related errors
log show --predicate 'process == "rmm"' --last 5m --style compact 2>/dev/null | grep -i "permission\|denied\|accessibility"
```

### macOS: "RMM.app is damaged"

**Solution:**
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /path/to/RMM.app

# Or right-click → Open (first time only)
```

### Configuration Not Working

**Check config file location:**
```bash
# macOS
cat ~/Library/Application\ Support/rmm/config.json

# Linux
cat ~/.config/rmm/config.json

# Windows
type %APPDATA%\rmm\config.json
```

**Create a test config with 5-second threshold:**
```bash
# macOS
mkdir -p ~/Library/Application\ Support/rmm
cat > ~/Library/Application\ Support/rmm/config.json << 'EOF'
{
  "inactivity_threshold": 5,
  "heartbeat_interval": 1,
  "worker_interval": 1,
  "movement_delta": 10,
  "max_errors": 10,
  "auto_start": false
}
EOF

# Linux
mkdir -p ~/.config/rmm
cat > ~/.config/rmm/config.json << 'EOF'
{
  "inactivity_threshold": 5,
  "heartbeat_interval": 1,
  "worker_interval": 1,
  "movement_delta": 10,
  "max_errors": 10,
  "auto_start": false
}
EOF
```

Then restart the app - it should move the mouse after 5 seconds of inactivity.

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test mouse_test
```

## Development

### Project Structure

```
rmm/
├── src/
│   ├── main.rs          # Application entry point
│   ├── tray.rs          # System tray implementation
│   ├── mouse.rs         # Mouse movement logic
│   ├── activity.rs      # Activity monitoring
│   ├── state.rs         # Application state
│   ├── config.rs        # Configuration management
│   └── error.rs         # Error types
├── macos/               # macOS .app bundle files
│   ├── Info.plist       # Bundle metadata
│   └── build-app.sh     # Build script for .app
├── resources/           # Application resources
│   └── mouse.png        # Tray icon
├── tests/              # Integration tests
├── Makefile            # Build automation
└── doc/                # Documentation
```

### Building macOS .app Bundle

The project includes scripts to build a native macOS .app bundle:

```bash
# Build the .app bundle (creates target/release/RMM.app)
make app

# Create a DMG installer (creates target/release/RMM.dmg)
make dmg

# Install to /Applications
make install
```

The build process:
1. Compiles the release binary
2. Creates the `.app` directory structure
3. Converts the PNG icon to `.icns` format
4. Packages everything into a proper macOS application bundle
5. Optionally creates a DMG installer

The resulting `.app` bundle can be double-clicked like any native macOS application.

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for issues
cargo check
```

### Pre-commit Hooks

Install Git hooks to automatically format code before commits:

```bash
# Install the pre-commit hook
bash .githooks/install.sh
```

The pre-commit hook will:
- Automatically run `cargo fmt` before each commit
- Prevent commits if code is not properly formatted
- Ensure consistent code style across the project

To bypass the hook (not recommended):
```bash
git commit --no-verify
```

### Creating a Release

To create a new release with pre-built binaries:

1. Update the version in `Cargo.toml`
2. Update `CHANGELOG.md` with the new version and changes
3. Commit the changes
4. Create and push a git tag:

```bash
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

5. GitHub Actions will automatically:
   - Build binaries for Linux, macOS (Intel & Apple Silicon), and Windows
   - Create a GitHub release
   - Upload the binaries as release assets

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [automatic-mouse-mover](https://github.com/prashantgupta24/automatic-mouse-mover)
- Created with LLM assistance for learning Rust concepts
- Author: redliu312

## Contributing

This is primarily a learning project, but contributions are welcome! Feel free to:

- Report bugs
- Suggest features
- Submit pull requests

## Learning Resources

This project demonstrates:
- Rust async programming with Tokio
- Cross-platform system tray applications
- Event-driven architecture
- Error handling with `thiserror` and `anyhow`
- Configuration management
- Platform-specific code with conditional compilation
