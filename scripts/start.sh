#!/usr/bin/env bash

set -euo pipefail

echo "Starting system..."

# Cleanup function

cleanup() {
echo "Shutting down..."
if [[ -n "${SERVER_PID:-}" ]]; then
kill "$SERVER_PID" 2>/dev/null || true
wait "$SERVER_PID" 2>/dev/null || true
fi
}

# Ensure cleanup runs on exit or Ctrl+C

trap cleanup EXIT INT TERM

# Step 1: Build check

echo "Running cargo check..."
cargo check

# Step 2: Start server

echo "Starting ingestion server..."
cargo run -p ingestion-server &
SERVER_PID=$!

# Wait for server to be ready (better than blind sleep)

echo "Waiting for server to be ready..."
sleep 2

# Step 3: Run device

echo "Starting device..."
cargo run -p device

echo "Device finished"
