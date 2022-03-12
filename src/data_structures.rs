use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, PartialEq, Clone, Copy)]
pub enum TaskTypes {
    #[display("habits")]
    Habits,
    #[display("one-time-tasks")]
    OneTimeTasks,
}

impl Default for TaskTypes {
    fn default() -> Self {
        Self::OneTimeTasks
    }
}

#[derive(Debug, Display, FromStr, PartialEq, Clone, Copy)]
pub enum TaskPriority {
    Urgent = 128,
    High = 80,
    Medium = 40,
    Low = 10,
}

impl Default for TaskPriority {
    fn default() -> Self {
        Self::Low
    }
}

pub struct Task {
    task: String,
    project: String,
    tasktype: TaskTypes,
    task_id: u64,
    priority: u64,
}
