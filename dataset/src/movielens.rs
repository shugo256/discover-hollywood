/// Structs to deserialize the data contained in the dataset CSV files.
mod csv_rows;

use std::{fs::File, io, path::PathBuf};

use discover_hollywood_models::{Movie, Rating, Tag};
use futures::TryFutureExt;
use itertools::{izip, Itertools};
use serde::Deserialize;
use zip::ZipArchive;

use crate::consts::{data_dir_path, zip_file_path, DATASET_URL, RESOURCES_DIR_NAME};

use self::csv_rows::{LinkRow, MovieLinkRow, MovieRow, RatingRow, TagRow};

async fn download_movielens() -> anyhow::Result<()> {
    let bytes = reqwest::get(DATASET_URL)
        .and_then(|resp| async { resp.bytes().await })
        .await?;

    let mut zip_file = File::create(zip_file_path())?;
    io::copy(&mut io::Cursor::new(bytes), &mut zip_file)?;

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

fn load_csv_into_vec<T: for<'de> Deserialize<'de>>(path: PathBuf) -> anyhow::Result<Vec<T>> {
    let mut reader = csv::Reader::from_path(path)?;
    let models: Vec<T> = reader.deserialize::<T>().into_iter().try_collect()?;
    Ok(models)
}

pub(crate) async fn load_dataset_as_models() -> anyhow::Result<(Vec<Movie>, Vec<Rating>, Vec<Tag>)>
{
    download_movielens().await?;
    unzip_movielens().await?;

    let movie_rows_iter = load_csv_into_vec::<MovieRow>(data_dir_path().join("movies.csv"))?
        .into_iter()
        .sorted_by_key(|m| m.id.clone());
    let link_rows_iter = load_csv_into_vec::<LinkRow>(data_dir_path().join("links.csv"))?
        .into_iter()
        .sorted_by_key(|l| l.movie_id.clone());

    let movies = izip!(movie_rows_iter, link_rows_iter)
        .map_into::<MovieLinkRow>()
        .map_into::<Movie>()
        .collect_vec();

    let ratings = load_csv_into_vec::<RatingRow>(data_dir_path().join("ratings.csv"))?
        .into_iter()
        .map_into::<Rating>()
        .collect_vec();

    let tags = load_csv_into_vec::<TagRow>(data_dir_path().join("tags.csv"))?
        .into_iter()
        .map_into::<Tag>()
        .collect_vec();

    Ok((movies, ratings, tags))
}
