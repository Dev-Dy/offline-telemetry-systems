use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use common::message::Message;
use transport::tcp::connection::Connection;

use crate::dedup::Dedup;

pub async fn run(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    tracing::info!(%addr, "server listening");

    let dedup = Arc::new(Mutex::new(Dedup::new()));

    loop {
        let (stream, peer) = listener.accept().await?;

        tracing::info!(%peer, "new connection");

        let dedup = dedup.clone();

        tokio::spawn(async move {
            let mut conn = Connection::new(stream);

            loop {
                match conn.read().await {
                    Ok(Some(msg)) => {
                        if let Message::Data { message_id, .. } = msg {
                            // Scope lock tightly
                            let is_duplicate = {
                                let mut store = dedup.lock().await;

                                if store.is_duplicate(&message_id) {
                                    true
                                } else {
                                    store.mark_seen(message_id.clone());
                                    false
                                }
                            };

                            if is_duplicate {
                                tracing::warn!(%message_id, "duplicate message ignored");
                            } else {
                                tracing::info!(%message_id, "processing message");
                            }

                            // ALWAYS send ACK (outside lock)
                            let ack = Message::Ack { message_id };

                            if let Err(e) = conn.send(&ack).await {
                                tracing::error!(%peer, error = %e, "failed to send ack");
                                break;
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
        });
    }
}
