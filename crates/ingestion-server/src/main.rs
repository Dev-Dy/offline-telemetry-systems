mod dedup;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_target(false).init();

    server::run("127.0.1:8080").await
}
