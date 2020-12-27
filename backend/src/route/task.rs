use actix_web::{
    get,
    delete,
    post,
    put,
    web::{self, Data, Json, ServiceConfig}
};
use data::task::*;
use crate::query::task::*;
use actix_session::{Session};
use crate::PgPool;
use crate::route::*;
use crate::error::*;

#[get("/task/todo")]
async fn get_todo(data: Data<PgPool>, ses: Session) -> Rsp<Vec<Task>> {
    with_auth(ses, data, |user, conn| {
        let tasks = get_tasks(user, false, &conn);
        Ok(Json(tasks))
    })
}

#[get("/task/done")]
async fn get_done(data: Data<PgPool>, ses: Session) -> Rsp<Vec<Task>> {
    with_auth(ses, data, |user, conn| {
        let tasks = get_tasks(user, true, &conn);
        Ok(Json(tasks))
    })
}

#[get("/task/{id}")]
async fn get_by_id(web::Path(id): web::Path<i32>, data: Data<PgPool>, ses: Session) -> Rsp<Task> {
    with_auth(ses, data, |_, conn| {
        let task = get_task(id, &conn)?;
        Ok(Json(task))
    })
}

#[post("/task")]
async fn commit_new(payload: Json<NewTask>, data: Data<PgPool>, ses: Session) -> Rsp<Task> {
    with_auth(ses, data, |user, conn| {
        let Json(new_task) = payload;
        let committed_task = commit_new_task(new_task, user, conn);
        Ok(Json(committed_task))
    })
}

#[put("/task/{id}")]
async fn update(
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
async fn complete(
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
    config.service(get_by_id);
    config.service(commit_new);
    config.service(update);
    config.service(complete);
    config.service(delete);
}