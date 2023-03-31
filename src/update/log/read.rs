use chrono::{DateTime, Local, TimeZone};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use super::LOG_FILE;

fn get_latest_update() -> Result<u64, Box<dyn Error>> {
    let file = File::open(LOG_FILE.as_path())?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

    let last_line = lines.last().unwrap_or_else(|| {
        eprintln!("Error getting latest update");
        std::process::exit(1);
    });

    match last_line.parse::<u64>() {
        Ok(n) => Ok(n),
        Err(e) => Err(Box::new(e)),
    }
}

fn convert_to_local(unix_epoch: u64) -> DateTime<Local> {
    Local.timestamp_opt(unix_epoch as i64, 0).unwrap()
}

pub fn print_history() -> Result<(), Box<dyn Error>> {
    let latest_update = get_latest_update()?;
    eprintln!(
        "Latest update: {}",
        convert_to_local(latest_update).format("%d/%m/%Y %H:%M")
    );
    Ok(())
}
