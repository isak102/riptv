use std::process::{Command, ExitStatus};
use url::Url;

pub fn play(url: Url) -> ExitStatus {
    // TODO: add channel name
    super::super::notif::stream("PLACEHOLDER");
    let exit_status = Command::new("mpv")
        .arg("--load-unsafe-playlists")
        .arg("--loop-playlist=inf")
        .arg("--volume=70")
        .arg(url.as_str())
        .status()
        .expect("failed to execute process");

    exit_status
}
