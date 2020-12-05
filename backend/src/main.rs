#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
#[macro_use]
extern crate diesel_migrations;

mod query;
mod models;
mod schema;

use actix_web::{get, error, http::StatusCode, post, web::{Data, Json}, App, HttpServer};
use actix_files as fs;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use data::task::*;
use data::user::*;
use crate::query::task::*;
use crate::query::user::*;
use crate::query::session::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::fs::read;
use actix_session::{Session, CookieSession};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

type Rsp<T> = actix_web::Result<Json<T>>;

const SESSION_KEY: &str = "session";

embed_migrations!("./migrations");

fn get_connection_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool.");
    let conn = pool.get().expect("Could not get connection");
    match  embedded_migrations::run(&conn) {
        Ok(_) => println!("Migration completed successfully"),
        Err(e) => println!("Could not complete migration because {}", e),
    }
    pool
}

/// Runs the provided function with a jwt from the headers, or else throws an error
fn with_auth<T, F>(ses: Session, data: Data<PgPool>, run: F)-> Rsp<T>
where
    F: FnOnce(models::QUser, PgPooledConnection) -> Rsp<T>
{
    let pool = data.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let session_cookie = ses.get::<models::QSession>(SESSION_KEY);
    match session_cookie {
        Ok(Some(session)) => {
            let user = get_session_user(&session, &conn)?;
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
    ses.set(SESSION_KEY, new_session)?;
    Ok(Json(User {uname: user.uname}))
}

#[post("/user")]
async fn sign_up_route(payload: Json<NewUser>, database: Data<PgPool>, ses: Session) -> Rsp<User> {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let Json(new_user) = payload;
    let user = save_new_user(new_user, &conn);
    let new_session = start_session(&user, &conn);
    ses.set(SESSION_KEY, new_session)?;
    Ok(Json(User {uname: user.uname}))
}

#[get("/task")]
async fn task_route(data: Data<PgPool>, ses: Session) -> Rsp<Vec<Task>> {
    with_auth(ses, data, |user, conn| {
        let tasks = get_tasks(user, conn);
        Ok(Json(tasks))
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = get_connection_pool();

    let secret = read("secrets/jwt.key").expect("Could not read jwt secret file.");

    let api_url = env::var("API_URL").expect("API_URL must be set");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                CookieSession::signed(&secret) // <- create cookie based session middleware
                      .secure(false)
            )
            .service(task_route)
            .service(commit_new_task_route)
            .service(sign_in_route)
            .service(sign_up_route)
            .service(fs::Files::new("/", "./site").index_file("index.html"))
    })
    .bind(api_url)?
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