use warp::Filter;
use types::task::{Task};

#[tokio::main]
async fn main() {

    let hi_route = warp::path!("hi").and(warp::get()).map(|| "Hi");

    let task_route = warp::path!("task").and(warp::get()).map(|| {
        let tasks = vec! [
            Task {
                id: 0,
                name: String::from("Write"),
                description: String::from("Write at least for 30 minutes"),
                checked: false,
                daily_loss: 3
            }
        ];
        let serialized_tasks = serde_json::to_string(&tasks).unwrap();
        serialized_tasks
    });
    
    let file_server = warp::fs::dir("../site");

    warp::serve(hi_route.or(task_route).or(file_server))
        .run(([127, 0, 0, 1], 3030))
        .await;
}