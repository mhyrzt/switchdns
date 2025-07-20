# switchdns

A terminal-based tool to easily switch your system DNS servers on Linux.

[![asciicast](https://asciinema.org/a/6EEEOJTXowPgoVF66jbegzvZ1.svg)](https://asciinema.org/a/6EEEOJTXowPgoVF66jbegzvZ1)

## Features
- Interactive terminal UI for selecting DNS providers
- Built-in list of popular DNS providers (Cloudflare, Google, Quad9, OpenDNS, CleanBrowsing, Shecan, Electro, Begzar)
- Option to reset to automatic/ISP DNS
- Add your own custom DNS providers via a config file
- Displays current DNS settings
- Automatically requests root privileges if needed

## Installation

You need [Rust](https://www.rust-lang.org/tools/install) installed.

Install from [crates.io](https://crates.io/crates/switchdns):

```sh
cargo install switchdns
```

Or build manually from source:

```sh
cargo build --release
```

The binary will be at `target/release/switchdns`.

## Usage

Run the tool:

```sh
switchdns
```

- If not run as root, the tool will automatically prompt for root privileges.
- Use the arrow keys to navigate the DNS provider list
- Press `Enter` to select and apply a DNS provider
- Press `r` to reset to automatic/ISP DNS
- Press `q` to quit

## Custom DNS Providers

You can add your own DNS providers by creating a file named `.dns-switcher` in your home directory. Each line should be in the format:

```
Provider Name | 1.2.3.4 | 5.6.7.8
```

Example:
```
MyDNS | 123.45.67.89 | 98.76.54.32
```

## How it works

- Modifies `/etc/resolv.conf` to set the selected DNS servers
- Reads current DNS from `/etc/resolv.conf`
- Uses [ratatui](https://crates.io/crates/ratatui) and [crossterm](https://crates.io/crates/crossterm) for the terminal UI

## License

MIT

---

© 2025 Mahyar Riazati
