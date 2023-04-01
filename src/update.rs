use super::file_io;
use std::error::Error;

mod log;

pub async fn update(url: Option<String>, history: bool) -> Result<(), Box<dyn Error>> {
    if history {
        log::read::print_history().unwrap();
        return Ok(());
    }

    let playlist = file_io::install_playlist::install(url).await.unwrap();

    file_io::extract_channels::extract_from_playlist(&playlist)?;

    log::write::log_date().unwrap();
    
    eprintln!("Update complete.");
    Ok(())
}
