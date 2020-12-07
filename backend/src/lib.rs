#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
#[macro_use]
extern crate diesel_migrations;

pub mod query;
mod models;
mod schema;
pub mod routes;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> PgPool {
    dotenv().ok();

    let pg_pw = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let pg_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");
    let pg_db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let database_url = format!("postgres://postgres:{}@{}/{}", pg_pw, pg_port, pg_db);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool.");
    pool
}

embed_migrations!("./migrations");

pub fn run_db_migration(pool: &PgPool) {
    let conn = pool.get().expect("Could not get database connection from the pool");
    match  embedded_migrations::run(&conn) {
        Ok(_) => println!("Migration completed successfully"),
        Err(e) => println!("Could not complete migration because {}", e),
    }
}