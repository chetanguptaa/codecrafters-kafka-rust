mod codec;
mod protocol;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::listener::run("127.0.0.1:9092").await
}
