#!/usr/bin/env bash

set -euo pipefail

echo "Running pre-commit checks..."

# Get list of staged files

STAGED_FILES=$(git diff --cached --name-only)

# Check if any staged file has unstaged changes

for file in $STAGED_FILES; do
if ! git diff --quiet -- "$file"; then
echo "❌ File '$file' has unstaged changes. Please stage all changes before committing."
exit 1
fi
done

# Format check

cargo fmt --all -- --check || {
echo "❌ Formatting issues. Run: cargo fmt"
exit 1
}

# Lint

cargo clippy --all-targets --all-features -- -D warnings || {
echo "❌ Clippy failed"
exit 1
}

# Build check

cargo check || {
echo "❌ Build failed"
exit 1
}

echo "✅ All checks passed"
