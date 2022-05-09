use std::fs::File;
use std::io::{self, Cursor};
use std::path::PathBuf;

use anyhow::Ok;
use diesel::query_builder::InsertStatement;
use diesel::query_dsl::methods;
use diesel::{Connection, Insertable, RunQueryDsl, SqliteConnection};
use futures::TryFutureExt;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use zip::ZipArchive;

use crate::models::{Link, Movie, Rating, Tag};
use crate::schema;

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

fn insert_csv_into_table<M, T>(
    path: PathBuf,
    table: T,
    conn: &mut SqliteConnection,
) -> anyhow::Result<()>
where
    M: DeserializeOwned,
    Vec<M>: Insertable<T>,
    InsertStatement<T, <Vec<M> as Insertable<T>>::Values>: methods::ExecuteDsl<SqliteConnection>,
{
    let mut reader = csv::Reader::from_path(path)?;
    let models: Vec<M> = reader.deserialize::<M>().into_iter().try_collect()?;
    models.insert_into(table).execute(conn)?;
    Ok(())
}

fn load_dataset_to_sqlite() -> anyhow::Result<()> {
    let mut conn = SqliteConnection::establish(sqlite_file_path().to_str().unwrap())?;
    diesel_migrations::run_pending_migrations(&mut conn)?;

    insert_csv_into_table::<Movie, _>(
        data_dir_path().join("movies.csv"),
        schema::movies::table,
        &mut conn,
    )?;
    insert_csv_into_table::<Rating, _>(
        data_dir_path().join("ratings.csv"),
        schema::ratings::table,
        &mut conn,
    )?;
    insert_csv_into_table::<Tag, _>(
        data_dir_path().join("tags.csv"),
        schema::tags::table,
        &mut conn,
    )?;
    insert_csv_into_table::<Link, _>(
        data_dir_path().join("links.csv"),
        schema::links::table,
        &mut conn,
    )?;

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
        let zip_res = download_movielens().await;
        res = res.and(zip_res);
    }
    if !data_dir_path().exists() {
        let unzip_res = unzip_movielens().await;
        res = res.and(unzip_res);
    }
    if !sqlite_file_path().exists() {
        let sqlite_res = load_dataset_to_sqlite();
        res = res.and(sqlite_res);
    }
    if res.is_err() {
        clear_resources_dir()?;
    }
    res
}
