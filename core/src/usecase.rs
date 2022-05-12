/// Data Transfer Objects for HTTP requests/responses.
pub mod dtos;

/// Error & Result type definition for the usecase.

#[cfg(feature = "adapters")]
pub(crate) mod error;

#[cfg(feature = "adapters")]
use {
    self::{
        dtos::{MovieInfo, SearchQuery, SearchResponse},
        error::UseCaseResult,
    },
    crate::schema::movies,
    diesel::{
        prelude::*,
        r2d2::{ConnectionManager, Pool},
    },
};

#[cfg(feature = "adapters")]
type SqliteConnectionPool = Pool<ConnectionManager<SqliteConnection>>;

/// Main logic struct of the app.
///
/// Responsible for converting request dtos into response dtos.
#[cfg(feature = "adapters")]
pub struct UseCase {
    pool: SqliteConnectionPool,
}

#[cfg(feature = "adapters")]
impl UseCase {
    /// Initialize a Connection Pool to the DB and build a new UseCase instance.
    pub fn new() -> anyhow::Result<Self> {
        let manager = ConnectionManager::new("./resources/movielens.db");
        let pool = Pool::builder().max_size(16).build(manager)?;
        Ok(Self { pool })
    }

    /// Get a [`Movie`] by id.
    ///
    /// Called by [`discover_hollywood_server::handlers::get_movie`].
    pub fn get_movie(&self, movie_id: &str) -> UseCaseResult<MovieInfo> {
        let conn = self.pool.get()?;
        let movie = diesel_queries::rating_joined_query()
            .filter(movies::id.eq(movie_id))
            .first(&conn)?;
        Ok(movie)
    }

    /// Get a list of [`Movie`]s that matches the [`SearchQuery`].
    ///
    /// Called by [`discover_hollywood_server::handlers::search_movie`].
    pub fn search_movie(&self, query: SearchQuery) -> UseCaseResult<SearchResponse> {
        let conn = self.pool.get()?;
        let sql_like_query = format!("%{}%", query.text.replace(" ", "%"));

        let results = diesel_queries::rating_joined_query()
            .filter(movies::title.like(sql_like_query))
            .order(movies::title.asc())
            .load::<MovieInfo>(&conn)?;
        Ok(results.into())
    }
}

/// Collection of complecated Diesel queries.
#[cfg(feature = "adapters")]
mod diesel_queries {
    use crate::schema::{movies, ratings};
    use diesel::backend::Backend;
    use diesel::dsl::sql;
    use diesel::expression::nullable::Nullable;
    use diesel::expression::operators::Eq;
    use diesel::prelude::*;
    use diesel::query_builder::BoxedSelectStatement;
    use diesel::query_source::joins::{Inner, Join, JoinOn};
    use diesel::sql_types::{Double, Integer};

    type RatingJoinedType = (movies::SqlType, Double, Integer);
    type RatingJoinedQueryStatement = JoinOn<
        Join<movies::table, ratings::table, Inner>,
        Eq<Nullable<ratings::movie_id>, Nullable<movies::id>>,
    >;

    /// Generates a query to inner_join [`movies`] and [`ratings`].
    pub(super) fn rating_joined_query<'a, DB: Backend>(
    ) -> BoxedSelectStatement<'a, RatingJoinedType, RatingJoinedQueryStatement, DB> {
        movies::table
            .inner_join(ratings::table)
            .group_by(movies::id)
            .select((
                movies::all_columns,
                // Reference: https://github.com/diesel-rs/diesel/issues/210
                sql::<Double>("avg(ratings.rating) AS rating"),
                sql::<Integer>("count(ratings.rating) AS rated_user_num"),
            ))
            .into_boxed()
    }
}
