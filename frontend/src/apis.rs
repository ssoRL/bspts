use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::{
    storage::{StorageService, Area},
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService
};
use data::task::*;
use data::user::*;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

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

pub fn set_jwt(jwt: String)  {
    ConsoleService::log(format!("Saving: {}", &jwt).as_str());
    let mut stor = StorageService::new(Area::Local).expect("Could not connect to the local storage");
    stor.store("auth-token", Ok(jwt));
}

/// Creates a new user account and returns the jwt used to auth the user
pub fn signup(new_user: NewUser, callback: FetchCallback<String>) -> FetchTask  {
    let post = Request::post("/user")
        .header("Content-Type", "application/json")
        .body(Json(&new_user))
        .unwrap();
    FetchService::fetch(post, callback).unwrap()
}

/// Get a list of all of the tasks
pub fn get_tasks(callback: FetchCallback<Vec<Task>>, jwt: &String) -> FetchTask {
    let get = Request::get("/task")
        .header("auth", jwt)
        .body(Nothing)
        .unwrap();
    FetchService::fetch(get, callback).unwrap()
}

/// Commits a new task to 
pub fn commit_new_task(new_task: NewTask, callback: FetchCallback<Task>, jwt: &String) -> FetchTask {
        let post = Request::post("/task")
            .header("Content-Type", "application/json")
            .header("auth", jwt)
            .body(Json(&new_task))
            .unwrap();
        FetchService::fetch(post, callback).unwrap()
}