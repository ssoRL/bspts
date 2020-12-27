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

#[get("/reward")]
async fn get_all(data: Data<PgPool>, ses: Session) -> Rsp<Vec<Task>> {
    with_auth(ses, data, |user, conn| {
        let tasks = get_tasks(user, false, &conn);
        Ok(Json(tasks))
    })
}

#[get("/reward/{id}")]
async fn get_by_id(web::Path(id): web::Path<i32>, data: Data<PgPool>, ses: Session) -> Rsp<Task> {
    with_auth(ses, data, |_, conn| {
        let task = get_task(id, &conn)?;
        Ok(Json(task))
    })
}

#[post("/reward")]
async fn new(payload: Json<NewTask>, data: Data<PgPool>, ses: Session) -> Rsp<Task> {
    with_auth(ses, data, |user, conn| {
        let Json(new_task) = payload;
        let committed_task = commit_new_task(new_task, user, conn);
        Ok(Json(committed_task))
    })
}

#[put("/reward/{id}")]
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

#[delete("/reward/{id}")]
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
    config.service(get_all);
    config.service(get_by_id);
    config.service(new);
    config.service(update);
    config.service(delete);
}