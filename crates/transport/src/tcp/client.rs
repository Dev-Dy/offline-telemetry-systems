use tokio::net::TcpStream;

use crate::tcp::connection::Connection;

pub async fn connect(addr: &str) -> std::io::Result<Connection> {
    let stream = TcpStream::connect(addr).await?;
    Ok(Connection::new(stream))
}
