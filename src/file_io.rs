use super::consts::DATA_DIRECTORY;
use reqwest::Url;
use std::io::Write;
use std::{error::Error, fs::File};

pub mod extract_channels;
pub mod install_playlist;

pub fn set_url(url: Url) -> Result<(), Box<dyn Error>> {
    let mut url_file = File::create(&DATA_DIRECTORY.join("url.txt"))?;
    url_file.write_all(url.as_str().as_bytes())?;

    Ok(())
}
