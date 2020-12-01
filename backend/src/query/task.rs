use diesel::prelude::*;
use data::task::*;
use data::user::Claim;
use chrono::{NaiveDate, Local, Duration, Datelike};
use crate::PgPooledConnection;
use crate::models;

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

fn query_task_to_task(qt: &models::QFullTask) -> Task {
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
        next_reset: qt.next_reset,
        frequency,
    }
}

/// Get all of the tasks for the user
pub fn get_tasks(user: Claim, conn: PgPooledConnection) -> Vec<Task> {
    use crate::schema::tasks::dsl::*;

    let q_tasks = tasks
        .filter(user_id.eq(user.id))
        .limit(5)
        .load::<models::QFullTask>(&conn)
        .expect("Error loading posts");
    q_tasks.iter().map(query_task_to_task).collect()
}

/// Add a new task to the database
pub fn commit_new_task(new_task: NewTask, user: Claim, conn: PgPooledConnection) -> Task {
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
    let full_task = models::InsertableTask {
        user_id: user.id,
        name: &new_task.name,
        description: &new_task.description,
        bspts: new_task.bspts,
        next_reset,
        every,
        time_unit,
        by_when,
    };
    
    let committed_task: models::QFullTask = diesel::insert_into(tasks::table)
        .values(full_task)
        .get_result(&conn)
        .expect("Error saving new post");

    query_task_to_task(&committed_task)
}