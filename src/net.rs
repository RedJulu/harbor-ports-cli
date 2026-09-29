use crate::listener::{PortInfo, terminate_port};
use colored::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Table};

#[cfg(unix)]
fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

pub fn list_active_ports(data: Vec<PortInfo>) {
    if data.is_empty() {
        println!(
            "{} No active processes listening on ports.",
            "INFO".blue().bold()
        );
        return;
    }

    println!(
        "\n     {} {}",
        "⚓ HARBOR".cyan().bold(),
        format!("({} active Ports)", data.len()).bright_black(),
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
        Cell::new("Process")
            .fg(Color::Magenta)
            .set_alignment(CellAlignment::Center),
        Cell::new("PID")
            .fg(Color::Blue)
            .set_alignment(CellAlignment::Center),
    ]);

    for l in data {
        table.add_row(vec![
            Cell::new(&l.address)
                .fg(Color::White)
                .set_alignment(CellAlignment::Center),
            Cell::new(l.port.to_string()).fg(Color::Yellow),
            Cell::new(&l.process).fg(Color::Green),
            Cell::new(l.pid.to_string()).fg(Color::Blue),
        ]);
    }

    println!("{table}\n")
}

pub fn kill_port(port: u16, force: bool) {
    let success = terminate_port(port, force);

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
