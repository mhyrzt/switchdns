//! DNS logic module for switchdns

use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    net::IpAddr,
    str::FromStr,
};

/// Represents a DNS provider option.
#[derive(Debug, Clone)]
pub struct DnsOption {
    pub name: String,
    pub servers: Vec<String>,
}

/// Returns a list of built-in DNS provider options.
pub fn builtin_dns_options() -> Vec<DnsOption> {
    vec![
        DnsOption {
            name: "Cloudflare".into(),
            servers: vec!["1.1.1.1".into(), "1.0.0.1".into()],
        },
        DnsOption {
            name: "Google".into(),
            servers: vec!["8.8.8.8".into(), "8.8.4.4".into()],
        },
        DnsOption {
            name: "Quad9".into(),
            servers: vec!["9.9.9.9".into(), "149.112.112.112".into()],
        },
        DnsOption {
            name: "OpenDNS".into(),
            servers: vec!["208.67.222.222".into(), "208.67.220.220".into()],
        },
        DnsOption {
            name: "CleanBrowsing".into(),
            servers: vec!["185.228.168.9".into(), "185.228.169.9".into()],
        },
        DnsOption {
            name: "Shecan".into(),
            servers: vec!["178.22.122.100".into(), "185.51.200.2".into()],
        },
        DnsOption {
            name: "Electro".into(),
            servers: vec!["78.157.42.100".into(), "78.157.42.101".into()],
        },
        DnsOption {
            name: "Begzar".into(),
            servers: vec!["185.55.226.26".into(), "185.55.225.25".into()],
        },
        DnsOption {
            name: "Reset (Automatic/ISP)".into(),
            servers: vec![],
        },
    ]
}

const CONFIG_FILE_NAME: &str = ".dns-switcher";

/// Reads extra DNS options from the user's config file.
pub fn read_extra_dns_options() -> Vec<DnsOption> {
    let path = dirs::home_dir()
        .map(|p| p.join(CONFIG_FILE_NAME))
        .filter(|p| p.exists());

    let file = match path {
        Some(ref p) => match File::open(p) {
            Ok(f) => f,
            Err(_) => return Vec::new(),
        },
        None => return Vec::new(),
    };

    BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            let mut parts = line.trim().split('|').map(str::trim);
            let name = parts.next()?.to_string();
            let servers: Vec<String> = parts
                .filter(|s| IpAddr::from_str(s).is_ok())
                .map(str::to_string)
                .collect();
            (!servers.is_empty()).then_some(DnsOption { name, servers })
        })
        .collect()
}

/// Returns all DNS options (built-in and user-defined).
pub fn all_dns_options() -> Vec<DnsOption> {
    builtin_dns_options()
        .into_iter()
        .chain(read_extra_dns_options())
        .collect()
}

/// Reads the current DNS servers from /etc/resolv.conf.
pub fn current_dns_servers() -> String {
    let content = std::fs::read_to_string("/etc/resolv.conf").unwrap_or_default();
    let servers: Vec<String> = content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with("nameserver") {
                line.split_whitespace().nth(1).map(str::to_string)
            } else {
                None
            }
        })
        .collect();
    if servers.is_empty() {
        "Automatic/ISP".to_string()
    } else {
        servers.join(", ")
    }
}

/// Writes the selected DNS option to /etc/resolv.conf.
pub fn write_resolv_conf(dns_option: &DnsOption) -> bool {
    let mut content = String::new();

    if dns_option.servers.is_empty() {
        content.push_str(&format!("# {}\n", dns_option.name));
    } else {
        content.push_str(&format!("# {} DNS\n", dns_option.name));
        for server in &dns_option.servers {
            content.push_str(&format!("nameserver {}\n", server));
        }
    }

    File::create("/etc/resolv.conf")
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .is_ok()
}
