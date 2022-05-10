#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    discover_hollywood_dataset::prepare().await?;
    Ok(())
}
