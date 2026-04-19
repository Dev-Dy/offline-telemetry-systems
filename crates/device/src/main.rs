mod queue;
mod retry;
mod sender;

use common::message::Message;
use transport::tcp::client::connect;

use queue::disk::DiskQueue;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_target(false).init();
    let mut queue = DiskQueue::new("queue.log").expect("failed to init queue");
    // seed only if empty
    if queue.is_empty() {
        for i in 0..5 {
            if let Err(e) = queue.push(Message::Data {
                device_id: "device-1".into(),
                message_id: format!("msg-{}", i),
            }) {
                tracing::error!(error = %e, "failed to seed queue");
            };
        }
    }

    let conn = connect("127.0.0.1:8080").await.expect("connect failed");

    sender::run(conn, queue).await;

    Ok(())
}
