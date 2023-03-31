use tokio;

mod args;
mod consts;
mod file_io;
mod notif;
mod player;
mod update;

use args::Commands;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();

    match args.command {
        None => return,
        Some(cmd) => match cmd {
            Commands::Update { url, commands } => {
                let print_history = commands.is_some();
                update::update(url, print_history).await.unwrap(); // TODO: pass update options in
                                                                   // some way
            }
            Commands::Play { stream_type, launcher, vpn, disable_vpn } => {
                player::play(stream_type, launcher, vpn, disable_vpn).unwrap();
            }
            Commands::SetUrl { url } => file_io::set_url(url).unwrap(),
        },
    }
}
