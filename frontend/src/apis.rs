use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::storage::{StorageService, Area};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use data::task::*;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;
pub enum BsptsFetchError {
    NotAuthed,
    TransportError,
}
type BsptsFetchResult = Result<FetchTask, BsptsFetchError>;

pub fn get_jwt() -> Option<String>  {
    let stor = StorageService::new(Area::Local).expect("Could not connect to the local storage");
    let auth_from_storage: Result<String, _> = stor.restore("auth-token");
    match auth_from_storage {
        Ok(jwt) => {
            Some(jwt)
        },
        Err(_) => {
            None
        }
    }
}

/// Get a list of all of the tasks
pub fn get_tasks(callback: FetchCallback<Vec<Task>>) -> BsptsFetchResult {
    match get_jwt() {
        Some(jwt) => {
            let get = Request::get("/task")
                .body(Nothing)
                .unwrap();
            let fetch = FetchService::fetch(get, callback).unwrap();
            Ok(fetch)
        },
        None => {
            Err(BsptsFetchError::NotAuthed)
        }
    }
}

/// Commits a new task to 
pub fn commit_new_task(new_task: NewTask, callback: FetchCallback<Task>) -> FetchTask {
    //match get_jwt() {
        // Some(jwt) => {
        //     let post = Request::post("/task")
        //         .header("Content-Type", "application/json")
        //         .header("auth", jwt)
        //         .body(Json(&new_task))
        //         .unwrap();
        //     let fetch = FetchService::fetch(post, callback).unwrap();
        //     Ok(fetch)
        // },
        // None => {
        //     Err(BsptsFetchError::NotAuthed)
        // }
        //}
        let post = Request::post("/task")
            .header("Content-Type", "application/json")
            .body(Json(&new_task))
            .unwrap();
        let fetch = FetchService::fetch(post, callback).unwrap();
        fetch
}