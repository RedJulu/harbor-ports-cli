mod cli;
mod listener;
mod net;

use crate::{
    listener::{filter_for_port, get_active_listeners},
    net::{kill_port, list_active_ports},
};
use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();
    let protocol = args.protocol;

    if args.all {
        list_active_ports(
            get_active_listeners(protocol),
            args.full_addr,
            args.amount,
            None,
            args.json,
        );
    } else if let Some(port) = args.kill {
        kill_port(port, args.force, protocol);
    } else if let Some(port) = args.filter {
        list_active_ports(
            filter_for_port(port, protocol),
            args.full_addr,
            args.amount,
            Some(port),
            args.json,
        );
    } else {
        use clap::CommandFactory;
        let _ = Cli::command().print_help();
        println!();
    }
}
