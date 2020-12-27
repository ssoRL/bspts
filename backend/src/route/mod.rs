pub mod task;
pub mod user;

use actix_web::{error, http::StatusCode, web::Data};
use actix_session::{Session};
use crate::{
    PgPool, PgPooledConnection,
    models,
    query::session::get_session_user,
    error::*,
};

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