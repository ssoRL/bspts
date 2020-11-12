#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

mod api;
mod models;
mod schema;

use warp::Filter;
use types::task::{Task, TaskInterval};
use crate::api::task::{get_tasks};

#[tokio::main]
async fn main() {

    let hi_route = warp::path!("hi").and(warp::get()).map(|| "Hi");

    let task_route = warp::path!("task").and(warp::get()).map(|| {
        let tasks_from_db = get_tasks();
        let tasks_for_transport: Vec<Task> = tasks_from_db.iter().map(|db_task| {
            let frequency =  match db_task.frequency.months {
                0 => TaskInterval::Days(db_task.frequency.days),
                _ => TaskInterval::Days(db_task.frequency.months),
            };
            Task {
                id: db_task.id,
                name: db_task.name.clone(),
                description: db_task.description.clone(),
                bspts: db_task.bspts,
                is_done: db_task.is_done,
                frequency,
                next_reset: db_task.next_reset
            }
        }).collect();
        let serialized_tasks = serde_json::to_string(&tasks_for_transport).unwrap();
        serialized_tasks
    });
    
    let file_server = warp::fs::dir("../site");

    warp::serve(hi_route.or(task_route).or(file_server))
        .run(([127, 0, 0, 1], 3030))
        .await;
}