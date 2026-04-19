#!/bin/sh

echo "Starting system..."


cargo check || exit 1


cargo run -p ingestion-server &
SERVER_PID=$!

sleep 2

cargo run -p device


kill $SERVER_PID
