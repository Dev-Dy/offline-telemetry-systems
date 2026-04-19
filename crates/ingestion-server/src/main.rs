mod dedup;
mod server;

#[tokio::main]
async fn main() {
    if let Err(e) = server::run("127.0.0.1:8080").await {
        eprintln!("server error: {}", e);
    }
}
