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
    // Task to be completed every x weeks on the specified weekday
    Weeks{every: u32, weekday: u32},
    // Task to be completed every x months on the [day_of_month]th of the month
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