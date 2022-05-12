use actix_web::{http, HttpResponse, HttpResponseBuilder, ResponseError};
use derive_more::Display;
use serde_json::json;
use thiserror::Error;

pub type UseCaseResult<T> = Result<T, UseCaseError>;

/// Error type for the UseCase logic.
///
/// Each variants will be mapped into a HTTP status code.
#[derive(Clone, Debug, Display, Error, PartialEq)]
pub enum UseCaseError {
    #[display(fmt = "ResourceNotFound: {}", _0)]
    ResourceNotFound(String),

    #[display(fmt = "ResourceNotFound: {}", _0)]
    InternalError(String),
}

impl ResponseError for UseCaseError {
    fn status_code(&self) -> http::StatusCode {
        match *self {
            UseCaseError::ResourceNotFound { .. } => http::StatusCode::NOT_FOUND,
            UseCaseError::InternalError(..) => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(json!({"message": self.to_string()}))
    }
}

impl From<diesel::result::Error> for UseCaseError {
    fn from(err: diesel::result::Error) -> Self {
        use diesel::result::Error;
        match err {
            Error::NotFound => Self::ResourceNotFound(err.to_string()),
            _ => Self::InternalError(err.to_string()),
        }
    }
}

impl From<r2d2::Error> for UseCaseError {
    fn from(err: r2d2::Error) -> Self {
        Self::InternalError(format!("{:?}", err))
    }
}

impl From<anyhow::Error> for UseCaseError {
    fn from(err: anyhow::Error) -> Self {
        Self::InternalError(format!("{:?}", err))
    }
}
