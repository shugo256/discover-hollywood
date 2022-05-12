#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    discover_hollywood_dataset::prepare().await?;

    discover_hollywood_server::start().await
}
