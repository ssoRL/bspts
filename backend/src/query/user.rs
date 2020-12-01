use ring::{digest, pbkdf2};
use std::num::NonZeroU32;
use data::user::*;
use crate::models;
use crate::PgPooledConnection;
use diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use jsonwebtoken;
use actix_web::{error, Result, http::StatusCode};
use std::time::{SystemTime, UNIX_EPOCH};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

// TODO: Maybe this isn't security?
const SALT: &str = "dkjfjkfdjfkd";
pub const JWT_SECRET: &[u8] = b"ewiruhnnisdfkjn";

// The salt should have a user-specific component so that an attacker
// cannot crack one password for multiple users in the database. It
// should have a database-unique component so that an attacker cannot
// crack the same user's password across databases in the unfortunate
// but common case that the user has used the same password for
// multiple systems.
fn salt(username: &str) -> Vec<u8> {
    let salt_bytes = SALT.as_bytes();
    let uname_bytes = username.as_bytes();
    let mut salt = Vec::with_capacity(salt_bytes.len() + uname_bytes.len());
    salt.extend(salt_bytes);
    salt.extend(uname_bytes);
    salt
}

fn encrypt_password(username: &str, password: &str) -> Vec<u8> {
    let salt = salt(username);
    let mut encrypted_password: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut encrypted_password
    );
    encrypted_password.to_vec()
}

/// Converts a user to a jwt string that can be used in the future
pub fn user_to_token(user: models::QUser) -> String {
    const DEFAULT_TOKEN_VALIDITY: i64 = 3600;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Could not get current time");
    let claim = Claim {
        id: user.id,
        sub: user.uname,
        iat:  now.as_secs() as i64,
        exp:  now.as_secs() as i64 + DEFAULT_TOKEN_VALIDITY,
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET)
    );

    token.expect("Failed to turn user into token")
}

pub fn login_user(user: NewUser, conn: PgPooledConnection) -> Result<models::QUser> {
    use crate::schema::users::dsl::*;
    use diesel::query_dsl::filter_dsl::FilterDsl;

    let q_users: Vec<models::QUser> = users
        .filter(uname.eq(user.uname))
        .load::<models::QUser>(&conn)
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
            let encrypted_password = encrypt_password(&q_user.uname, &user.password);
            if encrypted_password == q_user.password {
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
                "There's no user with that username".to_string(),
                StatusCode::CONFLICT
            );
            Err(error.into())
        }
    }
}

/// Saves a new user to the database and then returns that users name and id
pub fn save_new_user(user: NewUser, conn: PgPooledConnection) -> models::QUser {
    use crate::schema::users;

    let encrypted_password = encrypt_password(&user.uname, &user.password);
    let insert = models::InsertableUser {
        uname: &user.uname,
        password: encrypted_password,
    };
    
    diesel::insert_into(users::table)
        .values(insert)
        .get_result(&conn)
        .expect("Error saving new post")
}