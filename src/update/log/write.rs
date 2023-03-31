use super::LOG_FILE;
use std::io::Write;
use std::{fs::OpenOptions, time::SystemTime};

pub fn log_date() -> std::io::Result<()> {
    // Open the file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE.as_path())?;

    // Get the current date and time
    let now = SystemTime::now();
    let datetime = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Write the date and time to the file
    writeln!(file, "{}", datetime)?;

    Ok(())
}
