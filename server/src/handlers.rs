use actix_web::{get, web, HttpResponse, Result};

use discover_hollywood_core::usecase::{dtos::SearchQuery, UseCase};

/// Handler for `GET /movies/:id`
///
/// Returns a Movie that corresponds to `:id`.
/// Internally calls [`UseCase::get_movie`](UseCase).
///
/// ## Response Body
/// [`MovieInfo`](discover_hollywood_core::usecase::dtos::MovieInfo)
///
/// ## Status
/// * `200`: Successfully found a movie.
/// * `404`: Movie of `:id` not found.
#[get("/movies/{id}")]
pub(crate) async fn get_movie(
    id: web::Path<String>,
    usecase: web::Data<UseCase>,
) -> Result<HttpResponse> {
    let movie = usecase.get_movie(id.as_str())?;
    Ok(HttpResponse::Ok().json(movie))
}

/// Handler for `GET /movies/search`
///
/// Returns a Movie that matches the given query.
/// Internally calls [`UseCase::search_movie`](UseCase).
///
/// ## Query Param
/// [`SearchQuery`](discover_hollywood_core::usecase::dtos::SearchQuery)
///
/// ## Response Body
/// [`SearchResult`](discover_hollywood_core::usecase::dtos::SearchResponse)
///
/// ## Status
/// * `200`: Successfully searched for the query.
#[get("/movies/search")]
pub(crate) async fn search_movie(
    query: web::Query<SearchQuery>,
    usecase: web::Data<UseCase>,
) -> Result<HttpResponse> {
    let movie = usecase.search_movie(query.into_inner())?;
    Ok(HttpResponse::Ok().json(movie))
}
