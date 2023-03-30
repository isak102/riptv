use super::file_io;
use std::{error::Error, path::PathBuf};

// TODO: keep track of latest update time

pub async fn update(data_directory: &PathBuf, url: Option<String>) -> Result<(), Box<dyn Error>> {
    let playlist = file_io::install_playlist::install(data_directory, url)
        .await
        .unwrap();

    file_io::extract_channels::extract_from_playlist(&playlist, &data_directory)?;

    Ok(())
}
