#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

mod query;
mod models;
mod schema;

use actix::prelude::*;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use types::task::{Task, NewTask};
use crate::query::task::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
//use warp::{test};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn get_connection_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

// fn with_db(pool: PgPool) -> impl Filter<Extract = (PgPooledConnection,), Error = std::convert::Infallible> + Clone {
//     warp::any().map(move || {
//         let conn = pool.get().expect("Failed to get connection from pool");
//         conn
//     })
// }

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
            .service(fs::Files::new("/", "../site").index_file("../site/index.html"))
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