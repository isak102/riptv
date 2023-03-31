use super::file_io;
use std::error::Error;

mod log;

pub async fn update(url: Option<String>) -> Result<(), Box<dyn Error>> {
    let playlist = file_io::install_playlist::install(url).await.unwrap();

    file_io::extract_channels::extract_from_playlist(&playlist)?;

    log::write::log_date().unwrap();
    Ok(())
}
