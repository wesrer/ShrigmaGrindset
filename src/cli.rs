use clap::{Parser, Subcommand};
use rusqlite::Connection as SqliteConnection;

use crate::data_structures::{TaskPriority, TaskTypes};
use crate::db::{add_table, add_task, fetch_available_id, list_all};
use crate::defaults::default_project;

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
        task: String,
        #[clap(short, long, default_value_t = default_project.to_string())]
        project: String,
        #[clap(short = 'y', long, default_value_t = TaskTypes::OneTimeTasks)]
        tasktype: TaskTypes,
        #[clap(short = 'r', long, default_value_t = TaskPriority::Low as u64)]
        priority: u64,
    },
    /// List tasks
    List {
        #[clap(short, long, default_value_t = default_project.to_string())]
        project: String,
        #[clap(short = 'y', long, default_value_t = TaskTypes::OneTimeTasks)]
        tasktype: TaskTypes,
    },
    /// Start task
    Start {
        id: u64,
        #[clap(short, long, default_value_t = default_project.to_string())]
        project: String,
        #[clap(short = 'y', long, default_value_t = TaskTypes::OneTimeTasks)]
        tasktype: TaskTypes,
    },
    /// Mark a task as done
    Done {
        id: u64,
        #[clap(short, long, default_value_t = default_project.to_string())]
        project: String,
        #[clap(short = 'y', long, default_value_t = TaskTypes::OneTimeTasks)]
        tasktype: TaskTypes,
    },
}

impl MainCommands {
    // NOTE: calling this `parse_command` instead of parse just to differentiate
    //       from the clap `parse` command
    pub fn parse_command(&self, db_connection: &SqliteConnection) {
        match self {
            MainCommands::Add {
                task,
                project,
                tasktype,
                priority,
            } => {
                add_table(db_connection, project).unwrap();
                add_task(db_connection, project, *tasktype, *priority, task).unwrap();
            }
            MainCommands::List { project, tasktype } => {
                list_all(db_connection, project);
            }
            MainCommands::Start {
                id,
                project,
                tasktype,
            } => {
                dbg!(id);
            }
            MainCommands::Done {
                id,
                project,
                tasktype,
            } => {
                let id = fetch_available_id(db_connection, project, *tasktype).unwrap();
                dbg!(id);
            }
        }
    }
}
