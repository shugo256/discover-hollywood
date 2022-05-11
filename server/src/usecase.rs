pub(crate) mod dtos;
pub(crate) mod error;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use discover_hollywood_models::{
    schema::movies::{self, title},
    Movie,
};

use self::dtos::SearchResponse;
use self::{dtos::SearchQuery, error::UseCaseResult};

type SqliteConnectionPool = Pool<ConnectionManager<SqliteConnection>>;

/// Main logic struct of the app.
///
/// Responsible for converting request dtos into response dtos.
pub struct UseCase {
    pool: SqliteConnectionPool,
}

impl UseCase {
    /// Initialize a Connection Pool to the DB and build a new UseCase instance.
    pub fn new() -> anyhow::Result<Self> {
        let manager = ConnectionManager::new("./resources/movielens.db");
        let pool = Pool::builder().max_size(16).build(manager)?;
        Ok(Self { pool })
    }

    /// Get a [`Movie`] by id.
    ///
    /// Called by [`crate::handlers::get_movie`].
    pub(crate) fn get_movie(&self, movie_id: &str) -> UseCaseResult<Movie> {
        let conn = self.pool.get()?;
        let movie = movies::table.find(movie_id).first(&conn)?;
        Ok(movie)
    }

    /// Get a list of [`Movie`]s that matches the [`SearchQuery`].
    ///
    /// Called by [`crate::handlers::search_movie`].
    pub(crate) fn search_movie(&self, query: SearchQuery) -> UseCaseResult<SearchResponse> {
        let conn = self.pool.get()?;
        let sql_like_query = format!("%{}%", query.text.join("%"));
        println!("{}", sql_like_query);
        let results = movies::table
            .filter(title.like(sql_like_query))
            .order(title.asc())
            .load::<Movie>(&conn)?;
        Ok(results.into())
    }
}
