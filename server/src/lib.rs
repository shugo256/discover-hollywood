/// Set of HTTP Request Handlers
///
/// This module is responsible for
/// 1. Mapping Actix Web request objects into UseCase request DTOs
/// 1. Calling the UseCase using the DTOs.
/// 1. And then converting UseCase response DTOs into Actix Web response objects.
mod handlers;

use actix_files::{Files, NamedFile};
use actix_web::{middleware::Logger, web, App, HttpServer, Result};
use discover_hollywood_core::usecase::UseCase;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./client/dist/index.html")?)
}

/// Start the server with Actix Web.
pub async fn start() -> anyhow::Result<()> {
    let usecase = web::Data::new(UseCase::new()?);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a \"%r\" %s (%T s)"))
            .app_data(usecase.clone())
            .service(handlers::search_movie)
            .service(handlers::get_movie)
            .service(Files::new("/", "./client/dist/").index_file("index.html"))
            .default_service(web::route().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
