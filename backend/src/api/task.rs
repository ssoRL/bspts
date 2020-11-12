use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::models::*;
use crate::schema::tasks::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    println!("durl: {}", database_url);
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/** Get all of the tasks for the user */
pub fn get_tasks() -> Vec<Task> {
    let connection = establish_connection();
    tasks.limit(5).load::<Task>(&connection).expect("Error loading posts")
}