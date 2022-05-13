/// Main function of the entire project.
///
/// Prepare the dataset using [`discover_hollywood_dataset`] and then start the server using [`discover_hollywood_server`].
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    discover_hollywood_dataset::prepare().await?;

    discover_hollywood_server::start().await
}
