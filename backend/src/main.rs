#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

mod query;
mod models;
mod schema;

use actix_web::{get, post, web, App, HttpServer};
use actix_files as fs;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use data::task::*;
use data::user::*;
use crate::query::task::*;
use crate::query::user::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

type Rsp<T> = actix_web::Result<web::Json<T>>;

fn get_connection_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

#[post("/login")]
async fn signin_route(payload: web::Json<NewUser>, database: web::Data<PgPool>) -> Rsp<String>  {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let web::Json(new_user) = payload;
    let user_result = login_user(new_user, conn);
    match user_result {
        Ok(user) => {
            let token = user_to_token(user);
            Ok(web::Json(token))
        },
        Err(e) => Err(e)
    }
}

#[post("/user")]
async fn signup_route(payload: web::Json<NewUser>, database: web::Data<PgPool>) -> web::Json<String> {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let web::Json(new_user) = payload;
    let user = save_new_user(new_user, conn);
    let token = user_to_token(user);
    web::Json(token)
}

#[get("/task")]
async fn task_route(database: web::Data<PgPool>) -> web::Json<Vec<Task>> {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let tasks = get_tasks(conn);
    web::Json(tasks)
}

#[post("/task")]
async fn commit_new_task_route(payload: web::Json<NewTask>, database: web::Data<PgPool>) -> web::Json<Task> {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let web::Json(new_task) = payload;
    let committed_task = commit_new_task(new_task, conn);
    web::Json(committed_task)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(task_route)
            .service(commit_new_task_route)
            .service(signin_route)
            .service(signup_route)
            .service(fs::Files::new("/", "./site").index_file("index.html"))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    
    #[actix_rt::test]
    async fn test_get_task() {
        let pool = get_connection_pool();
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(task_route)
        ).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/task").to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("{:#?}", resp);
        assert!(resp.status().is_success());

    }
}