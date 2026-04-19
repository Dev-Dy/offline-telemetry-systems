use bytes::{BufMut, BytesMut};
use common::message::Message;

pub fn encode(msg: &Message) -> Result<BytesMut, serde_json::Error> {
    let payload = serde_json::to_vec(msg)?;

    let mut buf = BytesMut::with_capacity(4 + payload.len());

    // length prefix (big-endian u32)
    buf.put_u32(payload.len() as u32);
    buf.extend_from_slice(&payload);

    Ok(buf)
}
