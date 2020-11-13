use diesel::prelude::*;
use types::task::{Task, NewTask, TaskInterval};
use crate::models;
use diesel::pg::data_types::PgInterval;
use chrono::{Local, NaiveDate, Duration, Datelike};
use crate::PgPooledConnection;

fn convert_model_task_to_transport_task(model_task: &models::Task) -> Task {
    let freq =  match model_task.frequency.months {
        0 => TaskInterval::Days(model_task.frequency.days as u32),
        _ => TaskInterval::Months(model_task.frequency.months as u32),
    };
    Task {
        id: model_task.id,
        name: model_task.name.clone(),
        description: model_task.description.clone(),
        bspts: model_task.bspts,
        is_done: model_task.is_done,
        frequency: freq,
        next_reset: model_task.next_reset
    }
}

/// adds the frequency to the current date to get the next date this will trigger
fn calc_next_reset(current_reset_date: NaiveDate, frequency: TaskInterval) -> NaiveDate {
    match frequency {
        TaskInterval::Days(days) => {
            let duration = Duration::days(days.into());
            let new_date_option = current_reset_date.checked_add_signed(duration);
            match new_date_option {
                Some(date) => date,
                // This happens only in case of an overflow circa 200,000 CE
                None => current_reset_date,
            }
        },
        TaskInterval::Months(months) => {
            let current_month = current_reset_date.month();
            let new_month_unmodded = current_month + months;
            let new_month = new_month_unmodded % 12;
            let new_year = current_reset_date.year() + (new_month_unmodded / 12) as i32;
            NaiveDate::from_ymd(new_year, new_month, current_reset_date.day())
        },
    }
}

/// Get all of the tasks for the user
pub fn get_tasks(conn: PgPooledConnection) -> Vec<Task> {
    use crate::schema::tasks::dsl::*;

    let model_tasks = tasks.limit(5).load::<models::Task>(&conn).expect("Error loading posts");
    model_tasks.iter().map(convert_model_task_to_transport_task).collect()
}

/// Add a new task to the database
pub fn commit_new_task(new_task: NewTask, conn: PgPooledConnection) -> Task {
    use crate::schema::tasks;

    // First add the required fields to save the task
    let frequency = match new_task.frequency {
        TaskInterval::Days(days) => PgInterval::from_days(days as i32),
        TaskInterval::Months(months) => PgInterval::from_months(months as i32),
    };
    let today = Local::today().naive_local();
    let next_reset = calc_next_reset(today, new_task.frequency);
    let full_task = models::NewTask {
        name: &new_task.name,
        description: &new_task.description,
        bspts: new_task.bspts,
        next_reset,
        frequency,
    };
    let committed_task: models::Task = diesel::insert_into(tasks::table)
        .values(&full_task)
        .get_result(&conn)
        .expect("Error saving new post");
    convert_model_task_to_transport_task(&committed_task)
}