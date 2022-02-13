use clap::{Parser, Subcommand};

use crate::data_structures::TaskTypes;
use crate::defaults::{default_table, default_tasktype};

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
        #[clap(short, long, default_value_t = default_table.to_string())]
        project: String,
        #[clap(short = 'y', long, default_value_t = default_tasktype)]
        tasktype: TaskTypes,
    },
    /// List tasks
    List {
        #[clap(short, long, default_value_t = default_table.to_string())]
        project: String,
        #[clap(short = 'y', long, default_value_t = default_tasktype)]
        tasktype: TaskTypes,
    },
}

impl MainCommands {
    // NOTE: calling this `parse_command` instead of parse just to differentiate
    //       from the clap `parse` command
    pub fn parse_command(&self) {
        match self {
            MainCommands::Add {
                task,
                project,
                tasktype,
            } => {
                dbg!(task, project, tasktype);
            }
            MainCommands::List { project, tasktype } => {
                dbg!(project, tasktype);
            }
        }
    }
}
