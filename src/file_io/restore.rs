use crate::consts::DATA_DIRECTORY;
use crate::file_io::install_playlist::PLAYLIST_PATH;
use std::io::Result as IoResult;
use std::time::UNIX_EPOCH;
use std::{error::Error, path::PathBuf};
use std::{fs, io};

fn get_latest_backup() -> IoResult<PathBuf> {
    let backup_dir = DATA_DIRECTORY.join("backup");

    fs::read_dir(backup_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let metadata = entry.metadata().ok()?;
            if metadata.is_file() {
                let modified_time = metadata.modified().ok()?;
                Some((entry.path(), modified_time))
            } else {
                None
            }
        })
        .max_by_key(|&(_, modified_time)| {
            modified_time.duration_since(UNIX_EPOCH).unwrap_or_default()
        })
        .map(|(path, _)| path)
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "No files found"))
}

pub fn restore() -> Result<(), Box<dyn Error>> { // TODO: update the log when restoring
    let latest_backup = get_latest_backup().expect("Error when getting the latest backup");

    super::extract_channels::extract_from_playlist(&latest_backup)
        .expect("Error when extracting channels from the old playlist");

    fs::rename(&latest_backup, PLAYLIST_PATH.as_path())
        .expect("Error when restoring the latest backup");

    eprintln!("Latest backup restored sucessfully.");
    Ok(())
}
