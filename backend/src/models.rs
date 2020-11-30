use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct QFullTask {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub bspts: i32,
    pub is_done: bool,
    pub next_reset: NaiveDate,
    pub every: i32,
    pub time_unit: String,
    pub by_when: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="tasks"]
pub struct InsertableTask<'a> {
    pub user_id: i32,
    pub name: &'a str,
    pub description: &'a str,
    pub bspts: i32,
    pub next_reset: NaiveDate,
    pub every: i32,
    pub time_unit: &'a str,
    pub by_when: i32,
}

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct QUser {
    pub id: i32,
    pub uname: String,
    pub password: Vec<u8>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct InsertableUser<'a> {
    pub uname: &'a str,
    pub password: Vec<u8>,
}