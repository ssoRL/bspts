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
use crate::app;
use yew_router::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

const UNAME_KEY: &str = "user-name";

/// Check if the user has signed in
pub fn get_stored_user() -> Option<User> {
    // There will be a user-name value in local storage if the user has signed in
    // and gotten back a session cookie
    let stor = StorageService::new(Area::Local).expect("Could not connect to the local storage");
    let stored_string: Result<String, anyhow::Error> = stor.restore::<>(UNAME_KEY);
    let serial = stored_string.ok()?;
    let user: User = serde_json::from_str(&serial).ok()?;
    Some(user)
}

/// Remove the user's id from local storage to
/// mark them as unauthorized, then re-routes to
/// the home page
pub fn sign_out_frontend() {
    let mut stor = StorageService::new(Area::Local).expect("Could not connect to the local storage");
    stor.remove(UNAME_KEY);
    let mut agent_dispatch: RouteAgentDispatcher<()> = RouteAgentDispatcher::default();
    agent_dispatch.send(ChangeRoute(app::Route::HomePage.into()));
}

/// Sets the user-name field in local storage.
/// This communicates to the router that the user is
/// signed in. Do this instead of checking the session
/// cookie since yew doesn't have a cookie service.
/// Then route the use to the home route
pub fn store_user(user: User) {
    let mut stor = StorageService::new(Area::Local).expect("Could not connect to the local storage");
    // let serial = serde_json::to_string(&user).map_err(|e| anyhow::Error::new(e));
    // let dump = Json(&user);
    stor.store(UNAME_KEY, Json(&user));
    let mut agent_dispatch: RouteAgentDispatcher<()> = RouteAgentDispatcher::default();
    agent_dispatch.send(ChangeRoute(app::Route::HomePage.into()));
}

/// Sends the user's creds to the backend, which responds with an error, or a jwt
pub fn sign_in(user: NewUser, callback: FetchCallback<User>) -> FetchTask  {
    let login = Request::post("/login")
        .header("Content-Type", "application/json")
        .body(Json(&user))
        .expect("Failed to create ");
    FetchService::fetch(login, callback).expect("Failed to send POST request to /login")
}

/// Creates a new user account and returns the jwt used to auth the user
pub fn sign_up(new_user: NewUser, callback: FetchCallback<User>) -> FetchTask  {
    let post = Request::post("/user")
        .header("Content-Type", "application/json")
        .body(Json(&new_user))
        .unwrap();
    FetchService::fetch(post, callback).unwrap()
}

/// Get a list of all of the tasks not yet completed
pub fn get_todo_tasks(callback: FetchCallback<Vec<Task>>) -> FetchTask {
    let get = Request::get("/task/todo")
        .body(Nothing)
        .unwrap();
    FetchService::fetch(get, callback).unwrap()
}

/// Get a list of all of the completed tasks
pub fn get_done_tasks(callback: FetchCallback<Vec<Task>>) -> FetchTask {
    let get = Request::get("/task/done")
        .body(Nothing)
        .unwrap();
    FetchService::fetch(get, callback).unwrap()
}

/// Commits to a new task
pub fn commit_new_task(new_task: NewTask, callback: FetchCallback<Task>) -> FetchTask {
        let post = Request::post("/task")
            .header("Content-Type", "application/json")
            .body(Json(&new_task))
            .unwrap();
        FetchService::fetch(post, callback).unwrap()
}

/// Update a task
pub fn update_task(task_id: i32, task_edits: NewTask, callback: FetchCallback<Task>) -> FetchTask {
        let post = Request::put(format!("/task/{}", task_id))
            .header("Content-Type", "application/json")
            .body(Json(&task_edits))
            .unwrap();
        FetchService::fetch(post, callback).unwrap()
}

/// Mark a task as completed
pub fn complete_task(task_id: i32, callback: FetchCallback<i32>) -> FetchTask {
        let post = Request::post(format!("/task/complete/{}", task_id))
            .header("Content-Type", "application/json")
            .body(Nothing)
            .unwrap();
        FetchService::fetch(post, callback).unwrap()
}

pub fn delete_task(task_id: i32, callback: FetchCallback<()>) -> FetchTask {
        let delete = Request::delete(format!("/task/{}", task_id))
            .body(Nothing)
            .unwrap();
        FetchService::fetch(delete, callback).unwrap()
}