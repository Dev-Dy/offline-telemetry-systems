use std::sync::Arc;
use tokio::net::TcpListener;

use common::message::Message;
use tracing::Instrument;
use transport::tcp::connection::Connection;

use crate::dedup::Dedup;

pub async fn run(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    tracing::info!(%addr, "server listening");

    let dedup = Arc::new(Dedup::new());

    loop {
        let (stream, peer) = listener.accept().await?;

        tracing::info!(%peer, "new connection");

        let dedup = dedup.clone();

        tokio::spawn(
            async move {
                let mut conn = Connection::new(stream);

                loop {
                    match conn.read().await {
                        Ok(Some(msg)) => {
                            match msg {
                                Message::Data { message_id, .. } => {
                                    let is_duplicate = dedup.check_and_insert(message_id.clone());

                                    if is_duplicate {
                                        tracing::warn!(
                                            %peer,
                                            %message_id,
                                            "duplicate message ignored"
                                        );
                                    } else {
                                        tracing::info!(
                                            %peer,
                                            %message_id,
                                            "processing message"
                                        );
                                    }

                                    // Always send ACK
                                    let ack = Message::Ack { message_id };

                                    if let Err(e) = conn.send(&ack).await {
                                        tracing::error!(
                                            %peer,
                                            error = %e,
                                            "failed to send ack"
                                        );
                                        break;
                                    }
                                }

                                other => {
                                    tracing::debug!(
                                        %peer,
                                        ?other,
                                        "unexpected message type"
                                    );
                                }
                            }
                        }

                        Ok(None) => {
                            tracing::info!(%peer, "connection closed");
                            break;
                        }

                        Err(e) => {
                            tracing::error!(%peer, error = %e, "read error");
                            break;
                        }
                    }
                }
            }
            .instrument(tracing::info_span!("connection", %peer)),
        );
    }
}
