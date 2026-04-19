use tokio::time::{Duration, sleep};

use crate::metrics::METRICS;
use common::message::Message;
use transport::tcp::client::connect;

use queue::disk::DiskQueue;

mod metrics;
mod queue;
mod retry;
mod sender;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_target(false).init();

    let mut queue = DiskQueue::new("queue.log")?;

    // seed only if empty
    if queue.is_empty() {
        for i in 0..5 {
            if let Err(e) = queue.push(Message::Data {
                device_id: "device-1".into(),
                message_id: format!("msg-{}", i),
            }) {
                tracing::error!(error = %e, "failed to seed queue");
            }
        }
    }

    let conn = connect("127.0.0.1:8080").await?;

    tokio::spawn(async {
        loop {
            sleep(Duration::from_secs(5)).await;

            tracing::info!(
                sent = METRICS.sent.load(std::sync::atomic::Ordering::Relaxed),
                retries = METRICS.retries.load(std::sync::atomic::Ordering::Relaxed),
                failed = METRICS.failed.load(std::sync::atomic::Ordering::Relaxed),
                "device metrics"
            );
        }
    });

    sender::run(conn, queue).await;

    Ok(())
}
