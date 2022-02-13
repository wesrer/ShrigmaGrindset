mod cli;
mod data_structures;
mod db;
mod defaults;

use clap::Parser;

use cli::{MainCommands, ShrigmaCli};

fn main() {
    let cli = ShrigmaCli::parse();

    cli.command.parse_command();
}
