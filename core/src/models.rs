use serde::{Deserialize, Serialize};

#[cfg(feature = "adapters")]
use {
    crate::schema::*,
    diesel::{Insertable, Queryable},
};

/// Domain model of movie information
///
/// Corresponds to [`movies`] table.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "adapters", derive(Queryable, Insertable))]
#[cfg_attr(feature = "adapters", table_name = "movies")]
pub struct Movie {
    pub id: String,
    pub title: String,
    /// Pipe-separated list of generes.
    pub genres: String,
    /// Identifier used by <http://www.imdb.com>.
    pub imdb_id: String,
    /// Identifier used by <https://www.themoviedb.org>.
    pub tmdb_id: String,
}

/// Domain model of rating of one movie by one user.
///
/// Corresponds to [`ratings`] table.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "adapters", derive(Queryable, Insertable))]
#[cfg_attr(feature = "adapters", table_name = "ratings")]
pub struct Rating {
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "movieId"))]
    pub movie_id: String,
    /// Ratings are made on a 5-star scale, with half-star increments (0.5 stars - 5.0 stars).
    pub rating: f64,
    /// Seconds since midnight Coordinated Universal Time (UTC) of January 1, 1970.
    pub timestamp: i32,
}

/// Domain model of tag applied to one movie by one user.
///
/// Corresponds to [`tags`] table.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "adapters", derive(Queryable, Insertable))]
#[cfg_attr(feature = "adapters", table_name = "tags")]
pub struct Tag {
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "movieId"))]
    pub movie_id: String,
    pub tag: String,
    /// Seconds since midnight Coordinated Universal Time (UTC) of January 1, 1970.
    pub timestamp: i32,
}
