use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use common::message::Message;
use protocol::{decoder::try_decode, encoder::encode};

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    // send message
    pub async fn send(&mut self, msg: &Message) -> std::io::Result<()> {
        let data = encode(msg).map_err(std::io::Error::other)?;
        self.stream.write_all(&data).await
    }

    // read next message
    pub async fn read(&mut self) -> std::io::Result<Option<Message>> {
        loop {
            // try decoding first
            if let Some(msg) = try_decode(&mut self.buffer) {
                return Ok(Some(msg));
            }

            let mut temp = [0u8; 1024];
            let n = self.stream.read(&mut temp).await?;

            if n == 0 {
                return Ok(None); // connection closed
            }

            self.buffer.extend_from_slice(&temp[..n]);
        }
    }
}
