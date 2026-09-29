mod cli;
mod listener;
mod net;

use crate::net::{kill_port, list_active_ports};
use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    if args.all {
        list_active_ports();
    } else if let Some(port) = args.kill {
        kill_port(port, args.force);
    } else {
        use clap::CommandFactory;
        let _ = Cli::command().print_help();
        println!();
    }
}
