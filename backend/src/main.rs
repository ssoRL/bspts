#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

mod api;
mod models;
mod schema;

use warp::Filter;
use types::task::{NewTask};
use crate::api::task::{get_tasks, commit_new_task};

#[tokio::main]
async fn main() {

    let task_route = warp::path!("task").and(warp::get()).map(|| {
        let tasks = get_tasks();
        let serialized_tasks = serde_json::to_string(&tasks).unwrap();
        serialized_tasks
    });
        
    let commit_new_task_route = warp::path!("task")
        .and(warp::post())
        .and(warp::body::json())
        .map(|new_task: NewTask| {
            let committed_task = commit_new_task(new_task);
            let serialized_task = serde_json::to_string(&committed_task).unwrap();
            serialized_task
        });
    
    let file_server = warp::fs::dir("../site");

    warp::serve(task_route.or(commit_new_task_route).or(file_server))
        .run(([127, 0, 0, 1], 3030))
        .await;
}