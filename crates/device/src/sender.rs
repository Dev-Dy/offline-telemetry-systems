use tokio::time::{Duration, sleep, timeout};

use common::message::Message;
use transport::tcp::connection::Connection;

use crate::queue::disk::DiskQueue;
use crate::retry::backoff;

pub async fn run(mut conn: Connection, mut queue: DiskQueue) {
    let mut attempt = 0;

    loop {
        if let Some(msg) = queue.peek() {
            tracing::info!(?msg, "sending message");

            // SEND WITH TIMEOUT
            let send_result = timeout(Duration::from_secs(2), conn.send(msg)).await;

            match send_result {
                Ok(Ok(())) => {
                    // WAIT FOR ACK WITH TIMEOUT
                    let read_result = timeout(Duration::from_secs(2), conn.read()).await;

                    match read_result {
                        Ok(Ok(Some(Message::Ack { message_id }))) => {
                            tracing::info!(%message_id, "ack received");

                            if let Err(e) = queue.pop() {
                                tracing::error!(error = %e, "failed to pop queue");
                            }

                            attempt = 0; // reset backoff
                        }

                        Ok(Ok(_)) => {
                            tracing::warn!("unexpected response");
                        }

                        Ok(Err(e)) => {
                            tracing::error!(error = %e, "read error");
                        }

                        Err(_) => {
                            tracing::warn!("ack timeout");
                        }
                    }
                }

                Ok(Err(e)) => {
                    tracing::error!(error = %e, "send failed");
                }

                Err(_) => {
                    tracing::warn!("send timeout");
                }
            }

            // BACKOFF
            let delay = backoff(attempt);
            tracing::info!(?delay, attempt, "retrying after backoff");

            sleep(delay).await;
            attempt += 1;
        } else {
            sleep(Duration::from_secs(1)).await;
        }
    }
}
