extern crate diesel;
extern crate dotenv;
extern crate chrono;

use actix_web::{App, HttpServer};
use actix_files as fs;
use dotenv::dotenv;
use std::env;
use actix_session::{CookieSession};
use backend_lib::{self, route, get_connection_pool, run_db_migration};

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
                CookieSession::signed(&cookie_key.as_bytes())
                      .secure(false)
            )
            .configure(route::task::configure)
            .configure(route::reward::configure)
            .configure(route::user::configure)
            .service(fs::Files::new("/", "./site").index_file("index.html"))
    })
    .bind(api_url)?
    .run()
    .await
}