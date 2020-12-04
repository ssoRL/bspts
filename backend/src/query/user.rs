use ring::{digest, pbkdf2};
use rand_core::{RngCore, OsRng};
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
pub const JWT_SECRET: &[u8] = b"ewiruhnnisdfkjn";

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

    // &jsonwebtoken::EncodingKey::from_rsa_pem(key: &[u8]);

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

    let creds = generate_creds(&user.password);
    let insert = models::InsertableUser {
        uname: &user.uname,
        password: creds.0,
        salt: creds.1
    };
    
    diesel::insert_into(users::table)
        .values(insert)
        .get_result(&conn)
        .expect("Error saving new post")
}