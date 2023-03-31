use clap::{Parser, Subcommand};
use reqwest::Url;
use strum_macros::Display;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
pub(super) struct Args {
    #[command(subcommand)]
    pub(super) command: Option<Commands>,
}

#[derive(clap::ValueEnum, Clone, Display)]
pub enum StreamType {
    #[strum(serialize = "live")]
    Live,
    #[strum(serialize = "vod")]
    Vod,
    #[value(skip)]
    Other,
}
impl From<&str> for StreamType {
    fn from(url: &str) -> StreamType {
        let ext = url.split(".").last().expect("URL should contain dot \".\"");
        match ext {
            "mp4" => StreamType::Vod,
            "m3u8" => StreamType::Live,
            "mkv" => StreamType::Live,
            _ => StreamType::Other,
        }
    }
}

#[derive(clap::ValueEnum, Clone)]
pub enum Launcher {
    // TODO: use this
    Dmenu,
    Rofi,
    Fzf,
}

#[derive(Subcommand)]
pub(super) enum Commands {
    // TODO: add Setup command
    /// Update playlists [alias = u]
    #[command(alias = "u")]
    Update {
        /// The URL of the playlist. By default this is taken from url.txt inside of the
        /// data-directory
        #[arg(short, long)]
        url: Option<String>,

        #[command(subcommand)]
        commands: Option<UpdateCommands>,
    },

    /// Play a stream [alias = p]
    #[command(arg_required_else_help(true), alias = "p")]
    Play {
        /// Either play [live] streams or watch [vod] content
        #[clap(value_enum)]
        stream_type: StreamType,

        /// Use fzf instead of dmenu
        #[arg(long)]
        fzf: bool,
    },

    /// Set URL as default URL when running update
    #[command(arg_required_else_help(true))]
    SetUrl {
        /// The URL to set
        #[arg(short, long)]
        url: Url,
    },
}

#[derive(Subcommand)]
pub(super) enum UpdateCommands {
    /// Show update history, no update will be performed [alias = h]
    #[command(alias = "h")]
    History {}, // FIXME: implement this
}
