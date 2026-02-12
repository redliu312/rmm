# Git Hooks

This directory contains Git hooks for the project.

## Installation

To install the hooks, run:

```bash
bash .githooks/install.sh
```

Or manually:

```bash
chmod +x .githooks/pre-commit
ln -sf ../../.githooks/pre-commit .git/hooks/pre-commit
```

## Pre-commit Hook

The pre-commit hook automatically runs `cargo fmt` to format your Rust code before each commit.

If the code is not properly formatted:
1. The hook will format the code
2. The commit will be aborted
3. You need to review the changes and commit again

## Bypassing Hooks

If you need to bypass the hooks (not recommended), use:

```bash
git commit --no-verify
```
