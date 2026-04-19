use std::sync::atomic::{AtomicU64, Ordering};

pub struct Metrics {
    pub sent: AtomicU64,
    pub retries: AtomicU64,
    pub failed: AtomicU64,
}

impl Metrics {
    pub const fn new() -> Self {
        Self {
            sent: AtomicU64::new(0),
            retries: AtomicU64::new(0),
            failed: AtomicU64::new(0),
        }
    }

    pub fn inc_sent(&self) {
        self.sent.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_retry(&self) {
        self.retries.fetch_add(1, Ordering::Relaxed);
    }
    #[allow(unused)]
    pub fn inc_failed(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }
}

pub static METRICS: Metrics = Metrics::new();
