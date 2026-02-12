#!/bin/bash
#
# Install git hooks
#

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_DIR="$(git rev-parse --git-dir)"

echo "Installing git hooks..."

# Make hooks executable
chmod +x "$SCRIPT_DIR/pre-commit"

# Create symlink to pre-commit hook
ln -sf "$SCRIPT_DIR/pre-commit" "$GIT_DIR/hooks/pre-commit"

echo "Git hooks installed successfully!"
echo "Pre-commit hook will now run 'cargo fmt' before each commit."
