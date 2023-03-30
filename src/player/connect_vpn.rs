use std::process::{Command, ExitStatus};

pub fn connect(vpn_name: &str) -> ExitStatus {
    eprintln!("Connecting to {}...", vpn_name);
    let exit_status = Command::new("nmcli")
        .arg("c")
        .arg("u")
        .arg(vpn_name)
        .status()
        .expect("failed to execute process");

    exit_status
}
