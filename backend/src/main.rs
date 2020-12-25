extern crate diesel;
extern crate dotenv;
extern crate chrono;

use actix_web::{App, HttpServer};
use actix_files as fs;
use dotenv::dotenv;
use std::env;
use actix_session::{CookieSession};
use backend_lib::{get_connection_pool, run_db_migration, routes::*};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = get_connection_pool();
    run_db_migration(&pool);

    let api_url = env::var("API_URL").expect("API_URL must be set");
    let cookie_key =  env::var("COOKIE_KEY").expect("COOKIE_KEY must be set");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                CookieSession::signed(&cookie_key.as_bytes()) // <- create cookie based session middleware
                      .secure(false)
            )
            .service(get_todo_tasks_route)
            .service(get_done_tasks_route)
            .service(get_task_route)
            .service(commit_new_task_route)
            .service(update_task_route)
            .service(delete_task_route)
            .service(complete_task_route)
            .service(sign_in_route)
            .service(sign_up_route)
            .service(fs::Files::new("/", "./site").index_file("index.html"))
    })
    .bind(api_url)?
    .run()
    .await
}