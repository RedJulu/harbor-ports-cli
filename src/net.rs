use crate::listener::{PortInfo, ProtocolFilter, terminate_port};
use colored::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Table};

#[cfg(unix)]
fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

pub fn list_active_ports(
    mut data: Vec<PortInfo>,
    full_addr: bool,
    amount: Option<u32>,
    filter_flag: Option<u16>,
    json: bool,
) {
    data.sort_by(|a, b| (a.port, &a.protocol, &a.address).cmp(&(b.port, &b.protocol, &b.address)));

    if json {
        if let Some(n) = amount {
            data.truncate(n as usize);
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&data).unwrap_or_else(|_| "[]".to_string())
        );
        return;
    }

    if data.is_empty() {
        match filter_flag {
            Some(port) => println!(
                "{} No active processes listening on port {}.",
                "INFO".blue().bold(),
                port.to_string().yellow(),
            ),
            None => println!(
                "{} No active processes listening on ports.",
                "INFO".blue().bold()
            ),
        }
        return;
    }

    let total = data.len();
    if let Some(n) = amount {
        data.truncate(n as usize);
    }

    let count_label = if data.len() < total {
        format!("(showing {} of {} Ports)", data.len(), total)
    } else {
        format!("({} active Ports)", total)
    };

    println!(
        "\n     {} {}",
        "⚓ HARBOR".cyan().bold(),
        count_label.bright_black(),
    );

    #[cfg(unix)]
    if !is_root() {
        println!("       {}", "⚠️Limited permissions.".red());
    };

    println!("\n");

    let mut table = Table::new();
    table
        .load_style(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Adress")
            .fg(Color::Cyan)
            .set_alignment(CellAlignment::Center),
        Cell::new("Port")
            .fg(Color::Yellow)
            .set_alignment(CellAlignment::Center),
        Cell::new("Protocol")
            .fg(Color::Red)
            .set_alignment(CellAlignment::Center),
        Cell::new("Process")
            .fg(Color::Magenta)
            .set_alignment(CellAlignment::Center),
        Cell::new("PID")
            .fg(Color::Blue)
            .set_alignment(CellAlignment::Center),
    ]);

    for l in data {
        let address = format_address(&l.address, full_addr);
        let protocol_color = if l.protocol.eq_ignore_ascii_case("udp") {
            Color::Magenta
        } else {
            Color::Green
        };

        table.add_row(vec![
            Cell::new(address)
                .fg(Color::White)
                .set_alignment(CellAlignment::Center),
            Cell::new(l.port.to_string()).fg(Color::Yellow),
            Cell::new(&l.protocol)
                .fg(protocol_color)
                .set_alignment(CellAlignment::Center),
            Cell::new(&l.process).fg(Color::Green),
            Cell::new(l.pid.to_string()).fg(Color::Blue),
        ]);
    }

    println!("{table}\n");
    println!("{} {}\n", "⚓ »".bright_black(), invoked_command().yellow());
}

pub fn kill_port(port: u16, force: bool, filter: ProtocolFilter) {
    let success = terminate_port(port, force, filter);

    if success {
        let mode = if force { " (SIGKILL)" } else { "" };
        println!(
            "{} Process using port {}{} killed successfully.",
            "✔".green().bold(),
            port.to_string().yellow().bold(),
            mode,
        );
    } else {
        println!(
            "{} Could not kill process using port {}.",
            "✖".red().bold(),
            port.to_string().yellow().bold()
        );
    }
}

fn format_address(addr: &str, full: bool) -> String {
    const MAX: usize = 21;

    if addr == "::" {
        return "[::]".to_string();
    }

    if full || addr.len() <= MAX {
        return addr.to_string();
    }

    let head = &addr[..10];
    let tail = &addr[addr.len() - 8..];
    format!("{head}...{tail}")
}

fn invoked_command() -> String {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        "harbor".to_string()
    } else {
        format!("harbor {}", args.join(" "))
    }
}
