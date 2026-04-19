use tokio::time::{Duration, sleep};

use common::message::Message;
use transport::tcp::connection::Connection;

use crate::queue::disk::DiskQueue;

pub async fn run(mut conn: Connection, mut queue: DiskQueue) {
    loop {
        if let Some(msg) = queue.peek() {
            println!("sending: {:?}", msg);

            if let Err(e) = conn.send(msg).await {
                eprintln!("send failed: {}", e);
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            match conn.read().await {
                Ok(Some(Message::Ack { message_id })) => {
                    println!("ack received: {}", message_id);
                    if let Err(e) = queue.pop() {
                        eprint!("pop failed: {}", e);
                    };
                }
                Ok(_) => {
                    println!("unexpected response");
                }
                Err(e) => {
                    eprintln!("read failed: {}", e);
                }
            }
        } else {
            sleep(Duration::from_secs(1)).await;
        }
    }
}
