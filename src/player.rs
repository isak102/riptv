mod get_url;
mod mpv;
mod connect_vpn;

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
        connect_vpn::connect("se-sto");
        mpv::play(url);
    } else {
        std::process::exit(1);
    }

    Ok(())
}
