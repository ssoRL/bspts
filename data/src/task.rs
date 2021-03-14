use serde::{Deserialize, Serialize};
use chrono::{NaiveDate};
use crate::icon::{TaskIcon};

/// The fields that must be specified when creating a new task
/// The rest will be set to default values
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewTask {
    pub name: String,
    pub description: String,
    pub bspts: i32,
    pub frequency: TaskInterval, 
    pub icon: TaskIcon,
}

/// The interval at which this task should be completed
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum TaskInterval {
    /// Task to be completed every X days
    Days{every: u32},
    /// Task to be completed every X weeks on the specified weekday.
    /// Weekday is defined with Monday=0, Tue=1, etc.
    Weeks{every: u32, weekday: u32},
    /// Task to be completed every X months on specified day of the month
    /// Day of the month is defined so 1st=1, 2nd=2, etc.
    Months{every: u32, day_of_month: u32},
}

impl TaskInterval {
    /// Returns how often this is called
    pub fn every(self: &Self) -> u32 {
        match self {
            Self::Days{every} => *every,
            Self::Weeks{every, weekday:_} => *every,
            Self::Months{every, day_of_month:_} => *every,
        }
    }

    pub fn in_days(self: &Self) -> bool {
        match self {
            Self::Days{every:_} => true,
            _ => false,
        }
    }

    pub fn in_weeks(self: &Self) -> bool {
        match self {
            Self::Weeks{every:_, weekday:_} => true,
            _ => false,
        }
    }

    pub fn in_months(self: &Self) -> bool {
        match self {
            Self::Months{every:_, day_of_month:_} => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub bspts: i32,
    pub pts_lost: i32,
    pub is_done: bool,
    pub days_to_next_reset: i64,
    pub next_reset: NaiveDate,
    pub frequency: TaskInterval,
    pub icon: TaskIcon,
}

impl Into<NewTask> for Task {
    fn into(self: Self) -> NewTask {
        NewTask {
            name: self.name.clone(),
            description: self.description.clone(),
            bspts: self.bspts,
            frequency: self.frequency.clone(), 
            icon: self.icon.clone(),
        }
    }
}