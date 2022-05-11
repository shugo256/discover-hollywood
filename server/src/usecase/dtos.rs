use derive_more::From;
use discover_hollywood_models::Movie;
use serde::{Deserialize, Deserializer, Serialize};

/// Query struct to search for [`Movie`].
#[derive(Deserialize)]
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

#[derive(Serialize, From)]
pub struct SearchResponse {
    pub movies: Vec<Movie>,
}
