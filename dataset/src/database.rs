use anyhow::Ok;
use diesel::{Connection, Insertable, QueryDsl, RunQueryDsl, SqliteConnection};
use discover_hollywood_models::{Link, Movie, RatingEntry, TagEntry};
use itertools::Itertools;

use crate::consts::{data_dir_path, sqlite_file_path};

macro_rules! insert_csv_into_table {
    ($table_name:ident, $model_type:ty, $conn:expr) => {
        let csv_name = format!("{}.csv", stringify!($table_name));
        let mut reader = csv::Reader::from_path(data_dir_path().join(csv_name))?;
        let models: Vec<$model_type> = reader
            .deserialize::<$model_type>()
            .into_iter()
            .try_collect()?;
        models
            .insert_into(discover_hollywood_models::schema::$table_name::table)
            .execute($conn)?;
    };
}

pub(crate) fn load_dataset_to_sqlite() -> anyhow::Result<()> {
    let mut conn = SqliteConnection::establish(sqlite_file_path().to_str().unwrap())?;
    diesel_migrations::run_pending_migrations(&mut conn)?;

    insert_csv_into_table!(movies, Movie, &mut conn);
    insert_csv_into_table!(ratings, RatingEntry, &mut conn);
    insert_csv_into_table!(tags, TagEntry, &mut conn);
    insert_csv_into_table!(links, Link, &mut conn);

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
    check_table_count!(links, 9742, &mut conn);

    Ok(())
}
