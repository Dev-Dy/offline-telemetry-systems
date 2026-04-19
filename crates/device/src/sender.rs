use tokio::time::{Duration, sleep};

use common::message::Message;
use transport::tcp::connection::Connection;

use crate::queue::disk::DiskQueue;

pub async fn run(mut conn: Connection, mut queue: DiskQueue) {
    loop {
        if let Some(msg) = queue.peek() {
            tracing::info!(?msg, "sending message");

            // Try sending
            if let Err(e) = conn.send(msg).await {
                tracing::warn!(error = %e, "send failed, retrying");
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            // Wait for ACK
            match conn.read().await {
                Ok(Some(Message::Ack { message_id })) => {
                    tracing::info!(%message_id, "ack received");

                    if let Err(e) = queue.pop() {
                        tracing::error!(error = %e, "failed to pop from queue");
                    }
                }

                Ok(Some(other)) => {
                    tracing::warn!(?other, "unexpected response from server");
                }

                Ok(None) => {
                    tracing::warn!("connection closed by server");
                    sleep(Duration::from_secs(1)).await;
                }

                Err(e) => {
                    tracing::error!(error = %e, "read failed");
                    sleep(Duration::from_secs(1)).await;
                }
            }
        } else {
            // No messages → idle wait
            sleep(Duration::from_secs(1)).await;
        }
    }
}
