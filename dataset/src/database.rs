use anyhow::Ok;
use diesel::{Connection, Insertable, QueryDsl, RunQueryDsl, SqliteConnection};
use discover_hollywood_models::{schema, Movie, Rating, Tag};

use crate::consts::sqlite_file_path;

pub(crate) fn load_dataset_to_sqlite(
    movies: Vec<Movie>,
    ratings: Vec<Rating>,
    tags: Vec<Tag>,
) -> anyhow::Result<()> {
    let mut conn = SqliteConnection::establish(sqlite_file_path().to_str().unwrap())?;
    diesel_migrations::run_pending_migrations(&mut conn)?;

    movies
        .insert_into(schema::movies::table)
        .execute(&mut conn)?;
    ratings
        .insert_into(schema::ratings::table)
        .execute(&mut conn)?;
    tags.insert_into(schema::tags::table).execute(&mut conn)?;

    Ok(())
}

macro_rules! check_table_count {
    ($table_name:ident, $target_count:expr, $conn:expr) => {
        let count = discover_hollywood_models::schema::$table_name::table
            .count()
            .first::<i64>($conn)?;
        anyhow::ensure!(
            count == $target_count,
            "Table {} should have {} rows. (Actual count: {})",
            stringify!($table_name),
            $target_count,
            count
        )
    };
}

pub(crate) fn assert_db_contents() -> anyhow::Result<()> {
    let mut conn = SqliteConnection::establish(sqlite_file_path().to_str().unwrap())?;

    check_table_count!(movies, 9742, &mut conn);
    check_table_count!(ratings, 100836, &mut conn);
    check_table_count!(tags, 3683, &mut conn);

    Ok(())
}
