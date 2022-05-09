use crate::schema::*;
use diesel::{Insertable, Queryable};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Queryable, Insertable)]
pub struct Movie {
    #[serde(rename(deserialize = "movieId"))]
    pub id: String,
    pub title: String,
    pub genres: String,
}

#[derive(Clone, Debug, Deserialize, Queryable, Insertable)]
pub struct Rating {
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "movieId"))]
    pub movie_id: String,
    pub rating: f64,
    pub timestamp: i32,
}

#[derive(Clone, Debug, Deserialize, Queryable, Insertable)]
pub struct Tag {
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "movieId"))]
    pub movie_id: String,
    #[serde(rename(deserialize = "tag"))]
    pub name: String,
    pub timestamp: i32,
}

#[derive(Clone, Debug, Deserialize, Queryable, Insertable)]
pub struct Link {
    #[serde(rename(deserialize = "movieId"))]
    pub movie_id: String,
    #[serde(rename(deserialize = "imdbId"))]
    pub imdb_id: String,
    #[serde(rename(deserialize = "tmdbId"))]
    pub tmdb_id: String,
}
