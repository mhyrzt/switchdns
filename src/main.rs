//! Entry point for switchdns. Handles root check and launches the UI.

mod dns;
mod ui;

use std::env;
use std::process::{Command, exit};

fn main() -> anyhow::Result<()> {
    // Ensure running as root
    if !is_root::is_root() {
        let args: Vec<String> = env::args().collect();
        let status = Command::new("sudo")
            .args(&args)
            .status()
            .expect("Failed to execute sudo. Is it installed?");
        exit(status.code().unwrap_or(1));
    }

    // Run the UI event loop
    ui::run_ui()
}
