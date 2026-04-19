mod queue;
mod sender;

use common::message::Message;
use transport::tcp::client::connect;

use queue::memory::MemoryQueue;

#[tokio::main]
async fn main() {
    let mut queue = MemoryQueue::new();

    // seed messages
    for i in 0..5 {
        queue.push(Message::Data {
            device_id: "device-1".into(),
            message_id: format!("msg-{}", i),
        });
    }

    let conn = connect("127.0.0.1:8080").await.expect("connect failed");

    sender::run(conn, queue).await;
}
