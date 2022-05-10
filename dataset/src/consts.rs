use std::path::PathBuf;

pub(crate) const DATASET_URL: &str =
    "http://files.grouplens.org/datasets/movielens/ml-latest-small.zip";
pub(crate) const RESOURCES_DIR_NAME: &str = "resources";
pub(crate) const DATA_DIR_NAME: &str = "data";
pub(crate) const ZIP_FILE_NAME: &str = "movielens.zip";
pub(crate) const SQLITE_FILE_NAME: &str = "movielens.db";

pub(crate) fn zip_file_path() -> PathBuf {
    PathBuf::from_iter([RESOURCES_DIR_NAME, ZIP_FILE_NAME])
}

pub(crate) fn data_dir_path() -> PathBuf {
    PathBuf::from_iter([RESOURCES_DIR_NAME, DATA_DIR_NAME])
}

pub(crate) fn sqlite_file_path() -> PathBuf {
    PathBuf::from_iter([RESOURCES_DIR_NAME, SQLITE_FILE_NAME])
}
