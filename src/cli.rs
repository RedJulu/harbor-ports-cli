use crate::listener::ProtocolFilter;
use clap::Parser;
use clap::builder::styling::{AnsiColor, Effects, Styles};

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::BrightCyan.on_default() | Effects::BOLD)
        .usage(AnsiColor::BrightCyan.on_default() | Effects::BOLD)
        .literal(AnsiColor::BrightGreen.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::BrightYellow.on_default())
}

#[derive(Parser, Debug)]
#[command(
    name = "harbor",
    author = "RedJulu",
    version = "1.0",
    about = "⚓ Minimalist & fast port management CLI",
    styles = styles()
)]
pub struct Cli {
    #[arg(short, long, help = "List all active listening ports")]
    pub all: bool,

    #[arg(
        short = 'f',
        long,
        conflicts_with = "all",
        conflicts_with = "kill",
        conflicts_with = "force",
        value_name = "PORT",
        help = "Filters by port, PID or process name"
    )]
    pub filter: Option<String>,

    #[arg(
        short = 'p',
        long,
        conflicts_with = "kill",
        requires = "all",
        value_enum,
        default_value_t = ProtocolFilter::All,
        help = "Filters by Type TCP/UDP/All"
    )]
    pub protocol: ProtocolFilter,

    #[arg(
        long,
        help = "Show full adresses without shortening",
        requires = "all",
        conflicts_with = "kill"
    )]
    pub full_addr: bool,

    #[arg(
        short = 'n',
        long,
        value_name = "N",
        value_parser = clap::value_parser!(u32).range(1..),
        help = "Show only the first N entries"
    )]
    pub amount: Option<u32>,

    #[arg(long, help = "Output as JSON", conflicts_with = "kill")]
    pub json: bool,

    #[arg(
        short,
        long,
        value_name = "PORT",
        conflicts_with = "all",
        help = "Kill process running on specified port"
    )]
    pub kill: Option<u16>,

    #[arg(long, requires = "kill", help = "Force kill process (SIGKILL)")]
    pub force: bool,
}
