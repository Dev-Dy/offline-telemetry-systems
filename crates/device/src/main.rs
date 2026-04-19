use common::message::Message;
use transport::tcp::client::connect;

#[tokio::main]
async fn main() {
    let mut conn = connect("127.0.0.1:8080").await.expect("failed to connect");

    let msg = Message::Data {
        device_id: "device-1".into(),
        message_id: "msg-1".into(),
    };

    conn.send(&msg).await.expect("send failed");

    if let Some(response) = conn.read().await.expect("read failed") {
        println!("received from server: {:?}", response);
    }
}
