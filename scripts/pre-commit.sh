#!/bin/sh

echo "Running pre-commit checks..."

# Ensure no unstaged changes

if ! git diff --quiet; then
echo "❌ Unstaged changes found. Stage everything first."
exit 1
fi

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
