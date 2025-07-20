//! Entry point for switchdns. Handles root check and launches the UI.

mod dns;
mod ui;

use std::env;
use std::process::{Command, exit};

fn main() -> anyhow::Result<()> {
    // Ensure running as root
    if !is_root::is_root() {
        let exe = env::current_exe().expect("Failed to get current executable");
        let args: Vec<String> = env::args().skip(1).collect(); // skip program name
        let status = Command::new("sudo")
            .arg(exe)
            .args(&args)
            .status()
            .expect("Failed to execute sudo. Is it installed?");
        exit(status.code().unwrap_or(1));
    }

    // Run the UI event loop
    ui::run_ui()
}
