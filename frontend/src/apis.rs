use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{
    FetchService,
    FetchTask,
    Request,
    Response,
};
use http::request::Builder;
use data::{
    task::*,
    user::*,
    reward::*
};
use crate::app;
use yew_router::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
use js_sys::Date;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

fn get_with_head(route: &str) -> Builder {
    let get = Request::get(route);
    add_headers(get)
}

fn post_with_head(route: &str) -> Builder {
    let post = Request::post(route);
    add_headers(post)
}

fn put_with_head(route: &str) -> Builder {
    let put = Request::put(route);
    add_headers(put)
}

fn add_headers(request: Builder) -> Builder {
    let today = Date::new_0();
    request
        .header("Content-Type", "application/json")
        .header("year", today.get_full_year())
        .header("month", today.get_month() + 1)
        .header("day", today.get_date())
}

/// Gets the user state
pub fn get_user(callback: FetchCallback<Option<User>>) -> FetchTask {
    let get = Request::get("/user")
        .body(Nothing)
        .unwrap();
    FetchService::fetch(get, callback).unwrap()
}

/// Remove the user's id from local storage to
/// mark them as unauthorized, then re-routes to
/// the home page
pub fn sign_out_frontend() {
    // TODO: Make a call to invalidate the ses cookie on backend
    // let mut stor = StorageService::new(Area::Local).expect("Could not connect to the local storage");
    // stor.remove(UNAME_KEY);
    let mut agent_dispatch: RouteAgentDispatcher<()> = RouteAgentDispatcher::default();
    agent_dispatch.send(ChangeRoute(app::Route::HomePage.into()));
}

/// Sends the user's creds to the backend, which responds with an error, or a jwt
pub fn sign_in(user: NewUser, callback: FetchCallback<User>) -> FetchTask  {
    let login = post_with_head("/login")
        .body(Json(&user))
        .expect("Failed to create ");
    FetchService::fetch(login, callback).expect("Failed to send POST request to /login")
}

/// Creates a new user account and returns the jwt used to auth the user
pub fn sign_up(new_user: NewUser, callback: FetchCallback<User>) -> FetchTask  {
    let signup = post_with_head("/user")
        .body(Json(&new_user))
        .unwrap();
    FetchService::fetch(signup, callback).unwrap()
}

/// Get a list of all of the tasks not yet completed
pub fn get_todo_tasks(callback: FetchCallback<Vec<Task>>) -> FetchTask {
    let get = get_with_head("/task/todo").body(Nothing).unwrap();
    FetchService::fetch(get, callback).unwrap()
}

/// Get a list of all of the completed tasks
pub fn get_done_tasks(callback: FetchCallback<Vec<Task>>) -> FetchTask {
    let get = get_with_head("/task/done").body(Nothing).unwrap();
    FetchService::fetch(get, callback).unwrap()
}

/// Get a list of all of the completed tasks
pub fn undo_done_tasks(callback: FetchCallback<Vec<Task>>) -> FetchTask {
    let get = post_with_head("/task/undo").body(Nothing).unwrap();
    FetchService::fetch(get, callback).unwrap()
}

/// Commits to a new task
pub fn commit_new_task(new_task: NewTask, callback: FetchCallback<Task>) -> FetchTask {
        let post = post_with_head("/task")
            .body(Json(&new_task))
            .unwrap();
        FetchService::fetch(post, callback).unwrap()
}

/// Update a task
pub fn update_task(task_id: i32, task_edits: NewTask, callback: FetchCallback<Task>) -> FetchTask {
        let update = put_with_head(&format!("/task/{}", task_id))
            .body(Json(&task_edits))
            .unwrap();
        FetchService::fetch(update, callback).unwrap()
}

/// Mark a task as completed
pub fn complete_task(task_id: i32, callback: FetchCallback<Task>) -> FetchTask {
        let post = post_with_head(&format!("/task/complete/{}", task_id))
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

pub fn get_rewards(callback: FetchCallback<Vec<Reward>>) -> FetchTask {
    let get = Request::get("/reward")
        .body(Nothing)
        .unwrap();
    FetchService::fetch(get, callback).unwrap()
}

pub fn new_reward(new_reward: &NewReward, callback: FetchCallback<Reward>) -> FetchTask {
        let post = post_with_head("/reward")
            .body(Json(new_reward))
            .unwrap();
        FetchService::fetch(post, callback).unwrap()
}

/// You've done the reward. Probably well deserved!
pub fn do_reward(reward_id: i32, callback: FetchCallback<i32>) -> FetchTask {
        let post = post_with_head(&format!("/reward/do/{}", reward_id))
            .body(Nothing)
            .unwrap();
        FetchService::fetch(post, callback).unwrap()
}

pub fn update_reward(reward_id: i32, reward_edits: NewReward, callback: FetchCallback<Reward>) -> FetchTask {
        let update = put_with_head(&format!("/reward/{}", reward_id))
            .body(Json(&reward_edits))
            .unwrap();
        FetchService::fetch(update, callback).unwrap()
}

pub fn delete_reward(reward_id: i32, callback: FetchCallback<()>) -> FetchTask {
        let delete = Request::delete(format!("/reward/{}", reward_id))
            .body(Nothing)
            .unwrap();
        FetchService::fetch(delete, callback).unwrap()
}