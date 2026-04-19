use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use common::message::Message;
use transport::tcp::connection::Connection;

use crate::dedup::Dedup;

pub async fn run(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    println!("server listening on {}", addr);

    let dedup = Arc::new(Mutex::new(Dedup::new()));

    loop {
        let (stream, peer) = listener.accept().await?;

        println!("new connection: {}", peer);

        let dedup = dedup.clone();

        tokio::spawn(async move {
            let mut conn = Connection::new(stream);

            loop {
                match conn.read().await {
                    Ok(Some(msg)) => {
                        if let Message::Data { message_id, .. } = msg {
                            let mut store = dedup.lock().await;

                            if store.is_duplicate(&message_id) {
                                println!("duplicate ignored: {}", message_id);
                            } else {
                                println!("processing message: {}", message_id);
                                store.mark_seen(message_id.clone());
                            }

                            // ALWAYS send ACK
                            let ack = Message::Ack { message_id };

                            if let Err(e) = conn.send(&ack).await {
                                eprintln!("send error: {}", e);
                                break;
                            }
                        }
                    }

                    Ok(None) => {
                        println!("connection closed: {}", peer);
                        break;
                    }

                    Err(e) => {
                        eprintln!("read error: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
