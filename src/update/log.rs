use crate::consts::DATA_DIRECTORY;
use lazy_static::lazy_static;
use std::{fs, path::PathBuf};

lazy_static! {
    static ref LOG_FILE: PathBuf = {
        let log_dir = &DATA_DIRECTORY.join("/log");
        fs::create_dir_all(&log_dir).expect("Failure when creating log dir");

        log_dir.join("log.txt")
    };
}

pub mod read;
pub mod write;
