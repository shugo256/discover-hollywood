use derive_more::From;
use diesel::Queryable;
use discover_hollywood_core::models::Movie;
use serde::{Deserialize, Deserializer, Serialize};

// TODO: これmodelsに移動しないと、frontから使えなそう
/// Struct that contains verbose information on a [`Movie`].
#[derive(Clone, Debug, Serialize, Queryable)]
pub struct MovieInfo {
    /// Basic information on the movie.
    ///
    /// This field will be flattend by [`serde`] when its converted into json body.
    /// See https://serde.rs/attr-flatten.html for details.
    #[serde(flatten)]
    movie: Movie,

    /// Average rating by users.
    rating: f64,

    /// The number of users who rated this movie.
    rated_user_num: i32,
}

/// Query struct to search for [`Movie`].
#[derive(Clone, Debug, Deserialize)]
pub struct SearchQuery {
    /// Query text for fuzzy string searching.
    ///
    /// This field is converted from a plus-separeted list in the query param
    /// using [`plus_separeted_list_to_vec`].
    #[serde(deserialize_with = "plus_separeted_list_to_vec")]
    pub text: Vec<String>,
}

fn plus_separeted_list_to_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let list: String = Deserialize::deserialize(deserializer)?;
    Ok(list.split('+').map(String::from).collect())
}

#[derive(Clone, Debug, Serialize, From)]
pub struct SearchResponse {
    pub movies: Vec<MovieInfo>,
}
