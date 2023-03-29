use std::io::{BufRead, BufReader};
use std::{error::Error, fs::File, path::PathBuf};

pub fn parse(playlist: &PathBuf, data_directory: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::open(playlist)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().skip(1);
    while let Some(title_line) = lines.next() {
        if let Some(url_line) = lines.next() {
            println!(
                // TODO: move link to file depending on the file extension
                "Channel: {}\nURL: {}\n",
                title_line?.split(":-1,").last().unwrap(),
                url_line?
            );
        }
    }

    Ok(())
}
