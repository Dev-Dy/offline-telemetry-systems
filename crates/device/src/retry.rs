use std::time::Duration;

pub fn backoff(attempt: u32) -> Duration {
    let base = 100; // ms
    let max = 5000;

    let delay = base * 2u64.pow(attempt.min(6)); // cap growth
    Duration::from_millis(delay.min(max))
}
