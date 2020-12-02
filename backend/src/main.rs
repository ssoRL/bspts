#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
#[macro_use]
extern crate diesel_migrations;

mod query;
mod models;
mod schema;

use actix_web::{get, error, http::StatusCode, post, web::{Data, HttpRequest, Json}, App, HttpServer};
use actix_files as fs;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use data::task::*;
use data::user::*;
use crate::query::task::*;
use crate::query::user::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use jsonwebtoken;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

type Rsp<T> = actix_web::Result<Json<T>>;

embed_migrations!("./migrations");

fn get_connection_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
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
fn with_auth<T, F: FnOnce(Claim) -> Rsp<T>>(req: HttpRequest, run: F) -> Rsp<T> {
    let headers = req.headers();
    let auth_header = headers.get("auth");
    match auth_header {
        Some(jwt_header) => {
            let jwt = jwt_header.to_str().expect("Could not deserialize jwt from auth header");
            let decoded_token_result = jsonwebtoken::decode::<Claim>(
                jwt,
                &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET),
                &jsonwebtoken::Validation::default()
            );
            match decoded_token_result {
                Ok(decoded_token) =>  {
                    let user = decoded_token.claims;
                    run(user)
                }
                Err(e) => {
                    let msg = format!("Invalid token to access {} because {}", req.uri(), e);
                    let error = error::InternalError::new(msg, StatusCode::UNAUTHORIZED);
                    Err(error.into())
                }
            }
        }
        None => {
            let msg = format!("Token required to access {}", req.uri());
            let error = error::InternalError::new(msg, StatusCode::UNAUTHORIZED);
            Err(error.into())
        }
    }
}

#[post("/login")]
async fn sign_in_route(payload: Json<NewUser>, database: Data<PgPool>) -> Rsp<String>  {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let Json(new_user) = payload;
    let user_result = login_user(new_user, conn);
    match user_result {
        Ok(user) => {
            let token = user_to_token(user);
            Ok(Json(token))
        },
        Err(e) => Err(e)
    }
}

#[post("/user")]
async fn sign_up_route(payload: Json<NewUser>, database: Data<PgPool>) -> Json<String> {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let Json(new_user) = payload;
    let user = save_new_user(new_user, conn);
    let token = user_to_token(user);
    Json(token)
}

#[get("/task")]
async fn task_route(req: HttpRequest, database: Data<PgPool>) -> Rsp<Vec<Task>> {
    with_auth(req, |user: Claim| {
        let pool = database.get_ref().clone();
        let conn = pool.get().expect("Failed to get database connection");
        let tasks = get_tasks(user, conn);
        Ok(Json(tasks))
    })
}

#[post("/task")]
async fn commit_new_task_route(req: HttpRequest, payload: Json<NewTask>, database: Data<PgPool>) -> Rsp<Task> {
    with_auth(req, |user: Claim| {
        let pool = database.get_ref().clone();
        let conn = pool.get().expect("Failed to get database connection");
        let Json(new_task) = payload;
        let committed_task = commit_new_task(new_task, user, conn);
        Ok(Json(committed_task))
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(task_route)
            .service(commit_new_task_route)
            .service(sign_in_route)
            .service(sign_up_route)
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