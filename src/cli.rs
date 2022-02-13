use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "sg")]
pub struct ShrigmaCli {
    #[clap(subcommand)]
    pub command: MainCommands,
}

#[derive(Subcommand, Debug)]
pub enum MainCommands {
    /// Add a task
    Add {
        #[clap(short, long)]
        task: String,
    },
}
