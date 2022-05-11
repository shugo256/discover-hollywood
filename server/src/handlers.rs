use actix_web::{get, web, HttpResponse, Result};

use crate::usecase::{dtos::SearchQuery, UseCase};

#[get("/{id}")]
pub async fn get_movie(id: web::Path<String>, usecase: web::Data<UseCase>) -> Result<HttpResponse> {
    let movie = usecase.get_movie(id.as_str())?;
    Ok(HttpResponse::Ok().json(movie))
}

#[get("/search")]
pub async fn search_movie(
    query: web::Query<SearchQuery>,
    usecase: web::Data<UseCase>,
) -> Result<HttpResponse> {
    let movie = usecase.search_movie(query.into_inner())?;
    Ok(HttpResponse::Ok().json(movie))
}
