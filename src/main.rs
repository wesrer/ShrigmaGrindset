mod cli;
mod data_structures;
mod db;
mod defaults;

use clap::Parser;

use cli::ShrigmaCli;
use db::create_connection;

fn main() {
    let conn = create_connection().unwrap();

    let cli = ShrigmaCli::parse();
    cli.command.parse_command(&conn);
}
