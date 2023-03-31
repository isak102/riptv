use super::super::consts::DATA_DIRECTORY;
use crate::args::StreamType;
use std::io::Result as IoResult;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::{error::Error, fs::File, path::PathBuf};

struct ChannelFiles {
    live_channels: File,
    vod_channels: File,
}
impl ChannelFiles {
    fn open() -> ChannelFiles {
        let live_entries = File::create(&DATA_DIRECTORY.join("live.txt")).unwrap();
        let vod_entries = File::create(&DATA_DIRECTORY.join("vod.txt")).unwrap();

        ChannelFiles {
            live_channels: live_entries,
            vod_channels: vod_entries,
        }
    }

    fn get_file(&self, stream_type: &StreamType) -> Option<&File> {
        match stream_type {
            StreamType::Live => Some(&self.live_channels),
            StreamType::Vod => Some(&self.vod_channels),
            StreamType::Other(_) => None,
        }
    }
}

fn write_entry(title: String, url: String, channel_files: &ChannelFiles) -> IoResult<()> {
    let stream_type = StreamType::from_url(url.as_str());

    let mut file = match channel_files.get_file(&stream_type) {
        Some(f) => f,
        None => return Ok(()),
    };

    let line = format!("{title}§{url}\n");
    file.write_all(line.as_bytes())
}

fn _remove_old_files() -> Result<(), String> {
    todo!()
}

pub fn extract_from_playlist(playlist: &PathBuf) -> Result<(), Box<dyn Error>> {
    let playlist_handle = File::open(playlist)?;
    let playlist_reader = BufReader::with_capacity(64 * 1024, playlist_handle);
    let channel_files = ChannelFiles::open();

    let mut title;
    let mut url;
    let mut lines = playlist_reader.lines().skip(1);

    eprintln!("Extracting channels...");
    while let Some(title_line) = lines.next() {
        if let Some(url_line) = lines.next() {
            let title_line = title_line.unwrap();
            title = title_line
                .split_once(":-1,")
                .expect(
                    "Error extracting stream title, make sure your playlist doesn't use m3u_plus",
                )
                .1;
            url = url_line.expect("Number of lines should not be odd");

            write_entry(title.to_string(), url, &channel_files).unwrap();
        }
    }

    Ok(())
}
