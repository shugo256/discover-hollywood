use std::{fs::File, io, path::PathBuf};

use futures::TryFutureExt;
use zip::ZipArchive;

use crate::consts::{data_dir_path, zip_file_path, DATASET_URL, RESOURCES_DIR_NAME};

pub(crate) async fn download_movielens() -> anyhow::Result<()> {
    let bytes = reqwest::get(DATASET_URL)
        .and_then(|resp| async { resp.bytes().await })
        .await?;

    let mut zip_file = File::create(zip_file_path())?;
    io::copy(&mut io::Cursor::new(bytes), &mut zip_file)?;

    Ok(())
}

pub(crate) async fn unzip_movielens() -> anyhow::Result<()> {
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
