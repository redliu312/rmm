#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Building RMM.app for macOS...${NC}"

# Configuration
APP_NAME="RMM"
BUNDLE_ID="com.redliu312.rmm"
VERSION="1.0.0"
BINARY_NAME="rmm"

# Paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_ROOT/target/release"
APP_DIR="$BUILD_DIR/$APP_NAME.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

echo -e "${BLUE}Step 1: Building release binary...${NC}"
cd "$PROJECT_ROOT"
cargo build --release

echo -e "${BLUE}Step 2: Creating .app bundle structure...${NC}"
rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

echo -e "${BLUE}Step 3: Copying Info.plist...${NC}"
cp "$SCRIPT_DIR/Info.plist" "$CONTENTS_DIR/Info.plist"

echo -e "${BLUE}Step 4: Converting PNG to ICNS...${NC}"
# Create iconset directory
ICONSET_DIR="$BUILD_DIR/icon.iconset"
rm -rf "$ICONSET_DIR"
mkdir -p "$ICONSET_DIR"

# Generate different icon sizes from the PNG
# macOS requires multiple sizes for the iconset
PNG_SOURCE="$PROJECT_ROOT/resources/mouse.png"

if command -v sips &> /dev/null; then
    # Use sips (built-in macOS tool) to resize
    sips -z 16 16     "$PNG_SOURCE" --out "$ICONSET_DIR/icon_16x16.png" > /dev/null 2>&1
    sips -z 32 32     "$PNG_SOURCE" --out "$ICONSET_DIR/icon_16x16@2x.png" > /dev/null 2>&1
    sips -z 32 32     "$PNG_SOURCE" --out "$ICONSET_DIR/icon_32x32.png" > /dev/null 2>&1
    sips -z 64 64     "$PNG_SOURCE" --out "$ICONSET_DIR/icon_32x32@2x.png" > /dev/null 2>&1
    sips -z 128 128   "$PNG_SOURCE" --out "$ICONSET_DIR/icon_128x128.png" > /dev/null 2>&1
    sips -z 256 256   "$PNG_SOURCE" --out "$ICONSET_DIR/icon_128x128@2x.png" > /dev/null 2>&1
    sips -z 256 256   "$PNG_SOURCE" --out "$ICONSET_DIR/icon_256x256.png" > /dev/null 2>&1
    sips -z 512 512   "$PNG_SOURCE" --out "$ICONSET_DIR/icon_256x256@2x.png" > /dev/null 2>&1
    sips -z 512 512   "$PNG_SOURCE" --out "$ICONSET_DIR/icon_512x512.png" > /dev/null 2>&1
    sips -z 1024 1024 "$PNG_SOURCE" --out "$ICONSET_DIR/icon_512x512@2x.png" > /dev/null 2>&1
    
    # Convert iconset to icns
    iconutil -c icns "$ICONSET_DIR" -o "$RESOURCES_DIR/icon.icns"
    rm -rf "$ICONSET_DIR"
    echo -e "${GREEN}✓ Icon converted successfully${NC}"
else
    echo "Warning: sips command not found. Skipping icon conversion."
    echo "The app will work but won't have a custom icon."
fi

echo -e "${BLUE}Step 5: Copying binary...${NC}"
cp "$BUILD_DIR/$BINARY_NAME" "$MACOS_DIR/$BINARY_NAME"
chmod +x "$MACOS_DIR/$BINARY_NAME"

echo -e "${BLUE}Step 6: Code signing...${NC}"
# Check for Developer ID certificate
SIGNING_IDENTITY=$(security find-identity -v -p codesigning 2>/dev/null | grep "Developer ID Application" | head -n 1 | awk -F'"' '{print $2}')

if [ -n "$SIGNING_IDENTITY" ]; then
    echo "Found Developer ID: $SIGNING_IDENTITY"
    codesign --force --deep \
        --sign "$SIGNING_IDENTITY" \
        --options runtime \
        --timestamp \
        "$APP_DIR" 2>/dev/null || {
        echo "Developer ID signing failed, falling back to ad-hoc signing"
        codesign --force --deep --sign - "$APP_DIR"
    }
    echo -e "${GREEN}✓ Signed with Developer ID${NC}"
else
    echo "No Developer ID found, using ad-hoc signing (local use only)"
    codesign --force --deep --sign - "$APP_DIR"
    echo -e "${GREEN}✓ Ad-hoc signed${NC}"
fi

echo -e "${BLUE}Step 7: Setting bundle attributes...${NC}"
# Remove quarantine attribute if present
xattr -cr "$APP_DIR" 2>/dev/null || true

echo -e "${GREEN}✓ Build complete!${NC}"
echo -e "${GREEN}Application bundle created at: $APP_DIR${NC}"
echo ""
echo "You can now:"
echo "  1. Double-click $APP_NAME.app to run it"
echo "  2. Drag it to /Applications folder"
echo "  3. Create a DMG: hdiutil create -volname $APP_NAME -srcfolder \"$APP_DIR\" -ov -format UDZO \"$BUILD_DIR/$APP_NAME.dmg\""
echo ""
if [ -z "$SIGNING_IDENTITY" ]; then
    echo "Note: App is ad-hoc signed (for local use only)."
    echo "For distribution, you need an Apple Developer account and Developer ID certificate."
    echo "See macos/CODE_SIGNING.md for details."
fi
