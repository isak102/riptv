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
    Other(String),
}
impl StreamType {
    pub fn from_url(url: &str) -> StreamType {
        let ext = url.split(".").last().expect("URL should contain dot \".\"");
        match ext {
            "m3u8" => StreamType::Live,
            "mp4" => StreamType::Vod,
            "mkv" => StreamType::Vod,
            s => StreamType::Other(s.to_owned()),
        }
    }
}

#[derive(clap::ValueEnum, Clone, Display)]
pub enum Launcher {
    #[strum(serialize = "dmenu")]
    Dmenu,
    #[strum(serialize = "fzf")]
    Fzf,
}

#[derive(Subcommand)]
pub(super) enum Commands {
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
        /// Type of stream to play
        #[clap(value_enum)]
        stream_type: StreamType,

        /// Force a launcher to use
        #[arg(long, short)]
        launcher: Option<Launcher>,
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
