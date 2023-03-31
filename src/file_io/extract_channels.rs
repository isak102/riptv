use super::super::consts::DATA_DIRECTORY;
use std::io::Result as IoResult;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::{error::Error, fs::File, path::PathBuf};

enum StreamExtension {
    MP4,
    M3U8,
    MKV,
    Other,
}

impl From<&str> for StreamExtension {
    fn from(url: &str) -> StreamExtension {
        let ext = url.split(".").last().expect("URL should contain dot \".\"");
        match ext {
            "mp4" => StreamExtension::MP4,
            "m3u8" => StreamExtension::M3U8,
            "mkv" => StreamExtension::MKV,
            _ => StreamExtension::Other,
        }
    }
}

fn write_entry(title: String, url: String, file: &mut File) -> IoResult<()> {
    let line = format!("{title}§{url}\n");
    file.write_all(line.as_bytes())
}

fn _remove_old_files() -> Result<(), String> {
    todo!()
}

pub fn extract_from_playlist(playlist: &PathBuf) -> Result<(), Box<dyn Error>> {
    let playlist_handle = File::open(playlist)?;
    let playlist_reader = BufReader::new(playlist_handle);

    let mut live_entries = File::create(&DATA_DIRECTORY.join("live.txt"))?;
    let mut vod_entries = File::create(&DATA_DIRECTORY.join("vod.txt"))?;

    let mut title;
    let mut url;
    let mut lines = playlist_reader.lines().skip(1);
    eprintln!("Extracting channels...");
    while let Some(title_line) = lines.next() {
        if let Some(url_line) = lines.next() {
            let title_line = title_line.unwrap();
            title = title_line.split(":-1,").last().unwrap();
            url = url_line.expect("Number of lines should not be odd");

            match StreamExtension::from(url.as_str()) {
                StreamExtension::M3U8 => write_entry(title.to_string(), url, &mut live_entries)?,
                StreamExtension::MP4 | StreamExtension::MKV => {
                    write_entry(title.to_string(), url, &mut vod_entries)?
                }
                StreamExtension::Other => continue,
            }
        }
    }

    Ok(())
}
