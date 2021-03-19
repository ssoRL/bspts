use actix_web::{
    get,
    delete,
    post,
    put,
    web::{self, Data, Json, ServiceConfig},
    HttpRequest,
};
use data::task::*;
use crate::query::task::*;
use actix_session::{Session};
use crate::PgPool;
use crate::route::*;
use crate::error::*;

#[get("/task/todo")]
async fn get_todo(req: HttpRequest, data: Data<PgPool>, ses: Session) -> Rsp<Vec<Task>> {
    with_auth(ses, data, |user, conn| {
        let today = get_date(req);
        let tasks = get_todo_tasks(user, &conn, today);
        Ok(Json(tasks))
    })
}

#[get("/task/done")]
async fn get_done(req: HttpRequest, data: Data<PgPool>, ses: Session) -> Rsp<Vec<Task>> {
    with_auth(ses, data, |user, conn| {
        let today = get_date(req);
        let tasks_lists = get_done_tasks(user, &conn, today);
        Ok(Json(tasks_lists))
    })
}

#[post("/task/undo")]
async fn undo(req: HttpRequest, data: Data<PgPool>, ses: Session) -> Rsp<Vec<Task>> {
    with_auth(ses, data, |user, conn| {
        let today = get_date(req);
        let tasks_lists = move_tasks_to_todo_if_ready(user, &conn, today);
        Ok(Json(tasks_lists))
    })
}

#[get("/task/{id}")]
async fn get_by_id(web::Path(id): web::Path<i32>, req: HttpRequest, data: Data<PgPool>, ses: Session) -> Rsp<Task> {
    with_auth(ses, data, |_, conn| {
        let today = get_date(req);
        let task = get_task(id, &conn, today)?;
        Ok(Json(task))
    })
}

#[post("/task")]
async fn commit_new(req: HttpRequest, payload: Json<NewTask>, data: Data<PgPool>, ses: Session) -> Rsp<Task> {
    with_auth(ses, data, |user, conn| {
        let Json(new_task) = payload;
        let today = get_date(req);
        let committed_task = commit_new_task(new_task, user, conn, today);
        Ok(Json(committed_task))
    })
}

#[put("/task/{id}")]
async fn update(
    web::Path(id): web::Path<i32>,
    req: HttpRequest,
    payload: Json<NewTask>,
    data: Data<PgPool>,
    ses: Session
) -> Rsp<Task> {
    with_auth(ses, data, |_, conn| {
        let Json(task_updates) = payload;
        let today = get_date(req);
        let updated_task = update_task(id, task_updates, &conn, today)?;
        Ok(Json(updated_task))
    })
}

#[post("/task/complete/{id}")]
async fn complete(
    web::Path(id): web::Path<i32>,
    req: HttpRequest, 
    data: Data<PgPool>,
    ses: Session
) -> Rsp<Task> {
    println!("In complete task route");
    with_auth(ses, data, |_, conn| {
        let today = get_date(req);
        let updated_pts = complete_task(id, &conn, today)?;
        Ok(Json(updated_pts))
    })
}

#[delete("/task/{id}")]
async fn delete(
    web::Path(id): web::Path<i32>,
    data: Data<PgPool>,
    ses: Session
) -> Rsp<()> {
    with_auth(ses, data, |_, conn| {
        delete_task(id, &conn)?;
        Ok(Json(()))
    })
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(get_todo);
    config.service(get_done);
    config.service(undo);
    config.service(get_by_id);
    config.service(commit_new);
    config.service(update);
    config.service(complete);
    config.service(delete);
}