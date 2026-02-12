.PHONY: help build test clean app dmg install

help:
	@echo "RMM - Rust Mouse Mover"
	@echo ""
	@echo "Available targets:"
	@echo "  make build    - Build release binary"
	@echo "  make test     - Run tests"
	@echo "  make app      - Build macOS .app bundle (macOS only)"
	@echo "  make dmg      - Create macOS DMG installer (macOS only)"
	@echo "  make install  - Install to /Applications (macOS only)"
	@echo "  make clean    - Clean build artifacts"

build:
	cargo build --release

test:
	cargo test

clean:
	cargo clean
	rm -rf target/release/RMM.app
	rm -f target/release/RMM.dmg

app:
	@if [ "$$(uname)" != "Darwin" ]; then \
		echo "Error: .app bundles can only be built on macOS"; \
		exit 1; \
	fi
	./macos/build-app.sh

dmg: app
	@echo "Creating DMG installer..."
	@hdiutil create -volname RMM \
		-srcfolder target/release/RMM.app \
		-ov -format UDZO \
		target/release/RMM.dmg
	@echo "✓ DMG created at target/release/RMM.dmg"

install: app
	@echo "Installing RMM.app to /Applications..."
	@cp -R target/release/RMM.app /Applications/
	@echo "✓ RMM.app installed to /Applications"
