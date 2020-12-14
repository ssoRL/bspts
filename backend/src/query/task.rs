use diesel::prelude::*;
use data::task::*;
use chrono::{NaiveDate, Local, Duration, Datelike};
use crate::PgPooledConnection;
use crate::models::*;
use actix_web::{Result, error, http::StatusCode};

pub const DAYS: &str = "Days";
pub const WEEKS: &str = "Weeks";
pub const MONTHS: &str = "Months";

/// adds the frequency to the current date to get the next date this will trigger
fn calc_next_reset(frequency: &TaskInterval) -> NaiveDate {
    let today = Local::today().naive_local();
    match frequency {
        TaskInterval::Days{every} => {
            let duration = Duration::days(*every as i64);
            today.checked_add_signed(duration).unwrap()
        },
        TaskInterval::Weeks{every, weekday} => {
            let current_day_of_week = today.weekday().num_days_from_monday();
            let days_to_jump: u32 = if *weekday > current_day_of_week {
                // The day this task must be done by is later this week
                weekday - current_day_of_week
            } else {
                // This day is on the following week
                (weekday + 7) - current_day_of_week
            };
            // Add on enough weeks to compensate for the every x weeks param
            let days_to_jump_plus_weeks = if *every > 1 {
                days_to_jump + (7 * every)
            } else {
                days_to_jump
            };
            let duration = Duration::days(days_to_jump_plus_weeks as i64);
            today.checked_add_signed(duration).unwrap()
        },
        TaskInterval::Months{every, day_of_month} => {
            let current_month = today.month0();
            let new_month_no_mod = current_month + every;
            // The month 0 indexed (requires 1 indexed for from_ymd fn)
            let new_month0 = new_month_no_mod % 12;
            let new_year = today.year() + (new_month_no_mod / 12) as i32;
            NaiveDate::from_ymd(new_year, new_month0 + 1, *day_of_month)
        }
    }
}

fn get_days_to_next_reset(next_reset: NaiveDate) -> i64 {
    let today = Local::today().naive_local();
    let duration = next_reset - today;
    duration.num_days()
}

fn query_task_to_task(qt: &QTask) -> Task {
    let frequency = match qt.time_unit.as_str() {
        DAYS => {
            TaskInterval::Days{every: qt.every as u32}
        },
        WEEKS => {
            TaskInterval::Weeks{every: qt.every as u32, weekday: qt.by_when as u32}
        },
        MONTHS => {
            TaskInterval::Months{every: qt.every as u32, day_of_month: qt.by_when as u32}
        },
        _ => {
            panic!(format!("Could not create a task interval for {}", qt.time_unit))
        }
    };

    Task {
        id: qt.id,
        name: qt.name.clone(),
        description: qt.description.clone(),
        bspts: qt.bspts,
        is_done: qt.is_done,
        days_to_next_reset: get_days_to_next_reset(qt.next_reset),
        next_reset: qt.next_reset,
        frequency,
    }
}

/// Get all of the tasks for the user
pub fn get_tasks(user: QUser, conn: &PgPooledConnection) -> Vec<Task> {
    let q_tasks = QTask::belonging_to(&user)
        .load::<QTask>(conn)
        .expect("Error loading tasks");
    q_tasks.iter().map(query_task_to_task).collect()
}

fn get_q_task(task_id: i32, conn: &PgPooledConnection) -> Option<QTask> {
    use crate::schema::tasks::dsl::*;

    let mut q_task = tasks
        .filter(id.eq(task_id))
        .load::<QTask>(conn)
        .expect("Error getting users");
    // Should be a vec of only one item, return that item
    q_task.pop()
}

/// Add a new task to the database
pub fn commit_new_task(new_task: NewTask, user: QUser, conn: PgPooledConnection) -> Task {
    use crate::schema::tasks;

    let next_reset = calc_next_reset(&new_task.frequency);
    let (time_unit, every, by_when) = match new_task.frequency {
        TaskInterval::Days{every} => {
            (DAYS, every as i32, 0)
        },
        TaskInterval::Weeks{every, weekday} => {
            (WEEKS, every as i32, weekday as i32)
        },
        TaskInterval::Months{every, day_of_month} => {
            (MONTHS, every as i32, day_of_month as i32)
        }
    };
    let full_task = InsertableTask {
        user_id: user.id,
        name: &new_task.name,
        description: &new_task.description,
        bspts: new_task.bspts,
        next_reset,
        every,
        time_unit,
        by_when,
    };
    
    let committed_task: QTask = diesel::insert_into(tasks::table)
        .values(full_task)
        .get_result(&conn)
        .expect("Error saving new task");

    query_task_to_task(&committed_task)
}

/// Add a new task to the database
pub fn update_task(task_id: i32, new_task: NewTask, conn: &PgPooledConnection) -> Result<Task> {
    use crate::schema::tasks::dsl::tasks;

    let mut q_task: QTask = match get_q_task(task_id, &conn) {
        Some(q) => q,
        None => {
            let error = error::InternalError::new(
                "There's no task with that id".to_string(),
                StatusCode::NOT_FOUND
            );
            return Err(error.into())
        }
    };

    let (time_unit, every, by_when) = match new_task.frequency {
        TaskInterval::Days{every} => {
            (DAYS, every as i32, 0)
        },
        TaskInterval::Weeks{every, weekday} => {
            (WEEKS, every as i32, weekday as i32)
        },
        TaskInterval::Months{every, day_of_month} => {
            (MONTHS, every as i32, day_of_month as i32)
        }
    };

    q_task.name = new_task.name;
    q_task.description = new_task.description;
    q_task.bspts = new_task.bspts;
    q_task.every = every;
    q_task.time_unit = time_unit.to_string();
    q_task.by_when = by_when;
    
    let committed_task: QTask = diesel::update(tasks.find(task_id))
        .set(&q_task)
        .get_result(conn)
        .expect("Error updating task");

    Ok(query_task_to_task(&committed_task))
}