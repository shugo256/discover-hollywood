use derive_more::From;
use discover_hollywood_core::models::{Movie, Rating, Tag};
use serde::{Deserialize, Serialize};

/// Movie Information
///
/// Corresponds to each row of movies.csv in the [Movielens Dataset](https://grouplens.org/datasets/movielens/).
/// Only movies with at least one rating or tag are included in the dataset. These movie ids are consistent with those used on the MovieLens web site (e.g., id `1` corresponds to the URL <https://movielens.org/movies/1>).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct MovieRow {
    #[serde(rename(deserialize = "movieId"))]
    pub id: String,
    pub title: String,
    /// Pipe-separated list of generes.
    pub genres: String,
}

/// Rating of one movie by one user.
///
/// Corresponds to each row of rating.csv in the [Movielens Dataset](https://grouplens.org/datasets/movielens/).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct RatingRow {
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "movieId"))]
    pub movie_id: String,
    /// Ratings are made on a 5-star scale, with half-star increments (0.5 stars - 5.0 stars).
    pub rating: f64,
    /// Seconds since midnight Coordinated Universal Time (UTC) of January 1, 1970.
    pub timestamp: i32,
}

impl From<RatingRow> for Rating {
    fn from(row: RatingRow) -> Self {
        Self {
            user_id: row.user_id,
            movie_id: row.movie_id,
            rating: row.rating,
            timestamp: row.timestamp,
        }
    }
}

/// Tag applied to one movie by one user.
///
/// Corresponds to each row of tags.csv in the [Movielens Dataset](https://grouplens.org/datasets/movielens/).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct TagRow {
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "movieId"))]
    pub movie_id: String,
    pub tag: String,
    /// Seconds since midnight Coordinated Universal Time (UTC) of January 1, 1970.
    pub timestamp: i32,
}

impl From<TagRow> for Tag {
    fn from(row: TagRow) -> Self {
        Self {
            user_id: row.user_id,
            movie_id: row.movie_id,
            tag: row.tag,
            timestamp: row.timestamp,
        }
    }
}

/// Identifiers that can be used to link to other sources of movie data
///
/// Corresponds to each row of links.csv in the [Movielens Dataset](https://grouplens.org/datasets/movielens/).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct LinkRow {
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

#[derive(Clone, Debug, From)]
pub(super) struct MovieLinkRow(MovieRow, LinkRow);

impl From<MovieLinkRow> for Movie {
    fn from(MovieLinkRow(mrow, lrow): MovieLinkRow) -> Self {
        Self {
            id: mrow.id,
            title: mrow.title,
            genres: mrow.genres,
            imdb_id: lrow.imdb_id,
            tmdb_id: lrow.tmdb_id,
        }
    }
}
