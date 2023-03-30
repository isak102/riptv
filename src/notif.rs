use std::process::{Command, ExitStatus};

fn notif(title: Option<&str>, body: &str) -> ExitStatus {
    let title = title.unwrap_or(" ");

    let exit_status = Command::new("notify-send")
        .arg("-t")
        .arg(2000.to_string())
        .arg("-c")
        .arg("no_title_and_history")
        .arg("-u")
        .arg("low")
        .arg(title)
        .arg(body)
        .status()
        .expect("failed to execute process");

    exit_status
}

pub fn stream(title: &str) -> ExitStatus {
    let message = format!("<span font='20px'>⏳ Loading <b>{}</b>...</span>", title);

    return notif(None, message.as_str());
}

pub fn error(title: &str, exit_status: ExitStatus) -> ExitStatus {
    let env_var_name = "GRUV_RED";
    let color = match std::env::var(env_var_name) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("{env_var_name} envirnoment variable doesn't exist");
            std::process::exit(1);
        }
    };

    let message = format!(
        "<span font='20px' color='{}'>❌ Error when playing <b>{}</b>. code:{} </span>",
        color, title, exit_status
    );

    return notif(None, message.as_str());
}
