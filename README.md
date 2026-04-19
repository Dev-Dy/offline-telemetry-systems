# Offline Telemetry System (Rust)

A production-oriented telemetry ingestion system built in Rust, designed to handle unreliable networks with strong delivery guarantees.

---

## 🚀 Overview

This project simulates a real-world data ingestion pipeline where edge devices send telemetry data to a central server over TCP.

The system is built with a focus on:

* reliability
* fault tolerance
* idempotency
* observability

---

## 🧠 Architecture

```
Device → TCP Transport → Ingestion Server → ACK
```

### Components

* **Device**

  * Generates telemetry messages
  * Stores them in a disk-backed queue
  * Sends messages over TCP
  * Retries until acknowledged

* **Transport Layer**

  * Async TCP communication using Tokio
  * Length-prefixed framing
  * Buffered decoding

* **Protocol Layer**

  * JSON-based message format
  * Encodes/decodes messages safely

* **Ingestion Server**

  * Accepts TCP connections
  * Processes messages
  * Sends ACKs
  * Deduplicates messages

---

## 🔒 Reliability Guarantees

### At-least-once Delivery

* Messages are persisted on disk before sending
* Messages are removed only after receiving ACK
* Retries are performed with exponential backoff

### Idempotent Processing

* Each message has a unique `message_id`
* Server tracks processed messages
* Duplicate messages are ignored safely

---

## ⚙️ Features

* Disk-backed queue (crash-safe)
* Exponential retry backoff
* Timeout handling
* Structured logging (`tracing`)
* Async TCP transport
* Modular architecture (protocol, transport, device, server)

---

## 📂 Project Structure

```
crates/
  common/             # shared message types
  protocol/           # encoding/decoding
  transport/          # TCP abstraction
  device/             # telemetry producer
  ingestion-server/   # message consumer
```

---

## 🧪 Running the System

### Start the system

```bash
./scripts/start.sh
```

Or manually:

```bash
# terminal 1
cargo run -p ingestion-server

# terminal 2
cargo run -p device
```

---

## 📊 Logging

The project uses `tracing` for structured logging.

Set log level:

```bash
RUST_LOG=info cargo run -p ingestion-server
```

---

## 🧠 Design Decisions

* **TCP over HTTP** for low-level control and learning
* **Length-prefixed framing** to handle stream boundaries
* **Disk queue** for crash safety
* **In-memory deduplication** for simplicity (can be extended to persistent store)
* **Exponential backoff** to avoid overwhelming server

---

## ⚠️ Current Limitations

* Deduplication is in-memory (lost on server restart)
* Queue rewrite is not atomic
* No authentication or encryption
* No horizontal scaling yet

---

## 🚀 Future Improvements

* Persistent deduplication (disk/DB)
* Atomic file writes
* Metrics (Prometheus)
* Multi-device simulation
* Load testing
* Config management

---

## 🎯 Purpose

This project is designed to demonstrate:

* distributed systems fundamentals
* failure handling strategies
* reliability patterns used in production systems

---

## 📌 Key Concepts Covered

* at-least-once delivery
* idempotency
* retry/backoff strategies
* TCP stream handling
* structured logging
* async concurrency in Rust

---

## 🧠 Author Notes

This project focuses on correctness and system design over completeness, gradually evolving into a production-grade system through iterative improvements.
