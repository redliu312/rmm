# Code Signing for macOS

This document explains how to code sign RMM.app for distribution on macOS.

## Why Code Signing?

macOS Gatekeeper requires apps to be code-signed to run without security warnings. Without code signing, users see:

> "RMM.app" is damaged and can't be opened. You should move it to the Trash.

## Signing Methods

### 1. Ad-hoc Signing (Local Development)

**What it is:** Self-signing without an Apple Developer certificate  
**Use case:** Local development and testing  
**Limitation:** Cannot be distributed to other users

```bash
# The build script does this automatically
codesign --force --deep --sign - target/release/RMM.app
```

**To run an ad-hoc signed app:**
- Right-click → Open (first time only)
- Or: `xattr -cr target/release/RMM.app` then double-click

### 2. Developer ID Signing (Distribution)

**What it is:** Official signing with Apple Developer certificate  
**Use case:** Distributing to users outside the App Store  
**Requirement:** Apple Developer Program membership ($99/year)

#### Steps to Get Developer ID Certificate

1. **Join Apple Developer Program**
   - Go to https://developer.apple.com/programs/
   - Enroll ($99/year)

2. **Create Certificate**
   - Go to https://developer.apple.com/account/resources/certificates
   - Click "+" to create new certificate
   - Choose "Developer ID Application"
   - Follow instructions to generate CSR (Certificate Signing Request)
   - Download and install the certificate

3. **Verify Certificate**
   ```bash
   security find-identity -v -p codesigning
   ```
   You should see: `Developer ID Application: Your Name (TEAM_ID)`

4. **Sign the App**
   ```bash
   # The build script will automatically detect and use your Developer ID
   make app
   
   # Or manually:
   codesign --force --deep \
     --sign "Developer ID Application: Your Name (TEAM_ID)" \
     --options runtime \
     --timestamp \
     target/release/RMM.app
   ```

5. **Verify Signature**
   ```bash
   codesign --verify --deep --strict --verbose=2 target/release/RMM.app
   spctl --assess --verbose=4 target/release/RMM.app
   ```

### 3. Notarization (Required for Distribution)

For apps distributed outside the App Store, Apple requires notarization (since macOS 10.15 Catalina).

#### Steps to Notarize

1. **Create App-Specific Password**
   - Go to https://appleid.apple.com
   - Sign in → Security → App-Specific Passwords
   - Generate password for "RMM Notarization"

2. **Submit for Notarization**
   ```bash
   # Create a ZIP of the app
   ditto -c -k --keepParent target/release/RMM.app target/release/RMM.zip
   
   # Submit to Apple
   xcrun notarytool submit target/release/RMM.zip \
     --apple-id "your-email@example.com" \
     --password "app-specific-password" \
     --team-id "YOUR_TEAM_ID" \
     --wait
   ```

3. **Staple the Notarization**
   ```bash
   xcrun stapler staple target/release/RMM.app
   ```

4. **Verify Notarization**
   ```bash
   spctl --assess -vv --type install target/release/RMM.app
   ```

## GitHub Actions Integration

To sign apps in CI/CD, you need to:

1. **Export Certificate**
   ```bash
   # Export from Keychain
   security find-identity -v -p codesigning
   # Export the certificate as .p12 file with password
   ```

2. **Add to GitHub Secrets**
   - `MACOS_CERTIFICATE`: Base64-encoded .p12 file
   - `MACOS_CERTIFICATE_PASSWORD`: Certificate password
   - `APPLE_ID`: Your Apple ID email
   - `APPLE_ID_PASSWORD`: App-specific password
   - `APPLE_TEAM_ID`: Your Team ID

3. **Update Workflow**
   ```yaml
   - name: Import Certificate
     run: |
       echo "${{ secrets.MACOS_CERTIFICATE }}" | base64 --decode > certificate.p12
       security create-keychain -p actions build.keychain
       security default-keychain -s build.keychain
       security unlock-keychain -p actions build.keychain
       security import certificate.p12 -k build.keychain -P "${{ secrets.MACOS_CERTIFICATE_PASSWORD }}" -T /usr/bin/codesign
       security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k actions build.keychain
   ```

## Workarounds for Users (No Developer Account)

If you don't have an Apple Developer account, users can still run the app:

### Method 1: Right-Click Open
1. Right-click (or Control-click) on RMM.app
2. Select "Open"
3. Click "Open" in the dialog
4. App will run and be remembered

### Method 2: Remove Quarantine
```bash
xattr -cr /path/to/RMM.app
```

### Method 3: System Settings
1. Try to open the app (it will be blocked)
2. Go to System Settings → Privacy & Security
3. Click "Open Anyway" next to the RMM.app message
4. Confirm by clicking "Open"

## Current Build Script Behavior

The `macos/build-app.sh` script automatically:

1. **Checks for Developer ID certificate**
   - If found: Signs with Developer ID
   - If not found: Uses ad-hoc signing

2. **Removes quarantine attribute**
   - Makes locally-built apps easier to run

3. **Provides instructions**
   - Tells you what type of signing was used
   - Explains limitations

## Summary

| Method | Cost | Distribution | User Experience |
|--------|------|--------------|-----------------|
| Ad-hoc | Free | Local only | Requires workaround |
| Developer ID | $99/year | Public | Works with workaround |
| Developer ID + Notarization | $99/year | Public | Works seamlessly |

For professional distribution, you need:
- ✅ Apple Developer Program membership
- ✅ Developer ID Application certificate
- ✅ Notarization with Apple

For local development:
- ✅ Ad-hoc signing (automatic in build script)
- ✅ Users use right-click → Open workaround
