# macOS .app Bundle

This directory contains files needed to build a native macOS .app bundle for RMM.

## Files

- **Info.plist** - Bundle metadata and configuration
- **build-app.sh** - Build script that creates the .app bundle

## Building

From the project root:

```bash
# Build the .app bundle
make app

# Or run the script directly
./macos/build-app.sh
```

The resulting `RMM.app` will be created in `target/release/RMM.app`.

## .app Bundle Structure

A macOS .app bundle is a directory with a specific structure:

```
RMM.app/
└── Contents/
    ├── Info.plist           # Bundle metadata
    ├── MacOS/
    │   └── rmm              # The executable binary
    └── Resources/
        └── icon.icns        # Application icon
```

## Info.plist

The Info.plist file contains important metadata:

- **CFBundleIdentifier**: `com.redliu312.rmm` - Unique bundle identifier
- **CFBundleExecutable**: `rmm` - Name of the executable in MacOS/
- **CFBundleIconFile**: `icon.icns` - Icon file in Resources/
- **LSUIElement**: `true` - Makes the app a menu bar app (no Dock icon)
- **NSHighResolutionCapable**: `true` - Enables Retina display support

## Icon Conversion

The build script automatically converts `resources/mouse.png` to `.icns` format using macOS built-in tools:

1. **sips** - Resizes the PNG to multiple sizes (16x16 to 1024x1024)
2. **iconutil** - Converts the iconset to .icns format

The .icns file contains multiple resolutions for different display contexts.

## DMG Creation

To create a DMG installer:

```bash
make dmg
```

This creates a compressed DMG file that users can download and mount to install the app.

## Installation

Users can install the app by:

1. Downloading the DMG file
2. Opening it (mounts as a disk image)
3. Dragging RMM.app to their Applications folder
4. Double-clicking RMM.app to run

## Differences from Unix Executable

| Aspect | Unix Executable | .app Bundle |
|--------|----------------|-------------|
| File type | Single binary file | Directory structure |
| Finder display | "Unix Executable File" | "Application" |
| Double-click | Opens in Terminal | Launches app |
| Icon | Generic document icon | Custom app icon |
| Installation | Manual copy | Drag to Applications |
| Uninstallation | Delete file | Move to Trash |
| Menu bar integration | Works | Works (preferred) |

## Why .app Bundle?

The .app bundle provides a better user experience on macOS:

- ✅ Appears as a native application in Finder
- ✅ Can be double-clicked to launch
- ✅ Shows custom icon
- ✅ Can be installed via drag-and-drop
- ✅ Integrates with Spotlight search
- ✅ Follows macOS conventions

This is why the Go project (automatic-mouse-mover) uses .app bundles - it provides a more polished, native macOS experience.
