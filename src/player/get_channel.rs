use super::super::consts::DATA_DIRECTORY;
use core::panic;
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

fn get_prompt(stream_type: &StreamType) -> &'static str {
    match stream_type {
        StreamType::Live => "Live channel📺",
        StreamType::Vod => "Video📺",
        StreamType::Other => panic!("StreamType should never be other"),
    }
}

fn get_channels_file(stream_type: &StreamType) -> PathBuf {
    if let StreamType::Other = stream_type {
        panic!("StreamType should never be other")
    };

    DATA_DIRECTORY.join(format!("{stream_type}.txt"))
}

fn cmd_output_to_url(output: &Vec<u8>) -> Option<Url> {
    let selection: String = String::from_utf8_lossy(&output).into_owned();

    let unstripped_url =
        std::str::from_utf8(&strip_ansi_escapes::strip(selection.as_str()).unwrap())
            .unwrap()
            .to_string();
    let stipped_url = strip_non_ascii(unstripped_url.trim());

    Url::parse(stipped_url.as_str()).ok()
}

impl Channel {
    pub fn get_with_dmenu(stream_type: StreamType) -> Result<Option<Channel>, Box<dyn Error>> {
        let prompt = get_prompt(&stream_type);
        let channels_file = get_channels_file(&stream_type);

        let cat_command = Command::new("cat")
            .arg(&channels_file)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute cat command");

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
            .stdin(cat_command.stdout.unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute dmenu command");

        let output = dmenu_command
            .wait_with_output()
            .expect("Failed to read pipe output")
            .stdout;

        let url = match cmd_output_to_url(&output) {
            Some(v) => v,
            None => return Ok(None),
        };
        let title = get_channel_title(&channels_file, &url);

        Ok(Some(Channel { url, title }))
    }

    pub fn get_with_fzf(stream_type: StreamType) -> Result<Option<Channel>, Box<dyn Error>> {
        let prompt = get_prompt(&stream_type);
        let channels_file = get_channels_file(&stream_type);

        let cat_command = Command::new("cat")
            .arg(&channels_file)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute cat command");

        let sed_command = Command::new("sed")
            .arg("s/§/\t/")
            .stdin(cat_command.stdout.unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute sed command");

        let fzf_command = Command::new("fzf")
            .arg("--layout=reverse")
            .arg("--height=66%")
            .arg("--info=hidden")
            .arg(format!("--prompt={}: ", prompt))
            .arg("-d")
            .arg("\t")
            .arg("--with-nth")
            .arg("-2")
            .stdin(sed_command.stdout.unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute sed command");

        let awk_command = Command::new("awk")
            .arg("{$1=$1;print}")
            .stdin(fzf_command.stdout.unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute awk command");

        let rev_command = Command::new("rev")
            .stdin(awk_command.stdout.unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute rev command");

        let cut_command = Command::new("cut")
            .arg("-d")
            .arg(" ")
            .arg("-f")
            .arg("1")
            .stdin(rev_command.stdout.unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute rev command");

        let rev2_command = Command::new("rev")
            .stdin(cut_command.stdout.unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute rev command");

        let output = rev2_command
            .wait_with_output()
            .expect("Failed to read pipe output")
            .stdout;

        let url = match cmd_output_to_url(&output) {
            Some(v) => v,
            None => return Ok(None),
        };
        let title = get_channel_title(&channels_file, &url);

        Ok(Some(Channel { url, title }))
    }
}
