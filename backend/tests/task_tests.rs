mod setup;

use backend_lib::*;
use actix_web::{self, test, http::Method};
use data::user::*;
use data::task::*;
use data::icon::TaskIcon;
use setup::*;
use actix_web::http::StatusCode;

const STANDARD_TASK_FREQUENCY: u32 = 3;

/* HELPER FUNCTIONS */

async fn create_new_task(
    pool: &PgPool,
    ses: &actix_web::http::Cookie<'static>,
    name: &str,
    bspts: i32
) -> Task {
    println!("Creating task {}", name);
    let mut app = make_service(|c| {c.service(route::task::commit_new);}, &pool).await;
    let new_task = NewTask {
        name: name.to_string(),
        description: "".to_string(),
        bspts,
        frequency: TaskInterval::Days{every: STANDARD_TASK_FREQUENCY},
        icon: TaskIcon::default(),
    };
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .header("year", "2021")
        .header("month", "1")
        .header("day", "1")
        .uri("/task")
        .method(Method::POST)
        .cookie(ses.clone())
        .set_json(&new_task)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    println!("{:#?}", resp);
    assert!(resp.status().is_success());
    test::read_body_json(resp).await
}

async fn complete_task(
    pool: &PgPool,
    ses: &actix_web::http::Cookie<'static>,
    task: &Task,
) -> Result<Task, StatusCode> {
    complete_task_in_days(pool, ses, task, 0).await
}

async fn complete_task_in_days(
    pool: &PgPool,
    ses: &actix_web::http::Cookie<'static>,
    task: &Task,
    days_in_future: u32,
) -> Result<Task, StatusCode> {
    println!("Completing task {}", task.id);
    let mut app = make_service(|c| {
        c.service(route::task::complete);
        c.service(route::task::get_by_id);
    }, &pool).await;
    let complete_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri(format!("/task/complete/{id}", id = task.id).as_str())
        .method(Method::POST)
        .header("year", "2021")
        .header("month", "1")
        .header("day", (1 + days_in_future).to_string())
        .cookie(ses.clone())
        .to_request();
    let complete_resp = test::call_service(&mut app, complete_req).await;
    println!("{:#?}", complete_resp);
    if !complete_resp.status().is_success() {
        return Err(complete_resp.status());
    }
    let task: Task = test::read_body_json(complete_resp).await;
    Ok(task)
}

async fn undo_complete_task(
    pool: &PgPool,
    ses: &actix_web::http::Cookie<'static>,
    task: &Task,
    days_in_future: u32,
) -> Vec<Task> {
    println!("Completing task {}", task.id);
    let mut app = make_service(|c| {
        c.service(route::task::undo);
        c.service(route::task::get_by_id);
    }, &pool).await;
    let undo_req = test::TestRequest::with_header("content-type", "text/plain")
        .header("year", "2021")
        .header("month", "1")
        .header("day", (1 + days_in_future).to_string())
        .uri("/task/undo")
        .method(Method::POST)
        .cookie(ses.clone())
        .set_json(&task)
        .to_request();
    let undo_resp = test::call_service(&mut app, undo_req).await;
    println!("{:#?}", undo_resp);
    assert!(undo_resp.status().is_success(), "Bad return value");
    test::read_body_json(undo_resp).await
}

/* TESTS START HERE */

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
    // let mut app = make_service(|c| {c.service(route::task::commit_new);}, &pool).await;
    let task_name = "TaskName".to_string();
    let body = create_new_task(&pool, &session_cookie, &task_name, 1).await;
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
    let saved_task = create_new_task(&pool, &session_cookie, &task_name, 1).await;
    println!("Now make a call to update task's bspts");
    let task_id = saved_task.id;
    let mut new_task: NewTask = saved_task.into();
    let new_pts = 2;
    new_task.bspts = new_pts;
    let put_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri(format!("/task/{}", task_id).as_str())
        .method(Method::PUT)
        .cookie(session_cookie)
        .set_json(&new_task)
        .to_request();
    let put_resp = test::call_service(&mut app, put_req).await;
    println!("{:#?}", put_resp);
    println!("Ensure that the call returned OK");
    assert!(put_resp.status().is_success());
    println!("deserialize the body and check that the value of bspts changed");
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
    let saved_task = create_new_task(&pool, &session_cookie, "TaskName", 1).await;
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
async fn test_complete_task() {
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
    
    let saved_task = create_new_task(&pool, &session_cookie, "TaskName", 1).await;
    assert!(!saved_task.is_done, "The task should be marked as not-done for now");

    match complete_task(&pool, &session_cookie, &saved_task).await {
        Ok(task) => assert!(task.is_done, "Task should be done"),
        _ => panic!("Failed to complete task"),
    }

    match complete_task(&pool, &session_cookie, &saved_task).await {
        Err(code) => {
            if code != StatusCode::BAD_REQUEST {
                panic!("Trying to complete twice should trigger BAD REQUEST error not {}", code)
            }
        }
        _ => panic!("Should not be allowed to complete the same task twice")
    }

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
    assert!(in_list, format!("Can't find task with id: {}", saved_task.id));
}

#[actix_rt::test]
async fn test_point_loss() {
    println!("Setup point loss test");
    let user = make_user("point_loss");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    
    let saved_task = create_new_task(&pool, &session_cookie, "TaskName", 1).await;
    assert!(!saved_task.is_done, "The task should be marked as not-done for now");

    println!("Try to complete the task after it should be completed");
    let past_due_completion = complete_task_in_days(&pool, &session_cookie, &saved_task, STANDARD_TASK_FREQUENCY + 1).await;
    assert!(!past_due_completion.is_ok(), "Task was successfully marked complete after due by date")
}

#[actix_rt::test]
async fn undo_task() {
    println!("Setup undo test");
    let user = make_user("undo_task");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    
    let saved_task = create_new_task(&pool, &session_cookie, "TaskName", 1).await;
    assert!(!saved_task.is_done, "The task should be marked as not-done for now");

    assert!(
        complete_task(&pool, &session_cookie, &saved_task).await.is_ok(),
        "Could complete task",
    );

    println!("Now run undo with a time freq-1 day in the future, which should NOT trigger an undo");
    let tasks_in_0_days: Vec<Task> = undo_complete_task(
        &pool, &session_cookie, &saved_task, STANDARD_TASK_FREQUENCY - 1
    ).await;
    let mut in_list: bool = false;
    for task in tasks_in_0_days {
        if task.id == saved_task.id {
            in_list = true;
        }
    }
    assert!(!in_list, format!("Task {} should not have been undone yet.", saved_task.id));

    println!("Now run undo with a time freq days in the future, which SHOULD trigger an undo");
    let tasks_in_0_days: Vec<Task> = undo_complete_task(
        &pool, &session_cookie, &saved_task, STANDARD_TASK_FREQUENCY
    ).await;
    let mut in_list: bool = false;
    for task in tasks_in_0_days {
        if task.id == saved_task.id {
            in_list = true;
            if task.is_done {
                assert!(false, "This task should have been marked as not done");
            }
        }
    }
    assert!(in_list, format!("Task {} should have been undone.", saved_task.id));
}