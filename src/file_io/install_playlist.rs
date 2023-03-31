use core::cmp::min;
use futures_util::StreamExt;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use reqwest::{self, Client};
use std::fs;
use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::{error::Error, path::PathBuf, result::Result};

use crate::consts::DATA_DIRECTORY;

// TODO: remove unwraps

// FOUND THIS FUNCTION HERE: https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    eprint!("Waiting for response from playlist...");
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));

    // download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", url, path));
    return Ok(());
}

pub async fn install(url: Option<String>) -> Result<PathBuf, Box<dyn Error>> {
    fn get_url(url_file: PathBuf) -> IoResult<String> {
        fs::read_to_string(url_file)
    }

    let dir = &DATA_DIRECTORY;
    if !dir.exists() {
        std::fs::create_dir_all(dir.as_path()).unwrap();
        eprintln!("Created {}...", dir.to_str().unwrap());
    }

    let playlist_url = match url {
        Some(str) => str,
        None => get_url(dir.join("url.txt"))
            .unwrap_or_else(|_| {
                eprintln!(
                    "URL file {} does not exist. use --url to manually enter a URL",
                    dir.join("url.txt").to_str().unwrap()
                );
                std::process::exit(1);
            })
            .strip_suffix("\n")
            .expect("String should have a newline.")
            .to_string(),
    };

    let save_path = dir.join("playlist.m3u");
    let client = reqwest::Client::new();

    download_file(&client, &playlist_url, save_path.to_str().unwrap())
        .await
        .unwrap();

    Ok(save_path)
}
