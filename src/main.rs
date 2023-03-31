use clap::{Parser, Subcommand};
use strum_macros::Display;
use tokio;

mod consts;
mod file_io;
mod notif;
mod player;
mod update;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
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
enum Commands {
    // TODO: add Setup command
    /// Update playlists
    Update {
        /// The URL of the playlist. By default this is taken from url.txt inside of the
        /// data-directory
        #[arg(short, long)]
        url: Option<String>,

        #[arg(long)]
        history: bool,
    },

    /// Play a stream
    #[command(arg_required_else_help(true))]
    Play {
        /// Either play [live] streams or watch [vod] content
        #[clap(value_enum)]
        stream_type: StreamType,

        /// Use fzf instead of dmenu
        #[arg(long)]
        fzf: bool,
    },
}

#[tokio::main]
async fn main() {
    let config = Args::parse();

    match config.command {
        None => return,
        Some(cmd) => match cmd {
            Commands::Update { url, history } => {
                update::update(url, history).await.unwrap();
            }
            Commands::Play { stream_type, fzf } => {
                player::play(stream_type, fzf).unwrap();
            }
        },
    }
}
