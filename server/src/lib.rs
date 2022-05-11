/// Set of HTTP Request Handlers
///
/// This module is responsible for
/// 1. Mapping Actix Web request objects into UseCase request DTOs
/// 1. Calling the UseCase using the DTOs.
/// 1. And then converting UseCase response DTOs into Actix Web response objects.
pub mod handlers;

/// Main logic of the application.
///
/// This module includes interaction with db, request/response DTOs and error-handling
pub mod usecase;
