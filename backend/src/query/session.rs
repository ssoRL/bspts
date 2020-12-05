use crate::models::*;
use crate::PgPooledConnection;
use diesel::prelude::*;
use actix_web::{error, Result, http::StatusCode};

/// Creates a new session and returns its id
pub fn start_session(q_user: &QUser, conn: &PgPooledConnection) -> QSession {
    use crate::schema::sessions;

    let insert_session = InsertableSession {
        user_id: q_user.id,
    };
    
    diesel::insert_into(sessions::table)
        .values(insert_session)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_session_user(q_ses: &QSession, conn: &PgPooledConnection) -> Result<QUser> {
    use crate::schema::users::dsl::*;

    let q_users: Vec<QUser> = users
        .filter(id.eq(q_ses.user_id))
        .load(conn)
        .expect("Error getting users from session id");

        match &q_users[..] {
            [] => {
                let error = error::InternalError::new(
                    format!("There's no user with id {}", q_ses.user_id),
                    StatusCode::NOT_FOUND
                );
                Err(error.into())
            }
            [q_user] =>  Ok(q_user.clone()),
            _ => {
                let error = error::InternalError::new(
                    format!("There was an error getting user with id {}", q_ses.user_id),
                    StatusCode::CONFLICT
                );
                Err(error.into())
            }
        }
}