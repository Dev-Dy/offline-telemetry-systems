use bytes::{Buf, BytesMut};
use common::message::Message;

pub fn try_decode(buf: &mut BytesMut) -> Option<Message> {
    // need at least header
    if buf.len() < 4 {
        return None;
    }

    let mut header = &buf[..4];
    let len = header.get_u32() as usize;

    // wait for full frame
    if buf.len() < 4 + len {
        return None;
    }

    // consume header
    buf.advance(4);

    // extract payload
    let payload = buf.split_to(len);

    serde_json::from_slice(&payload).ok()
}
