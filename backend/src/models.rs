use chrono::{NaiveDate};
use diesel::pg::data_types::PgInterval;

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