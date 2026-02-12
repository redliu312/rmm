#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

APP_PATH="${1:-target/release/RMM.app}"

if [ ! -d "$APP_PATH" ]; then
    echo "Error: $APP_PATH not found"
    echo "Usage: $0 [path/to/RMM.app]"
    exit 1
fi

echo -e "${BLUE}Code Signing RMM.app...${NC}"

# Check if user has a signing identity
SIGNING_IDENTITY=$(security find-identity -v -p codesigning | grep "Developer ID Application" | head -n 1 | awk -F'"' '{print $2}')

if [ -z "$SIGNING_IDENTITY" ]; then
    echo -e "${YELLOW}No Developer ID Application certificate found.${NC}"
    echo ""
    echo "Options:"
    echo "1. Ad-hoc signing (for local use only, not for distribution)"
    echo "2. Skip signing and use workaround"
    echo ""
    read -p "Choose option (1 or 2): " choice
    
    if [ "$choice" = "1" ]; then
        echo -e "${BLUE}Performing ad-hoc signing...${NC}"
        codesign --force --deep --sign - "$APP_PATH"
        echo -e "${GREEN}✓ Ad-hoc signed${NC}"
        echo ""
        echo "Note: Ad-hoc signed apps work locally but cannot be distributed."
        echo "For distribution, you need an Apple Developer account."
    else
        echo -e "${YELLOW}Skipping code signing.${NC}"
        echo ""
        echo "To run the app, use one of these methods:"
        echo "1. Right-click RMM.app → Open (first time only)"
        echo "2. Run: xattr -cr '$APP_PATH'"
        echo "3. System Settings → Privacy & Security → Allow anyway"
    fi
else
    echo -e "${GREEN}Found signing identity: $SIGNING_IDENTITY${NC}"
    echo -e "${BLUE}Signing with Developer ID...${NC}"
    
    # Sign the app with hardened runtime
    codesign --force --deep \
        --sign "$SIGNING_IDENTITY" \
        --options runtime \
        --timestamp \
        "$APP_PATH"
    
    echo -e "${GREEN}✓ Signed with Developer ID${NC}"
    
    # Verify the signature
    echo -e "${BLUE}Verifying signature...${NC}"
    codesign --verify --deep --strict --verbose=2 "$APP_PATH"
    
    echo -e "${GREEN}✓ Signature verified${NC}"
    echo ""
    echo "The app is now properly signed and can be distributed."
    echo ""
    echo "For notarization (required for distribution outside App Store):"
    echo "1. Create an app-specific password at appleid.apple.com"
    echo "2. Run: xcrun notarytool submit '$APP_PATH' --apple-id YOUR_EMAIL --password APP_PASSWORD --team-id TEAM_ID"
fi
