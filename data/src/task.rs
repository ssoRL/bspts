use serde::{Deserialize, Serialize};
use chrono::{NaiveDate};

/// The fields that must be specified when creating a new task
/// The rest will be set to default values
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewTask {
    pub name: String,
    pub description: String,
    pub bspts: i32,
    pub frequency: TaskInterval, 
}

/// The interval at which this task should be completed
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum TaskInterval {
    /// Task to be completed every [param1] days
    Days{every: u32},
    // Task to be completed every [param1] weeks on weekday [param2]
    Weeks{every: u32, weekday: u32},
    // Task to be completed every [param1] months on the [param2]th of the month
    Months{every: u32, day_of_month: u32},
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub bspts: i32,
    pub is_done: bool,
    pub next_reset: NaiveDate,
    pub frequency: TaskInterval,
}