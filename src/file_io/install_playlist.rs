use core::cmp::min;
use directories::BaseDirs;
use futures_util::StreamExt;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use lazy_static::lazy_static;
use reqwest::{self, Client};
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::Path;
use std::{error::Error, path::PathBuf, result::Result};

use crate::consts::DATA_DIRECTORY;

lazy_static! {
    pub static ref PLAYLIST_PATH: PathBuf = DATA_DIRECTORY.join("playlist.m3u");
}

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

fn create_file_with_number(name: &str) -> PathBuf {
    let mut path = PathBuf::from(name);
    let mut index = 1;
    while path.exists() {
        let new_name = format!("{}-{}.m3u", &name[..name.len() - 4], index);
        path = PathBuf::from(new_name);
        index += 1;
    }

    fs::File::create(&path).expect("Failed to create backup playlist file");
    return path;
}

fn keep_latest_files(folder_path: &Path, files_to_keep: usize) -> std::io::Result<()> {
    let mut entries: Vec<DirEntry> = fs::read_dir(folder_path)?
        .map(|res| res.map(|e| e))
        .collect::<Result<Vec<DirEntry>, std::io::Error>>()?;
    entries.sort_by(|a, b| {
        b.metadata()
            .unwrap()
            .modified()
            .unwrap()
            .cmp(&a.metadata().unwrap().modified().unwrap())
    });
    for entry in entries.iter().skip(files_to_keep) {
        if let Some(file_type) = entry.file_type().ok() {
            if file_type.is_file() {
                fs::remove_file(entry.path())?;
            } else if file_type.is_dir() {
                continue;
            }
        }
    }
    Ok(())
}

fn backup_old_playlist() -> IoResult<()> {
    const BACKUPS_TO_KEEP: usize = 5;

    let backup_dir = DATA_DIRECTORY.join("backup");
    std::fs::create_dir_all(&backup_dir).expect("Failure when creating backup dir");

    let backup_playlist_file =
        create_file_with_number(backup_dir.join("playlist.m3u").to_str().unwrap());

    fs::rename(PLAYLIST_PATH.as_path(), backup_playlist_file)
        .expect("Failure when moving playlist to backup folder");

    keep_latest_files(backup_dir.as_path(), BACKUPS_TO_KEEP)
        .expect("Failure removing old playlists");

    Ok(())
}

pub async fn install(url: Option<String>) -> Result<PathBuf, Box<dyn Error>> {
    fn get_url(url_file: PathBuf) -> IoResult<String> {
        fs::read_to_string(url_file)
    }

    let playlist_url = match url {
        Some(str) => str,
        None => get_url(DATA_DIRECTORY.join("url.txt"))
            .unwrap_or_else(|_| {
                eprintln!(
                    "URL file {} does not exist. use --url to manually enter a URL, or create URL file by running set-url",
                    DATA_DIRECTORY.join("url.txt").to_str().unwrap()
                );
                std::process::exit(1);
            })
            .trim()
            .to_string(),
    };

    let temp_save_path = BaseDirs::new()
        .unwrap()
        .cache_dir()
        .to_owned()
        .join("riptv")
        .join("tmp342892"); // TODO: randomly generate this
    fs::create_dir_all(temp_save_path.parent().unwrap()).unwrap();

    let client = reqwest::Client::new();

    download_file(&client, &playlist_url, temp_save_path.to_str().unwrap())
        .await
        .unwrap();

    backup_old_playlist().expect("Failure when backing up old playlist");
    fs::copy(&temp_save_path, PLAYLIST_PATH.as_path())
        .expect("Failure copying tmp file to real file");
    fs::remove_file(&temp_save_path).expect("Removing the tmp file failed");
    eprintln!("Sucessfully installed playlist.");

    Ok(PLAYLIST_PATH.clone())
}
