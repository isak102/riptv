mod connect_vpn;
mod get_url;
mod mpv;

use crate::StreamType;
use std::path::PathBuf;
use std::{error::Error, result::Result};
use url::Url;

#[derive(Debug)]
struct Channel {
    title: String,
    url: Url,
}

pub fn play(
    stream_type: StreamType,
    data_directory: &PathBuf,
    _fzf: bool,
) -> Result<(), Box<dyn Error>> {
    let result = Channel::dmenu(stream_type, data_directory)?;
    eprintln!("{:?}", result);

    if let Some(Channel { title, url }) = result {
        // FIXME: Check if stream is already playing and kill it.
        connect_vpn::connect("se-sto");
        mpv::play(url);
    } else {
        std::process::exit(1);
    }

    Ok(())
}
