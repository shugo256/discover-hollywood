use actix_web::{middleware::Logger, web, App, HttpServer};
use discover_hollywood_server::{
    handlers::{get_movie, search_movie},
    usecase::UseCase,
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    discover_hollywood_dataset::prepare().await?;

    let usecase = web::Data::new(UseCase::new()?);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a \"%r\" %s (%T s)"))
            .app_data(usecase.clone())
            .service(search_movie)
            .service(get_movie)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
