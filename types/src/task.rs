use serde::{Deserialize, Serialize};
use chrono::{NaiveDate};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum TaskInterval {
    Days(i32),
    Months(i32),
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