use super::Channel;
use std::process::{Command, ExitStatus};

pub fn play(channel: &Channel) -> ExitStatus {
    eprintln!("Playing {}...", &channel.title);
    super::super::notif::stream(&channel.title);
    let exit_status = Command::new("mpv")
        .arg("--load-unsafe-playlists")
        .arg("--loop-playlist=inf")
        .arg("--volume=70")
        .arg(format!("--title={} - riptv", &channel.title))
        .arg(channel.url.as_str())
        .status()
        .expect("failed to execute process");
    exit_status
}
