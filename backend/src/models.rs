use chrono::{NaiveDate};
use diesel::pg::data_types::PgInterval;
use crate::schema::*;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub bspts: i32,
    pub is_done: bool,
    pub next_reset: NaiveDate,
    pub frequency: PgInterval,
}

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub bspts: i32,
    pub next_reset: NaiveDate,
    pub frequency: PgInterval,
}