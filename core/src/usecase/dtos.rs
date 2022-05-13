use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::models::Movie;

#[cfg(feature = "adapters")]
use diesel::Queryable;

/// Struct that contains verbose information on a [`Movie`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "adapters", derive(Queryable))]
pub struct MovieInfo {
    /// Basic information on the movie.
    ///
    /// This field will be flattend by [`serde`] when its converted into json body.
    /// See <https://serde.rs/attr-flatten.html> for details.
    #[serde(flatten)]
    pub movie: Movie,

    /// Average rating by users.
    pub rating: f64,

    /// The number of users who rated this movie.
    pub rated_user_num: i32,
}

/// Query struct to search for [`Movie`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Query text for title search.
    ///
    /// A space-separated list of this field is used to perform an ambiguous search to the movie titles.
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, From)]
pub struct SearchResponse {
    pub movies: Vec<MovieInfo>,
}
