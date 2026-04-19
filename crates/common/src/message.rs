use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    Data {
        device_id: String,
        message_id: String,
    },
    Ack {
        message_id: String,
    },
}
