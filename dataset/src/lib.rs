pub(crate) mod consts;
pub(crate) mod database;
pub(crate) mod movielens;

use consts::{data_dir_path, sqlite_file_path, zip_file_path};
use database::{assert_db_contents, load_dataset_to_sqlite};
use movielens::load_dataset_as_models;

fn clear_resources_dir() -> anyhow::Result<()> {
    if zip_file_path().exists() {
        std::fs::remove_file(zip_file_path())?;
    }
    if data_dir_path().exists() {
        std::fs::remove_dir_all(data_dir_path())?;
    }
    if sqlite_file_path().exists() {
        std::fs::remove_file(sqlite_file_path())?;
    }
    Ok(())
}

async fn load_csv_to_db() -> anyhow::Result<()> {
    let (movies, ratings, tags) = load_dataset_as_models().await?;
    load_dataset_to_sqlite(movies, ratings, tags)?;
    assert_db_contents()?;
    Ok(())
}

/// Download the Movielens dataset from the Web and insert them into the Sqlite file.
pub async fn prepare() -> anyhow::Result<()> {
    if assert_db_contents().is_err() {
        clear_resources_dir()?;
        let res = load_csv_to_db().await;
        if res.is_err() {
            clear_resources_dir()?;
        }
        return res;
    }
    Ok(())
}
