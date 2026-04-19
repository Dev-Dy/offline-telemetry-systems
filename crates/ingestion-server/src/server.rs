use tokio::net::TcpListener;

use common::message::Message;
use transport::tcp::connection::Connection;

pub async fn run(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    println!("server listening on {}", addr);

    loop {
        let (stream, peer) = listener.accept().await?;

        println!("new connection: {}", peer);

        tokio::spawn(async move {
            let mut conn = Connection::new(stream);

            loop {
                match conn.read().await {
                    Ok(Some(msg)) => {
                        println!("received: {:?}", msg);

                        if let Message::Data { message_id, .. } = msg {
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
