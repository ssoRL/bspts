pub mod task;
pub mod user;
pub mod reward;

use actix_web::{error, http::StatusCode, web::Data, HttpRequest};
use actix_session::{Session};
use crate::{
    PgPool, PgPooledConnection,
    models,
    query::session::get_session_user,
    error::*,
};
use chrono::NaiveDate;

const SESSION_ID_KEY: &str = "session_id";

pub fn with_auth<T, F>(ses: Session, data: Data<PgPool>, run: F)-> Rsp<T>
where
    F: FnOnce(models::QUser, PgPooledConnection) -> Rsp<T>
{
    let pool = data.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let session_cookie = ses.get::<i32>(SESSION_ID_KEY);
    match session_cookie {
        Ok(Some(session_id)) => {
            let user = get_session_user(session_id, &conn)?;
            run(user, conn)
        }
        _ => {
            let error = error::InternalError::new("Could not get session", StatusCode::UNAUTHORIZED);
            Err(error.into())
        }
    }
}

// Get the date sent in by the client
pub fn get_date(req: HttpRequest) -> NaiveDate {
    let try_make_date = || -> Option<NaiveDate> {
        let headers = req.headers();
        let year_str = headers.get("year")?.to_str().ok()?;
        let year = year_str.parse::<i32>().ok()?;
        let month_str = headers.get("month")?.to_str().ok()?;
        let month = month_str.parse::<u32>().ok()?;
        let day_str = headers.get("day")?.to_str().ok()?;
        let day = day_str.parse::<u32>().ok()?;
        NaiveDate::from_ymd_opt(year, month, day)
    };
    match try_make_date() {
        Some(date) => date,
        None => panic!("Could not get date from the headers"),
    }
}