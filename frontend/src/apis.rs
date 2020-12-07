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
pub fn is_signed_in() -> bool {
    // There will be a user-name value in local storage if the user has signed in
    // and gotten back a session cookie
    let stor = StorageService::new(Area::Local).expect("Could not connect to the local storage");
    let auth_from_storage: Result<String, _> = stor.restore(UNAME_KEY);
    auth_from_storage.is_ok()
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
pub fn sign_in_frontend(user_name: String) {
    let mut stor = StorageService::new(Area::Local).expect("Could not connect to the local storage");
    stor.store(UNAME_KEY, Ok(user_name));
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

/// Get a list of all of the tasks
pub fn get_tasks(callback: FetchCallback<Vec<Task>>) -> FetchTask {
    let get = Request::get("/task")
        .header("auth", "")
        .body(Nothing)
        .unwrap();
    FetchService::fetch(get, callback).unwrap()
}

/// Commits a new task to 
pub fn commit_new_task(new_task: NewTask, callback: FetchCallback<Task>) -> FetchTask {
        let post = Request::post("/task")
            .header("Content-Type", "application/json")
            .header("auth", "")
            .body(Json(&new_task))
            .unwrap();
        FetchService::fetch(post, callback).unwrap()
}