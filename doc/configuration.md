# RMM Configuration Guide

## Configuration File Source

RMM's configuration file comes from the following logic (see [`src/config.rs`](../src/config.rs)):

### 1. Configuration File Path

The application uses the `directories` crate to determine the configuration file location:

```rust
fn config_path() -> Result<PathBuf> {
    ProjectDirs::from("com", "rmm", "rmm")
        .map(|dirs| dirs.config_dir().join("config.json"))
        .ok_or_else(|| crate::error::RmmError::Config("Cannot find config directory".into()))
}
```

**Actual Paths:**

| Operating System | Configuration File Path |
|-----------------|------------------------|
| macOS | `~/Library/Application Support/rmm/config.json` |
| Linux | `~/.config/rmm/config.json` |
| Windows | `%APPDATA%\rmm\config.json` |

### 2. Loading Logic

When the application starts, it executes `Config::load()`:

```rust
pub fn load() -> Result<Self> {
    let path = Self::config_path()?;
    if path.exists() {
        // If config file exists, read and parse JSON
        let content = fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        // If config file doesn't exist, use default values
        Ok(Self::default())
    }
}
```

**Flow Diagram:**

```
Start RMM
    ↓
Does config file exist?
    ↓
  YES ────→ Read config.json ────→ Parse JSON ────→ Use custom config
    ↓
   NO ────→ Use default config (Default::default())
```

### 3. Default Configuration

If the configuration file doesn't exist, the program uses hardcoded default values:

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            heartbeat_interval: 10,      // Heartbeat check interval (seconds)
            worker_interval: 10,         // Worker thread interval (seconds)
            inactivity_threshold: 10,    // Inactivity time threshold (seconds)
            movement_delta: 10,          // Mouse movement distance (pixels)
            max_errors: 10,              // Maximum error count
            auto_start: false,           // Auto-start monitoring
        }
    }
}
```

## Configuration File Lifecycle

### First Launch

```
1. User launches RMM.app
2. Program checks ~/Library/Application Support/rmm/config.json
3. File doesn't exist
4. Uses default configuration (all values are 10)
5. Application starts running
```

**At this point, the config file has not been created** - the program only uses default values in memory.

### Creating Configuration File

The configuration file must be **manually created**. The program does not automatically generate it. Two methods:

#### Method 1: Manual Creation

```bash
# macOS
mkdir -p ~/Library/Application\ Support/rmm
cat > ~/Library/Application\ Support/rmm/config.json << 'EOF'
{
  "inactivity_threshold": 300,
  "heartbeat_interval": 60,
  "worker_interval": 60,
  "movement_delta": 10,
  "max_errors": 10,
  "auto_start": false
}
EOF

# Linux
mkdir -p ~/.config/rmm
cat > ~/.config/rmm/config.json << 'EOF'
{
  "inactivity_threshold": 300,
  "heartbeat_interval": 60,
  "worker_interval": 60,
  "movement_delta": 10,
  "max_errors": 10,
  "auto_start": false
}
EOF

# Windows (PowerShell)
New-Item -ItemType Directory -Force -Path "$env:APPDATA\rmm"
@"
{
  "inactivity_threshold": 300,
  "heartbeat_interval": 60,
  "worker_interval": 60,
  "movement_delta": 10,
  "max_errors": 10,
  "auto_start": false
}
"@ | Out-File -FilePath "$env:APPDATA\rmm\config.json" -Encoding UTF8
```

#### Method 2: Using Code (Not Implemented)

The code has a `save()` method, but it's marked as `#[allow(dead_code)]`, meaning it's currently unused:

```rust
pub fn save(&self) -> Result<()> {
    let path = Self::config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;  // Auto-create directory
    }
    let content = serde_json::to_string_pretty(self)?;
    fs::write(&path, content)?;
    Ok(())
}
```

**Note:** The current RMM version has no UI or command to invoke `save()`, so the config file must be created manually.

### Modifying Configuration

```
1. Edit ~/Library/Application Support/rmm/config.json
2. Save the file
3. Restart RMM.app
4. Program reads the new configuration
```

## Configuration Parameters

### inactivity_threshold

- **Unit:** Seconds
- **Default:** 10 seconds
- **Purpose:** How long without keyboard/mouse activity before moving the mouse
- **Recommended Values:**
  - Testing: 5-10 seconds
  - Normal use: 300 seconds (5 minutes)
  - Prevent screensaver: 600 seconds (10 minutes)

### heartbeat_interval

- **Unit:** Seconds
- **Default:** 10 seconds
- **Purpose:** How often to check activity status
- **Recommended Values:** 10-60 seconds

### worker_interval

- **Unit:** Seconds
- **Default:** 10 seconds
- **Purpose:** Worker thread check interval
- **Recommended Values:** 10-60 seconds

### movement_delta

- **Unit:** Pixels
- **Default:** 10 pixels
- **Purpose:** Distance to move the mouse
- **Recommended Values:** 5-20 pixels (too large is noticeable)

### max_errors

- **Unit:** Count
- **Default:** 10 times
- **Purpose:** Stop after this many errors
- **Recommended Values:** 10-100 times

### auto_start

- **Type:** Boolean (true/false)
- **Default:** false
- **Purpose:** Whether to automatically start monitoring on launch
- **Recommended Values:** false (manual control is safer)

## Configuration Examples

### Example 1: Quick Testing

```json
{
  "inactivity_threshold": 5,
  "heartbeat_interval": 1,
  "worker_interval": 1,
  "movement_delta": 10,
  "max_errors": 10,
  "auto_start": false
}
```

**Effect:** Moves mouse after 5 seconds of inactivity, checks every second

### Example 2: Normal Use

```json
{
  "inactivity_threshold": 300,
  "heartbeat_interval": 60,
  "worker_interval": 60,
  "movement_delta": 5,
  "max_errors": 50,
  "auto_start": false
}
```

**Effect:** Moves mouse after 5 minutes of inactivity, checks every minute

### Example 3: Prevent Screensaver

```json
{
  "inactivity_threshold": 540,
  "heartbeat_interval": 60,
  "worker_interval": 60,
  "movement_delta": 3,
  "max_errors": 100,
  "auto_start": true
}
```

**Effect:** Moves after 9 minutes (assuming screensaver is 10 minutes), small movement

## Checking Current Configuration

### macOS

```bash
# Check if config file exists
ls -la ~/Library/Application\ Support/rmm/config.json

# View config content
cat ~/Library/Application\ Support/rmm/config.json

# Pretty print
cat ~/Library/Application\ Support/rmm/config.json | python3 -m json.tool
```

### Linux

```bash
# Check if config file exists
ls -la ~/.config/rmm/config.json

# View config content
cat ~/.config/rmm/config.json

# Pretty print
cat ~/.config/rmm/config.json | jq .
```

### Windows

```powershell
# Check if config file exists
Test-Path "$env:APPDATA\rmm\config.json"

# View config content
Get-Content "$env:APPDATA\rmm\config.json"
```

### If File Doesn't Exist

```bash
# Shows: No such file or directory
# This means the program is using default configuration (all values are 10)
```

## Frequently Asked Questions

### Q: Why doesn't my configuration change take effect?

**A:** You need to restart RMM.app. Configuration is only loaded once at startup.

### Q: Will the config file be created automatically?

**A:** No. The current version requires manual creation of the config file.

### Q: What happens if the config file format is wrong?

**A:** The program will fail to parse the JSON and may use default configuration or report an error. Use a JSON validator to check the format.

### Q: Where can I see the current configuration being used?

**A:** There's currently no UI to display it. You can:
1. View the config file content
2. If the file doesn't exist, default values are used (all values are 10)

### Q: Can I reload configuration without restarting?

**A:** No. The current implementation only loads configuration at startup.

## Troubleshooting

### Configuration Not Working

1. **Check file location is correct**
   ```bash
   # macOS
   ls -la ~/Library/Application\ Support/rmm/config.json
   ```

2. **Validate JSON format**
   ```bash
   # macOS/Linux
   cat ~/Library/Application\ Support/rmm/config.json | python3 -m json.tool
   
   # If this fails, your JSON is invalid
   ```

3. **Check file permissions**
   ```bash
   # macOS/Linux
   chmod 644 ~/Library/Application\ Support/rmm/config.json
   ```

4. **Restart the application**
   - Quit RMM from menu bar
   - Launch RMM.app again

### Invalid JSON Errors

Common JSON mistakes:

```json
// ❌ WRONG - Trailing comma
{
  "inactivity_threshold": 300,
  "heartbeat_interval": 60,
}

// ✅ CORRECT - No trailing comma
{
  "inactivity_threshold": 300,
  "heartbeat_interval": 60
}

// ❌ WRONG - Comments not allowed in JSON
{
  // This is a comment
  "inactivity_threshold": 300
}

// ✅ CORRECT - No comments
{
  "inactivity_threshold": 300
}
```

## Summary

```
Configuration Source Priority:
1. If config.json exists → Use file configuration
2. If config.json doesn't exist → Use code default values

Configuration File Location:
- macOS: ~/Library/Application Support/rmm/config.json
- Linux: ~/.config/rmm/config.json
- Windows: %APPDATA%\rmm\config.json

Modifying Configuration:
1. Manually edit config.json
2. Restart RMM.app
3. New configuration takes effect
```

## Related Documentation

- [Main README](../readme.md) - General usage and installation
- [macOS Permissions](../macos/PERMISSIONS.md) - macOS-specific permission setup
- [Source Code](../src/config.rs) - Configuration implementation
