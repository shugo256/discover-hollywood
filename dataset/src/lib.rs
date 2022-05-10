use std::fs::File;
use std::io::{self, Cursor};
use std::path::PathBuf;

use anyhow::Ok;
use diesel::{Connection, Insertable, QueryDsl, RunQueryDsl, SqliteConnection};
use discover_hollywood_models::{Link, Movie, RatingEntry, TagEntry};
use futures::TryFutureExt;
use itertools::Itertools;
use zip::ZipArchive;

const DATASET_URL: &str = "http://files.grouplens.org/datasets/movielens/ml-latest-small.zip";
const RESOURCES_DIR_NAME: &str = "resources";
const DATA_DIR_NAME: &str = "data";
const ZIP_FILE_NAME: &str = "movielens.zip";
const SQLITE_FILE_NAME: &str = "movielens.db";

fn zip_file_path() -> PathBuf {
    PathBuf::from_iter([RESOURCES_DIR_NAME, ZIP_FILE_NAME])
}

fn data_dir_path() -> PathBuf {
    PathBuf::from_iter([RESOURCES_DIR_NAME, DATA_DIR_NAME])
}

fn sqlite_file_path() -> PathBuf {
    PathBuf::from_iter([RESOURCES_DIR_NAME, SQLITE_FILE_NAME])
}

async fn download_movielens() -> anyhow::Result<()> {
    let bytes = reqwest::get(DATASET_URL)
        .and_then(|resp| async { resp.bytes().await })
        .await?;

    let mut zip_file = File::create(zip_file_path())?;
    io::copy(&mut Cursor::new(bytes), &mut zip_file)?;

    Ok(())
}

async fn unzip_movielens() -> anyhow::Result<()> {
    let zip_file = File::open(zip_file_path())?;

    let mut archive = ZipArchive::new(zip_file)?;
    archive.extract(RESOURCES_DIR_NAME)?;

    let dir_name = archive.by_index(0).unwrap().name().to_owned();
    std::fs::rename(
        PathBuf::from_iter([RESOURCES_DIR_NAME, &dir_name]),
        data_dir_path(),
    )?;

    Ok(())
}

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

fn load_dataset_to_sqlite() -> anyhow::Result<()> {
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

fn assert_db_contents() -> anyhow::Result<()> {
    let mut conn = SqliteConnection::establish(sqlite_file_path().to_str().unwrap())?;

    check_table_count!(movies, 9742, &mut conn);
    check_table_count!(ratings, 100836, &mut conn);
    check_table_count!(tags, 3683, &mut conn);
    check_table_count!(links, 9742, &mut conn);

    Ok(())
}

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

/// Download the Movielens dataset from the Web and insert them into the Sqlite file.
pub async fn prepare() -> anyhow::Result<()> {
    let mut res = Ok(());
    if !zip_file_path().exists() {
        res = res.and(download_movielens().await);
    }
    if !data_dir_path().exists() {
        res = res.and(unzip_movielens().await);
    }
    if !sqlite_file_path().exists() {
        res = res.and(load_dataset_to_sqlite());
    }
    res = res.and(assert_db_contents());

    if res.is_err() {
        clear_resources_dir()?;
    }
    res
}
