use actix_web::{
    get,
    delete,
    post,
    put,
    error,
    http::StatusCode,
    web::{self, Data, Json}
};
use data::task::*;
use data::user::*;
use crate::query::task::*;
use crate::query::user::*;
use crate::query::session::*;
use crate::models;
use actix_session::{Session};
use crate::{PgPool, PgPooledConnection};

type Rsp<T> = actix_web::Result<Json<T>>;

const SESSION_ID_KEY: &str = "session_id";

/// Runs the provided function after checking the user's session is OK
fn with_auth<T, F>(ses: Session, data: Data<PgPool>, run: F)-> Rsp<T>
where
    F: FnOnce(models::QUser, PgPooledConnection) -> Rsp<T>
{
    let pool = data.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let session_cookie = ses.get::<i32>(SESSION_ID_KEY);
    match session_cookie {
        Ok(Some(session_id)) => {
            let user = get_session_user(session_id, &conn)?;
            run(user, conn)
        }
        _ => {
            let error = error::InternalError::new("Could not get session", StatusCode::UNAUTHORIZED);
            Err(error.into())
        }
    }
}

#[post("/login")]
async fn sign_in_route(payload: Json<NewUser>, database: Data<PgPool>, ses: Session) -> Rsp<User>  {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let Json(new_user) = payload;
    let user = login_user(new_user, &conn)?;
    let new_session = start_session(&user, &conn);
    ses.set(SESSION_ID_KEY, new_session.id)?;
    Ok(Json(User {uname: user.uname, bspts: user.bspts}))
}

#[get("/user")]
async fn get_user_route(data: Data<PgPool>, ses: Session) -> Rsp<User> {
    with_auth(ses, data, |user, _| {
        Ok(Json(User {uname: user.uname, bspts: user.bspts}))
    })
}

#[post("/user")]
async fn sign_up_route(payload: Json<NewUser>, database: Data<PgPool>, ses: Session) -> Rsp<User> {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let Json(new_user) = payload;
    let user = save_new_user(&new_user, &conn)?;
    let new_session = start_session(&user, &conn);
    ses.set(SESSION_ID_KEY, new_session.id)?;
    Ok(Json(User {uname: user.uname, bspts: user.bspts}))
}

#[get("/task/todo")]
async fn get_todo_tasks_route(data: Data<PgPool>, ses: Session) -> Rsp<Vec<Task>> {
    with_auth(ses, data, |user, conn| {
        let tasks = get_tasks(user, false, &conn);
        Ok(Json(tasks))
    })
}

#[get("/task/done")]
async fn get_done_tasks_route(data: Data<PgPool>, ses: Session) -> Rsp<Vec<Task>> {
    with_auth(ses, data, |user, conn| {
        let tasks = get_tasks(user, true, &conn);
        Ok(Json(tasks))
    })
}

#[get("/task/{id}")]
async fn get_task_route(web::Path(id): web::Path<i32>, data: Data<PgPool>, ses: Session) -> Rsp<Task> {
    with_auth(ses, data, |_, conn| {
        let task = get_task(id, &conn)?;
        Ok(Json(task))
    })
}

#[post("/task")]
async fn commit_new_task_route(payload: Json<NewTask>, data: Data<PgPool>, ses: Session) -> Rsp<Task> {
    with_auth(ses, data, |user, conn| {
        let Json(new_task) = payload;
        let committed_task = commit_new_task(new_task, user, conn);
        Ok(Json(committed_task))
    })
}

#[put("/task/{id}")]
async fn update_task_route(
    web::Path(id): web::Path<i32>,
    payload: Json<NewTask>,
    data: Data<PgPool>,
    ses: Session
) -> Rsp<Task> {
    with_auth(ses, data, |_, conn| {
        let Json(task_updates) = payload;
        let updated_task = update_task(id, task_updates, &conn)?;
        Ok(Json(updated_task))
    })
}

#[post("/task/complete/{id}")]
async fn complete_task_route(
    web::Path(id): web::Path<i32>,
    data: Data<PgPool>,
    ses: Session
) -> Rsp<i32> {
    println!("In complete task route");
    with_auth(ses, data, |_, conn| {
        let updated_pts = complete_task(id, &conn)?;
        Ok(Json(updated_pts))
    })
}

#[delete("/task/{id}")]
async fn delete_task_route(
    web::Path(id): web::Path<i32>,
    data: Data<PgPool>,
    ses: Session
) -> Rsp<()> {
    with_auth(ses, data, |_, conn| {
        delete_task(id, &conn)?;
        Ok(Json(()))
    })
}