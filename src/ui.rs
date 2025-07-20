//! UI logic module for switchdns

use crate::dns::{DnsOption, all_dns_options, current_dns_servers, write_resolv_conf};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    TerminalOptions, Viewport,
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::{
    io::{self},
    time::Duration,
};

const INLINE_HEIGHT: u16 = 20;

/// Runs the main UI event loop.
pub fn run_ui() -> anyhow::Result<()> {
    let dns_list = all_dns_options();
    let mut msg: Option<String> = None;
    let mut state = ListState::default();
    state.select(Some(0));
    let mut current_dns = current_dns_servers();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Hide)?;

    let options = TerminalOptions {
        viewport: Viewport::Inline(INLINE_HEIGHT),
    };
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::with_options(backend, options)?;

    loop {
        draw_ui(&mut terminal, &dns_list, &mut state, &current_dns, &msg)?;

        if event::poll(Duration::from_millis(300))? {
            if let CEvent::Key(key) = event::read()? {
                let selected = state.selected().unwrap_or(0);
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => {
                        let reset_dns = DnsOption {
                            name: "Reset (Automatic/ISP)".into(),
                            servers: vec![],
                        };
                        let result = write_resolv_conf(&reset_dns);
                        if result {
                            current_dns = "Automatic/ISP".to_string();
                            msg = Some("Reset DNS to Automatic/ISP".to_string());
                        } else {
                            msg = Some("Failed to reset (are you root?)".to_string());
                        }
                    }
                    KeyCode::Up => state.select(Some(selected.saturating_sub(1))),
                    KeyCode::Down => {
                        state.select(Some(std::cmp::min(selected + 1, dns_list.len() - 1)))
                    }
                    KeyCode::Enter => {
                        let dns = &dns_list[selected];
                        let result = write_resolv_conf(dns);
                        if result {
                            current_dns = if dns.servers.is_empty() {
                                "Automatic/ISP".to_string()
                            } else {
                                dns.servers.join(", ")
                            };
                            msg = Some(format!("Switched DNS to: {}", dns.name));
                        } else {
                            msg = Some("Failed to switch (are you root?)".to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), Show)?;
    Ok(())
}

/// Draws the UI for the DNS switcher.
fn draw_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    dns_list: &[DnsOption],
    state: &mut ListState,
    current_dns: &str,
    msg: &Option<String>,
) -> anyhow::Result<()> {
    terminal.draw(|f| {
        let area = f.area();
        let current_height: u16 = 3;
        let status_height: u16 = if msg.is_some() { 3 } else { 0 };
        let list_height = area.height.saturating_sub(current_height + status_height);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(current_height),
                Constraint::Length(list_height),
                Constraint::Length(status_height),
            ])
            .split(area);

        // Current DNS info
        let current_block = Block::default()
            .title("Current DNS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let current_text = Paragraph::new(format!("Servers: {}", current_dns))
            .block(current_block)
            .alignment(Alignment::Left)
            .style(Style::default().fg(Color::White));
        f.render_widget(current_text, chunks[0]);

        // DNS options list
        let items: Vec<ListItem> = dns_list
            .iter()
            .map(|d| {
                let servers_str = if d.servers.is_empty() {
                    "(Automatic)".to_string()
                } else {
                    format!("({})", d.servers.join(", "))
                };
                ListItem::new(Line::from(vec![
                    Span::styled(
                        d.name.as_str(),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(servers_str, Style::default().fg(Color::Gray)),
                ]))
            })
            .collect();
        let list = List::new(items)
            .block(
                Block::default()
                    .title("Select DNS Provider (↑↓ navigate, Enter select, r reset, q quit)")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(list, chunks[1], state);

        // Status message (if present)
        if let Some(m) = msg {
            let status_block = Block::default()
                .title("Status")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta));
            let status_text = Paragraph::new(m.as_str())
                .block(status_block)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
            f.render_widget(status_text, chunks[2]);
        }
    })?;
    Ok(())
}
