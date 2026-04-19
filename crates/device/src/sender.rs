use tokio::time::{Duration, sleep, timeout};

use common::message::Message;
use transport::tcp::connection::Connection;

use crate::metrics::METRICS;
use crate::queue::disk::DiskQueue;
use crate::retry::backoff;

pub async fn run(mut conn: Connection, mut queue: DiskQueue) {
    let mut attempt = 0;

    loop {
        if let Some(msg) = queue.peek() {
            // Extract message_id for structured logging
            let message_id = match msg {
                Message::Data { message_id, .. } => message_id.clone(),
                _ => {
                    tracing::warn!("unexpected message type in queue");
                    continue;
                }
            };

            tracing::info!(
                %message_id,
                attempt,
                "sending message"
            );

            // SEND WITH TIMEOUT
            let send_result = timeout(Duration::from_secs(2), conn.send(msg)).await;

            let mut success = false;

            match send_result {
                Ok(Ok(())) => {
                    // WAIT FOR ACK WITH TIMEOUT
                    let read_result = timeout(Duration::from_secs(2), conn.read()).await;

                    match read_result {
                        Ok(Ok(Some(Message::Ack { message_id: ack_id }))) => {
                            tracing::info!(
                                %ack_id,
                                "ack received"
                            );

                            if let Err(e) = queue.pop() {
                                tracing::error!(error = %e, "failed to pop queue");
                            }

                            METRICS.inc_sent();
                            attempt = 0;
                            success = true;
                        }

                        Ok(Ok(_)) => {
                            tracing::warn!(
                                %message_id,
                                "unexpected response"
                            );
                        }

                        Ok(Err(e)) => {
                            tracing::error!(
                                %message_id,
                                error = %e,
                                "read error"
                            );
                        }

                        Err(_) => {
                            tracing::warn!(
                                %message_id,
                                "ack timeout"
                            );
                        }
                    }
                }

                Ok(Err(e)) => {
                    tracing::error!(
                        %message_id,
                        error = %e,
                        "send failed"
                    );
                }

                Err(_) => {
                    tracing::warn!(
                        %message_id,
                        "send timeout"
                    );
                }
            }

            // BACKOFF ONLY IF FAILED
            if !success {
                let delay = backoff(attempt);

                tracing::warn!(
                    %message_id,
                    attempt,
                    ?delay,
                    "retrying after backoff"
                );

                METRICS.inc_retry();

                sleep(delay).await;
                attempt += 1;
            }
        } else {
            sleep(Duration::from_secs(1)).await;
        }
    }
}
