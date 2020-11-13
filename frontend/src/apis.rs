use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use types::task::{Task, NewTask};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

/// Get a list of all of the tasks
pub fn get_tasks(callback: FetchCallback<Vec<Task>>) -> FetchTask {
    // Send out the fetch to populate the word from hi
    let get = Request::get("/task").body(Nothing).unwrap();
    FetchService::fetch(get, callback).unwrap()
}

/// Commits a new task to 
pub fn commit_new_task(new_task: NewTask, callback: FetchCallback<Task>) -> FetchTask {
    // Send out the fetch to populate the word from hi
    let body_serialized = serde_json::to_string(&new_task);
    // TODO: Is this really the only way??
    let dumb = Ok(body_serialized.unwrap());

    let post_new_task = Request::post("/task")
        .header("Content-Type", "application/json")
        .body(dumb)
        .unwrap();
    FetchService::fetch(post_new_task, callback).unwrap()
}