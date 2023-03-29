use reqwest;
use std::fs;
use std::fs::File;

use std::{error::Error, path::PathBuf, result::Result};

use std::io::{Error as IoError, ErrorKind, Result as IoResult, Write};

pub async fn install(dir: &PathBuf, url: Option<String>) -> Result<PathBuf, Box<dyn Error>> {
    fn get_url(url_file: PathBuf) -> IoResult<String> {
        fs::read_to_string(url_file)
    }

    // TODO: backup old playlist

    if !dir.exists() {
        return Err(Box::new(IoError::new(
            ErrorKind::NotFound,
            dir.to_string_lossy(),
        )));
    }

    let playlist_url = match url {
        Some(str) => str,
        None => get_url(dir.join("url.txt"))?
            .strip_suffix("\n")
            .expect("String should have a newline.")
            .to_string(),
    };
    let save_path = dir.join("playlist.m3u");

    let mut file = File::create(&save_path)?;

    let client = reqwest::Client::new();

    // TODO: add progress bar
    println!("Installing playlist from {}...", playlist_url);
    let response_text = client.get(playlist_url).send().await?.text().await?;
    println!("Installation done.");

    file.write_all(response_text.as_bytes())?;

    Ok(save_path)
}
