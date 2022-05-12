#[macro_use]
extern crate diesel;

/// Domain models of the application.
pub mod models;

/// Database schema module auto-generated by [Diesel](https://diesel.rs).
pub mod schema;

/// Main logic of the application.
///
/// This module includes interaction with db, request/response DTOs and error-handling
pub mod usecase;
