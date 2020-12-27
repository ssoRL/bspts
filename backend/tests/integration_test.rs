use backend_lib::*;
use actix_web::{self, http, test, dev, App, http::Method, web::ServiceConfig};
use actix_http;
use data::user::*;
use data::task::*;
use actix_session::{CookieSession};
use chrono::Local;

/// Creates a NewUser object by appending the formatted time to a given prefix
/// this ensures that no two test users will share a name
fn make_user(user_name_prefix: &str) -> NewUser {
    let user_name = format!("{}-{}", user_name_prefix, Local::now().format("%Y-%m-%d-%H-%M-%S"));
    NewUser {
        uname: user_name,
        password: "pw1".to_string(),
    }
}

/// Makes a new app service with the specified routes
async fn make_service<F>(
    config: F,
    pool: &PgPool
) -> impl dev::Service<Request = actix_http::Request, Response = dev::ServiceResponse<dev::Body>, Error = actix_web::Error>
where
    // F: dev::HttpServiceFactory + 'static
    F: FnOnce(&mut ServiceConfig)
{
    // Creates the service pool with both the sign in route to auth with
    // and the route passed as a parameter
    test::init_service(
        App::new()
            .data(pool.clone())
            .wrap(
                CookieSession::signed(&[0; 32]).secure(false)
            )
            .configure(config)
    ).await
}

/// Creates a new user with the given uname,
/// then logs that user in and returns the session cookie
async fn login(user: &NewUser, pool: &PgPool) -> Option<http::Cookie<'static>> {
    // Create a new user account by calling the query directly (not through http)
    let conn = pool.get().expect("Could not get connection from pool");
    let _ = query::user::save_new_user(&user, &conn);
    // Then sign in by calling the sign in route
    let mut app = make_service(|c| {c.service(route::user::sign_in);}, &pool).await;
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/login")
        .method(Method::POST)
        .set_json(&user)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    // pull the cookies out of the response
    let response = resp.response();
    let cookies = response.cookies();
    // Find the cookie named "actix-session" and return it
    let session_cookie = cookies.into_iter().filter(|cookie| {
        cookie.name() == "actix-session"
    }).last()?;
    Some(session_cookie.into_owned())
}

#[actix_rt::test]
async fn sign_up_user() {
    let pool = get_connection_pool();
    let user = make_user("sign_up_user");
    println!("u: {:#?}", user);
    let mut app = make_service(|c| {c.service(route::user::sign_up);}, &pool).await;
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/user")
        .method(Method::POST)
        .set_json(&user)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    println!("{:#?}", resp);
    assert!(resp.status().is_success());
}

// TODO: test login fails for uname and pw

#[actix_rt::test]
async fn get_tasks() {
    // create a new user and log in
    let user = make_user("get_tasks");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    let mut app = make_service(|c| {c.service(route::task::get_todo);}, &pool).await;
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/task/todo")
        .method(Method::GET)
        .cookie(session_cookie)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    println!("{:#?}", resp);
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn add_task() {
    let user = make_user("add_task");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    let mut app = make_service(|c| {c.service(route::task::commit_new);}, &pool).await;
    let task_name = "TaskName".to_string();
    let new_task = NewTask {
        name: task_name.clone(),
        description: "".to_string(),
        bspts: 1,
        frequency: TaskInterval::Days{every: 3},
    };
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/task")
        .method(Method::POST)
        .cookie(session_cookie)
        .set_json(&new_task)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    println!("{:#?}", resp);
    // First ensure that the call returned OK
    assert!(resp.status().is_success());
    // Then deserialize the body and check that the returned name is the same name as was added
    let body: Task = test::read_body_json(resp).await;
    assert_eq!(body.name, task_name);
}

#[actix_rt::test]
async fn update_task() {
    let user = make_user("update_task");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    let mut app = make_service(
        |c| {
            c.service(route::task::commit_new);
            c.service(route::task::update);
        },
        &pool
    ).await;
    let task_name = "TaskName".to_string();
    let mut task = NewTask {
        name: task_name.clone(),
        description: "".to_string(),
        bspts: 1,
        frequency: TaskInterval::Days{every: 3},
    };
    let set_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/task")
        .method(Method::POST)
        .cookie(session_cookie.clone())
        .set_json(&task)
        .to_request();
    let set_resp = test::call_service(&mut app, set_req).await;
    println!("{:#?}", set_resp);
    // First ensure that the call returned OK
    assert!(set_resp.status().is_success());
    let saved_task: Task = test::read_body_json(set_resp).await;
    // Now make a call to update the task
    // First change the number of points
    let new_pts = 2;
    task.bspts = new_pts;
    let put_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri(format!("/task/{}", saved_task.id).as_str())
        .method(Method::PUT)
        .cookie(session_cookie)
        .set_json(&task)
        .to_request();
    let put_resp = test::call_service(&mut app, put_req).await;
    println!("{:#?}", put_resp);
    // First ensure that the call returned OK
    assert!(put_resp.status().is_success());
    // Then deserialize the body and check that the
    // value of bspts changed
    let body: Task = test::read_body_json(put_resp).await;
    assert_eq!(body.bspts, new_pts);
}

#[actix_rt::test]
async fn delete_task() {
    let user = make_user("delete_task");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    let mut app = make_service(
        |c| {
            c.service(route::task::get_todo);
            c.service(route::task::commit_new);
            c.service(route::task::delete);
        },
        &pool
    ).await;
    let task = NewTask {
        name: "TaskName".to_string(),
        description: "".to_string(),
        bspts: 1,
        frequency: TaskInterval::Days{every: 3},
    };
    let set_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/task")
        .method(Method::POST)
        .cookie(session_cookie.clone())
        .set_json(&task)
        .to_request();
    let set_resp = test::call_service(&mut app, set_req).await;
    println!("{:#?}", set_resp);
    // First ensure that the call returned OK
    assert!(set_resp.status().is_success());
    let saved_task: Task = test::read_body_json(set_resp).await;
    // Now make a call to delete the task
    let delete_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri(format!("/task/{}", saved_task.id).as_str())
        .method(Method::DELETE)
        .cookie(session_cookie.clone())
        .to_request();
    let delete_resp = test::call_service(&mut app, delete_req).await;
    println!("{:#?}", delete_resp);
    // Ensure that the delete call returned OK
    assert!(delete_resp.status().is_success());
    // Finally get the whole list of tasks and make sure the
    // deleted one is not included
    let get_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/task/todo")
        .method(Method::GET)
        .cookie(session_cookie)
        .to_request();
    let get_resp = test::call_service(&mut app, get_req).await;
    assert!(get_resp.status().is_success());
    let tasks: Vec<Task> = test::read_body_json(get_resp).await;
    for task in tasks {
        if task.id == saved_task.id {
            panic!(format!(
                "Failed to delete task with id {} for user {}",
                task.id,
                user.uname,
            ))
        }
    }
}

#[actix_rt::test]
async fn complete_task() {
    println!("Setup complete test");
    let user = make_user("complete_task");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    let mut app = make_service(
        |c| {
            c.service(route::user::get_user);
            c.service(route::task::get_done);
            c.service(route::task::get_by_id);
            c.service(route::task::commit_new);
            c.service(route::task::complete);
        },
        &pool
    ).await;
    let task_points = 1;

    println!("Create the task");
    let task = NewTask {
        name: "TaskName".to_string(),
        description: "".to_string(),
        bspts: task_points,
        frequency: TaskInterval::Days{every: 3},
    };
    let set_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/task")
        .method(Method::POST)
        .cookie(session_cookie.clone())
        .set_json(&task)
        .to_request();
    let set_resp = test::call_service(&mut app, set_req).await;
    println!("{:#?}", set_resp);
    assert!(set_resp.status().is_success());
    let saved_task: Task = test::read_body_json(set_resp).await;
    assert!(!saved_task.is_done, "The task should be marked as not-done for now");

    println!("Now make a call to mark the task complete");
    let complete_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri(format!("/task/complete/{id}", id = saved_task.id).as_str())
        .method(Method::POST)
        .cookie(session_cookie.clone())
        .to_request();
    let complete_resp = test::call_service(&mut app, complete_req).await;
    println!("{:#?}", complete_resp);
    assert!(complete_resp.status().is_success());
    println!("Get the task and ensure it's now marked completed");
    let get_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri(format!("/task/{}", saved_task.id).as_str())
        .method(Method::GET)
        .cookie(session_cookie.clone())
        .to_request();
    let get_resp = test::call_service(&mut app, get_req).await;
    println!("{:#?}", get_resp);
    assert!(get_resp.status().is_success());
    let task: Task = test::read_body_json(get_resp).await;
    assert!(task.is_done);
    println!("Get the user and ensure they got their bs points");
    let get_user_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/user")
        .method(Method::GET)
        .cookie(session_cookie.clone())
        .to_request();
    let get_user_resp = test::call_service(&mut app, get_user_req).await;
    println!("{:#?}", get_user_resp);
    assert!(get_user_resp.status().is_success());
    let user: User = test::read_body_json(get_user_resp).await;
    assert_eq!(user.bspts, task_points);
    println!("Get the list of completed tasks and ensure this task is on there");
    let get_done_tasks_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/task/done")
        .method(Method::GET)
        .cookie(session_cookie.clone())
        .to_request();
    let get_tasks_done_resp = test::call_service(&mut app, get_done_tasks_req).await;
    println!("{:#?}", get_tasks_done_resp);
    assert!(get_tasks_done_resp.status().is_success());
    let tasks: Vec<Task> = test::read_body_json(get_tasks_done_resp).await;
    let mut in_list: bool = false;
    for task in tasks {
        if task.id == saved_task.id {
            in_list = true;
        }
    }
    if !in_list {
        panic!(format!("Can't find task with id: {}", saved_task.id))
    }
}