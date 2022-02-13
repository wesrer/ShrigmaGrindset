mod cli;
mod db;

use clap::Parser;

use cli::{MainCommands, ShrigmaCli};

fn main() {
    let cli = ShrigmaCli::parse();

    match &cli.command {
        MainCommands::Add { task } => {
            dbg!(task);
        }
    }
}
