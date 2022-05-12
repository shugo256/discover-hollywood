pub(crate) mod dtos;
pub(crate) mod error;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use discover_hollywood_models::schema::movies;

use self::dtos::{MovieInfo, SearchResponse};
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
    pub(crate) fn get_movie(&self, movie_id: &str) -> UseCaseResult<MovieInfo> {
        let conn = self.pool.get()?;
        let movie = diesel_queries::rating_joined_query()
            .filter(movies::id.eq(movie_id))
            .first(&conn)?;
        Ok(movie)
    }

    /// Get a list of [`Movie`]s that matches the [`SearchQuery`].
    ///
    /// Called by [`crate::handlers::search_movie`].
    pub(crate) fn search_movie(&self, query: SearchQuery) -> UseCaseResult<SearchResponse> {
        let conn = self.pool.get()?;
        let sql_like_query = format!("%{}%", query.text.join("%"));

        let results = diesel_queries::rating_joined_query()
            .filter(movies::title.like(sql_like_query))
            .order(movies::title.asc())
            .load::<MovieInfo>(&conn)?;
        Ok(results.into())
    }
}

/// Collection of complecated Diesel queries.
mod diesel_queries {
    use diesel::backend::Backend;
    use diesel::dsl::sql;
    use diesel::expression::nullable::Nullable;
    use diesel::expression::operators::Eq;
    use diesel::prelude::*;
    use diesel::query_builder::BoxedSelectStatement;
    use diesel::query_source::joins::{Inner, Join, JoinOn};
    use diesel::sql_types::{Double, Integer};
    use discover_hollywood_models::schema::{movies, ratings};

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
