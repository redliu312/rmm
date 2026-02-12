# macOS Permissions Guide

RMM requires special permissions on macOS to monitor keyboard and mouse activity.

## Required Permissions

### Accessibility Permission (Required)

RMM needs **Accessibility** permission to:
- Monitor keyboard activity
- Monitor mouse activity  
- Move the mouse cursor

Without this permission, the app will run but won't be able to detect inactivity or move the mouse.

## Granting Permissions

### First Time Setup

1. **Launch RMM.app**
   - Double-click RMM.app (or right-click → Open if not code-signed)
   - The app icon will appear in the menu bar

2. **Grant Accessibility Permission**
   - macOS will show a dialog: "RMM would like to control this computer using accessibility features"
   - Click "Open System Settings"
   - Or manually go to: **System Settings → Privacy & Security → Accessibility**

3. **Enable RMM**
   - Find "RMM" in the list
   - Toggle the switch to **ON** (blue)
   - You may need to click the lock icon and enter your password first

4. **Restart RMM**
   - Quit RMM from the menu bar (click icon → Quit)
   - Launch RMM.app again
   - The app should now work properly

### Verifying Permissions

To check if RMM has the necessary permissions:

```bash
# Check Accessibility permission
sqlite3 ~/Library/Application\ Support/com.apple.TCC/TCC.db \
  "SELECT service, client, allowed FROM access WHERE service='kTCCServiceAccessibility' AND client LIKE '%rmm%';"
```

You should see a row with `allowed = 1`.

## Troubleshooting

### App Runs But Mouse Doesn't Move

**Symptom:** Icon appears in menu bar, but mouse doesn't move after inactivity

**Solution:**
1. Check Accessibility permission is granted
2. Restart the app after granting permission
3. Check the configuration file has correct settings

### Permission Dialog Doesn't Appear

**Symptom:** No permission dialog when launching the app

**Solution:**
1. Manually go to System Settings → Privacy & Security → Accessibility
2. Click the "+" button
3. Navigate to RMM.app and add it
4. Toggle it ON

### "RMM.app" is damaged and can't be opened

**Symptom:** App won't launch at all

**Solution:**
```bash
# Remove quarantine attribute
xattr -cr /path/to/RMM.app

# Or right-click → Open (first time only)
```

See [`CODE_SIGNING.md`](CODE_SIGNING.md) for more details.

### Permission Denied Errors in Logs

**Symptom:** Errors in Console.app or logs about permissions

**Solution:**
1. Remove RMM from Accessibility list
2. Re-add it
3. Restart the app

## Testing the App

After granting permissions:

1. **Launch RMM.app**
2. **Wait for inactivity** (default: 10 seconds with no keyboard/mouse activity)
3. **Watch for mouse movement** - the cursor should move slightly
4. **Check menu bar icon** - should show the mouse icon

### Quick Test with Custom Config

Create a config file for faster testing:

```bash
# Create config directory
mkdir -p ~/Library/Application\ Support/rmm

# Create config with 5-second threshold
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

# Restart RMM.app
# Now it will move the mouse after just 5 seconds of inactivity
```

## Privacy Considerations

RMM only monitors:
- **When** keyboard/mouse events occur (for inactivity detection)
- Does **NOT** capture what keys are pressed
- Does **NOT** record mouse positions
- Does **NOT** send any data externally

All monitoring happens locally on your machine.

## Uninstalling

To completely remove RMM and its permissions:

1. **Quit RMM** from menu bar
2. **Remove from Accessibility**
   - System Settings → Privacy & Security → Accessibility
   - Select RMM and click "-" button
3. **Delete the app**
   ```bash
   rm -rf /Applications/RMM.app
   ```
4. **Remove config** (optional)
   ```bash
   rm -rf ~/Library/Application\ Support/rmm
   ```

## Summary

| Permission | Required | Purpose |
|------------|----------|---------|
| Accessibility | ✅ Yes | Monitor keyboard/mouse, move cursor |
| Screen Recording | ❌ No | Not needed |
| Input Monitoring | ❌ No | Covered by Accessibility |

The app will **not work** without Accessibility permission on macOS.
