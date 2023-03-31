use std::error::Error;

use super::file_io;

mod log;

pub async fn update(url: Option<String>) -> Result<(), Box<dyn Error>> {
    let playlist = file_io::install_playlist::install(url).await.unwrap();

    file_io::extract_channels::extract_from_playlist(&playlist)?;

    log::log().unwrap();
    Ok(())
}
