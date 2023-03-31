mod connect_vpn;
mod get_channel;
mod mpv;

use crate::args::StreamType;
use crate::notif;
use std::path::PathBuf;
use std::{error::Error, result::Result};
use url::Url;

#[derive(Debug)]
pub struct Channel {
    title: String,
    url: Url,
}

pub fn play(stream_type: StreamType, _fzf: bool) -> Result<(), Box<dyn Error>> {
    let result = Channel::dmenu(stream_type)?;

    let channel = result.unwrap_or_else(|| {
        std::process::exit(1);
    });

    connect_vpn::connect("se-sto"); // TODO: add vpn option
    let exit_status = mpv::play(&channel);

    match exit_status.code().unwrap() {
        0 | 4 => return Ok(()),
        _ => {
            notif::error(&channel.title, exit_status);
            std::process::exit(1);
        }
    }
}
