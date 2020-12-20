use ring::{digest, pbkdf2};
use rand_core::{RngCore, OsRng};
use std::num::NonZeroU32;
use data::user::*;
use crate::models;
use crate::PgPooledConnection;
use diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use actix_web::{error, Result, http::StatusCode};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

/// Takes a user name and password and returns the hashed pw and a random salt
fn generate_creds(password: &str) -> (Vec<u8>, Vec<u8>) {
    // Generate a salt for this user
    let mut salt = [0u8, 16];
    OsRng.fill_bytes(&mut salt);
    let mut encrypted_password: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut encrypted_password
    );
    (encrypted_password.to_vec(), salt.to_vec())
}

/// checks that a given password is valid for a given user
fn check_password(password: &str, user: &models::QUser) -> bool {
    let stored_hash: &Vec<u8> = &user.password;
    let mut generated_hash: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        NonZeroU32::new(100_000).unwrap(),
        &user.salt,
        password.as_bytes(),
        &mut generated_hash
    );
    stored_hash == &generated_hash.to_vec()
}

/// Returns the user with the given username
fn get_q_user_by_id(user_id: i32, conn: &PgPooledConnection) -> Result<models::QUser> {
    use crate::schema::users::dsl::*;
    use diesel::query_dsl::filter_dsl::FilterDsl;

    let q_users: Vec<models::QUser> = users
        .filter(id.eq(user_id))
        .load::<models::QUser>(conn)
        .expect("Error getting users");

    match &q_users[..] {
        [] => {
            let error = error::InternalError::new(
                format!("Found no user with id {}", user_id),
                StatusCode::NOT_FOUND
            );
            Err(error.into())
        }
        [q_user] =>  {
            Ok(q_user.clone())
        },
        _ => {
            let error = error::InternalError::new(
                "Ambiguous user id".to_string(),
                StatusCode::CONFLICT
            );
            Err(error.into())
        }
    }
}

fn update_q_user(q_user: models::QUser, conn: &PgPooledConnection) -> Result<models::QUser> {
    use crate::schema::users::dsl::*;
    use crate::diesel::query_dsl::filter_dsl::FindDsl;

    diesel::update(users.find(q_user.id))
        .set(&q_user)
        .get_result(conn)
        .map_err(|_| {
            error::InternalError::new(
                format!("Error updating for user {}", q_user.id),
                StatusCode::BAD_REQUEST
            ).into()
        })
}

/// Returns the user if they are allowed to log in with that password, or an error otherwise
pub fn login_user(user: NewUser, conn: &PgPooledConnection) -> Result<models::QUser> {
    use crate::schema::users::dsl::*;
    use diesel::query_dsl::filter_dsl::FilterDsl;

    let q_users: Vec<models::QUser> = users
        .filter(uname.eq(user.uname))
        .load::<models::QUser>(conn)
        .expect("Error getting users");

    match &q_users[..] {
        [] => {
            let error = error::InternalError::new(
                "There's no user with that username".to_string(),
                StatusCode::NOT_FOUND
            );
            Err(error.into())
        }
        [q_user] =>  {
            if check_password(&user.password, q_user) {
                Ok(q_user.clone())
            } else {
                let error = error::InternalError::new(
                    "Incorrect password".to_string(),
                    StatusCode::UNAUTHORIZED
                );
                Err(error.into())
            }
        },
        _ => {
            let error = error::InternalError::new(
                "Ambiguous user name".to_string(),
                StatusCode::CONFLICT
            );
            Err(error.into())
        }
    }
}

/// Saves a new user to the database and then returns that users name and id
pub fn save_new_user(user: &NewUser, conn: &PgPooledConnection) -> Result<models::QUser> {
    use crate::schema::users;

    let creds = generate_creds(&user.password);
    let insert = models::InsertableUser {
        uname: &user.uname,
        password: creds.0,
        salt: creds.1
    };
    
    let save_user_result = diesel::insert_into(users::table)
        .values(insert)
        .get_result::<models::QUser>(conn);

    save_user_result.map_err(|_| {
        let error = error::InternalError::new(
            "There's already a user with that username".to_string(),
            StatusCode::CONFLICT
        );
        error.into()
    })
}

/// Adds the provided number of points to the user's total
/// Returns their total points after the addition
pub fn update_bspts(user_id: i32, pts: i32, conn: &PgPooledConnection) -> Result<i32> {
    let mut q_user = get_q_user_by_id(user_id, conn)?;
    q_user.bspts += pts;
    let updated_q_user = update_q_user(q_user, conn)?;
    Ok(updated_q_user.bspts)
}