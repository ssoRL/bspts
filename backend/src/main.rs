#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

mod query;
mod models;
mod schema;

use warp::Filter;
use types::task::{NewTask};
use crate::query::task::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

fn with_db(pool: Pool<ConnectionManager<PgConnection>>) -> impl Filter<Extract = (PgPooledConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || {
        let conn = pool.get().expect("Failed to get connection from pool");
        conn
    })
}

#[tokio::main]
async fn main() {

    let pool = get_connection_pool();

    let task_route = warp::path!("task")
        .and(warp::get())
        .and(with_db(pool.clone()))
        .map(|conn| {
            let tasks = get_tasks(conn);
            let serialized_tasks = serde_json::to_string(&tasks).unwrap();
            serialized_tasks
        });
        
    let commit_new_task_route = warp::path!("task")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .map(|new_task: NewTask, conn| {
            let committed_task = commit_new_task(new_task, conn);
            let serialized_task = serde_json::to_string(&committed_task).unwrap();
            serialized_task
        });
    
    let file_server = warp::fs::dir("../site");

    warp::serve(task_route.or(commit_new_task_route).or(file_server))
        .run(([127, 0, 0, 1], 3030))
        .await;
}