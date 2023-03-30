mod get_url;
mod mpv; // FIXME: implement
         // mod notif; // FIXME: implement
         // mod connect_vpn; // FIXME: implement

use crate::StreamType;
use std::path::PathBuf;
use std::{error::Error, result::Result};

pub fn play(
    stream_type: StreamType,
    data_directory: &PathBuf,
    _fzf: bool,
) -> Result<(), Box<dyn Error>> {
    let result = get_url::dmenu(stream_type, data_directory)?;
    eprintln!("{:?}", result);
    if let Some(url) = result {
        mpv::play(url);
    } else {
        std::process::exit(1);
    }

    Ok(())
}
