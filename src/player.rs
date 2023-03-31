mod connect_vpn;
mod get_channel;
mod mpv;

use crate::args::StreamType;
use crate::{args::Launcher, notif};
use std::path::PathBuf;
use std::{error::Error, result::Result};
use url::Url;

use is_terminal::IsTerminal;

#[derive(Debug)]
pub struct Channel {
    title: String,
    url: Url,
}

pub fn play(stream_type: StreamType, launcher: Option<Launcher>) -> Result<(), Box<dyn Error>> {
    let result = if launcher.is_some() {
        match launcher.unwrap() {
            Launcher::Dmenu => Channel::get_with_dmenu(stream_type)?,
            Launcher::Fzf => Channel::get_with_fzf(stream_type)?,
        }
    } else {
        if std::io::stdout().is_terminal() {
            Channel::get_with_fzf(stream_type)?
        } else {
            Channel::get_with_dmenu(stream_type)?
        }
    };

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
