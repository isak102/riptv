use super::super::consts::DATA_DIRECTORY;
use std::process::{Command, Stdio};
use strip_ansi_escapes;
use url::Url;

use super::*;

fn strip_non_ascii(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii()).collect()
}

fn get_channel_title(channels_file: &PathBuf, url: &Url) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "grep \"{}\" \"{}\" | awk -F§ 'NF{{NF-=1}};1'",
            url,
            channels_file.to_str().unwrap()
        ))
        .output()
        .expect("Failed to execute command");

    let title_untrimmed = String::from_utf8(output.stdout).unwrap(); // TODO: remove unwrap
    let title = title_untrimmed.trim();

    title.to_string()
}

impl Channel {
    pub fn dmenu(stream_type: StreamType) -> Result<Option<Channel>, Box<dyn Error>> {
        let prompt = match stream_type {
            StreamType::Live => "Live channel📺",
            StreamType::Vod => "Video📺",
        };

        let data_directory = PathBuf::from(&DATA_DIRECTORY);

        let channels_file = data_directory.join(format!("{stream_type}.txt"));
        let channels = Command::new("cat")
            .arg(&channels_file)
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
        let stipped_url = strip_non_ascii(unstripped_url.trim());

        let url = match Url::parse(stipped_url.as_str()) {
            Ok(url_option) => url_option,
            Err(_) => return Ok(None),
        };

        let title = get_channel_title(&channels_file, &url);

        Ok(Some(Channel { url, title }))
    }
}
