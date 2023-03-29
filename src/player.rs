use crate::StreamType;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{error::Error, result::Result};
use strip_ansi_escapes;
use url::Url;

fn strip_non_ascii(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii()).collect()
}

fn dmenu(stream_type: StreamType, data_directory: &PathBuf) -> Result<Option<Url>, Box<dyn Error>> {
    let prompt = "Live channel📺";

    let channels = Command::new("cat")
        .arg(data_directory.join(format!("{stream_type}.txt")))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute cat command");

    let dmenu_command = Command::new("dmenu")
        .arg("-D")
        .arg("§")
        .arg("-l")
        .arg("20")
        .arg("-g")
        .arg("2")
        .arg("-p")
        .arg(prompt)
        .arg("-vf")
        .stdin(channels.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute dmenu command");

    let dmenu_output = dmenu_command
        .wait_with_output()
        .expect("failed to read dmenu output")
        .stdout;
    let selection: String = String::from_utf8_lossy(&dmenu_output).into_owned();

    let unstripped_url =
        std::str::from_utf8(&strip_ansi_escapes::strip(selection.as_str()).unwrap())
            .unwrap()
            .to_string();
    let url = strip_non_ascii(unstripped_url.trim());

    match Url::parse(url.as_str()) {
        Ok(url_option) => Ok(Some(url_option)),
        Err(_) => Ok(None),
    }
}

pub fn play(
    stream_type: StreamType,
    data_directory: &PathBuf,
    _fzf: bool,
) -> Result<(), Box<dyn Error>> {
    let url = dmenu(stream_type, data_directory)?;

    Ok(())
}
