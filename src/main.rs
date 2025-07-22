mod dns;
mod ui;

use std::env;
use std::process::{exit, Command};

fn main() -> anyhow::Result<()> {
    if !is_root::is_root() {
        let exe = env::current_exe().expect("Failed to get current executable");
        let args: Vec<String> = env::args().skip(1).collect();
        let status = Command::new("sudo")
            .arg(exe)
            .args(&args)
            .status()
            .expect("Failed to execute sudo. Is it installed?");
        exit(status.code().unwrap_or(1));
    }

    ui::run_ui()
}
