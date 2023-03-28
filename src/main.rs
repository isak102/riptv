use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod install;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, default_value = "/home/isak102/.local/share/riptv/")]
    data_directory: PathBuf,
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
}

#[tokio::main]
async fn main() {
    let config = Args::parse();

    eprintln!("{:?}", install::install(config.data_directory, None).await);
}
