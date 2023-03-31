use super::file_io;
use std::error::Error;

// TODO: keep track of latest update time

pub async fn update(url: Option<String>) -> Result<(), Box<dyn Error>> {
    let playlist = file_io::install_playlist::install(url).await.unwrap();

    file_io::extract_channels::extract_from_playlist(&playlist)?;

    Ok(())
}
