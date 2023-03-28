use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, default_value = "/home/isak102/.local/share/riptv/")]
    data_directory: String,
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

fn main() {
    let _config = Args::parse();
}
