use clap::{Parser, Subcommand};
use std::path::PathBuf;
use strum_macros::Display;
use tokio;

mod file_io;
mod notif;
mod player;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, default_value = "/home/isak102/.local/share/riptv/")]
    data_directory: PathBuf,
}

#[derive(clap::ValueEnum, Clone, Display)]
pub enum StreamType {
    #[strum(serialize = "live")]
    Live,
    #[strum(serialize = "vod")]
    Vod,
}

#[derive(clap::ValueEnum, Clone)]
pub enum Launcher {
    Dmenu,
    Rofi,
    Fzf,
}

#[derive(Subcommand)]
enum Commands {
    /// Update playlists
    Update {
        /// The URL of the playlist. By default this is taken from url.txt inside of the
        /// data-directory
        #[arg(short, long)]
        url: Option<String>,
    },

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
            Commands::Update { url } => {
                let playlist = file_io::install::install(&config.data_directory, url)
                    .await
                    .unwrap();

                file_io::extract_channels::extract_from_playlist(&playlist, &config.data_directory)
                    .unwrap();
            }
            Commands::Play { stream_type, fzf } => {
                player::play(stream_type, &config.data_directory, fzf).unwrap();
            }
        },
    }
}
