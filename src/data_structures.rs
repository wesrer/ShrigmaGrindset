use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, PartialEq)]
pub enum TaskTypes {
    #[display("habits")]
    Habits,
    #[display("one time tasks")]
    OneTimes,
}
