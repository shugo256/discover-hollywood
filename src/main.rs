use discover_hollywood::dataset;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dataset::prepare().await?;
    Ok(())
}
