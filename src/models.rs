use crate::schema::*;
use diesel::{Insertable, Queryable};
use serde::Deserialize;

/// Movie Information
///
/// Corresponds to each row of movies.csv in the [Movielens Dataset](https://grouplens.org/datasets/movielens/).
/// Only movies with at least one rating or tag are included in the dataset. These movie ids are consistent with those used on the MovieLens web site (e.g., id `1` corresponds to the URL <https://movielens.org/movies/1>).
#[derive(Clone, Debug, Deserialize, Queryable, Insertable)]
pub struct Movie {
    #[serde(rename(deserialize = "movieId"))]
    pub id: String,
    pub title: String,
    /// Pipe-separated list of generes.
    pub genres: String,
}

/// Rating of one movie by one user.
///
/// Corresponds to each row of rating.csv in the [Movielens Dataset](https://grouplens.org/datasets/movielens/).
#[derive(Clone, Debug, Deserialize, Queryable, Insertable)]
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

/// Tag applied to one movie by one user.
///
/// Corresponds to each row of tags.csv in the [Movielens Dataset](https://grouplens.org/datasets/movielens/).
#[derive(Clone, Debug, Deserialize, Queryable, Insertable)]
pub struct Tag {
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "movieId"))]
    pub movie_id: String,
    #[serde(rename(deserialize = "tag"))]
    pub name: String,
    /// Seconds since midnight Coordinated Universal Time (UTC) of January 1, 1970.
    pub timestamp: i32,
}

/// Identifiers that can be used to link to other sources of movie data
///
/// Corresponds to each row of links.csv in the [Movielens Dataset](https://grouplens.org/datasets/movielens/).
#[derive(Clone, Debug, Deserialize, Queryable, Insertable)]
pub struct Link {
    /// Identifier used by <https://movielens.org>.
    #[serde(rename(deserialize = "movieId"))]
    pub movie_id: String,
    /// Identifier used by <http://www.imdb.com>.
    #[serde(rename(deserialize = "imdbId"))]
    pub imdb_id: String,
    /// Identifier used by <https://www.themoviedb.org>.
    #[serde(rename(deserialize = "tmdbId"))]
    pub tmdb_id: String,
}
