# RMM - Rust Mouse Monitor

[![CI](https://github.com/redliu312/rmm/workflows/CI/badge.svg)](https://github.com/redliu312/rmm/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)

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

Available platforms:
- Linux (x86_64)
- macOS (Intel and Apple Silicon)
- Windows (x86_64)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/redliu312/rmm.git
cd rmm

# Build the project
cargo build --release

# The binary will be in target/release/rmm (or rmm.exe on Windows)
```

## Usage

Simply run the application:

```bash
cargo run --release
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
  "inactivity_threshold": 300,
  "heartbeat_interval": 60
}
```

- `inactivity_threshold`: Seconds of inactivity before moving mouse (default: 300)
- `heartbeat_interval`: Seconds between activity checks (default: 60)

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
├── resources/           # Application resources
│   └── mouse.png        # Tray icon
├── tests/              # Integration tests
└── doc/                # Documentation
```

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
