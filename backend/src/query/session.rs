use crate::models::*;
use crate::PgPooledConnection;
use diesel::prelude::*;
use crate::error::*;

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

fn get_session(session_id: i32, conn: &PgPooledConnection) -> Result<QSession> {
    use crate::schema::sessions::dsl::*;

    let q_sessions = sessions
        .filter(id.eq(session_id))
        .load::<QSession>(conn)
        .expect("Error getting users from session id");

    match &q_sessions[..] {
        [] => Err(unauthorized(format!("There's no session with id {}", session_id))),
        [q_ses] =>  Ok(q_ses.clone()),
        _ => Err(conflict(format!("There was an error getting session with id {}", session_id))),
    }
}

pub fn get_session_user(session_id: i32, conn: &PgPooledConnection) -> Result<QUser> {
    use crate::schema::users::dsl::*;

    let q_ses = get_session(session_id, &conn)?;

    let q_users: Vec<QUser> = users
        .filter(id.eq(q_ses.user_id))
        .load(conn)
        .expect("Error getting users from session id");

    match &q_users[..] {
        [] => Err(not_found(format!("There's no user with id {}", q_ses.user_id))),
        [q_user] =>  Ok(q_user.clone()),
        _ => Err(conflict(format!("There was an error getting user with id {}", q_ses.user_id))),
    }
}