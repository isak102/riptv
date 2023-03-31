use crate::consts::DATA_DIRECTORY;
use lazy_static::lazy_static;
use std::{fs, path::PathBuf};

lazy_static! {
    static ref LOG_FILE: PathBuf = {
        let mut output = DATA_DIRECTORY.clone();
        output.push("log");

        fs::create_dir_all(&output).expect("Failure when creating log dir");
        output.push("log.txt");

        output
    };
}

pub mod read;
pub mod write;
