use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use data::user::*;
use crate::models;
use crate::PgPooledConnection;
use diesel::RunQueryDsl;
use jsonwebtoken;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

// TODO: Maybe this isn't secruity?
const SALT: &str = "dkjfjkfdjfkd";
const JWT_SECRET: &str = "ewiruhnnisdfkjn";

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
pub fn user_to_token(user: User) -> String {
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &user,
        &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET.as_ref())
    );

    token.expect("Failed to turn user into token")
}

/// Saves a new user to the database and then returns that users name and id
pub fn save_new_user(user: NewUser, conn: PgPooledConnection) -> User {
    use crate::schema::users;

    let encrypted_password = encrypt_password(&user.uname, &user.password);
    let insert = models::InsertableUser {
        uname: &user.uname,
        password: encrypted_password,
    };
    
    let new_user: models::QUser = diesel::insert_into(users::table)
        .values(insert)
        .get_result(&conn)
        .expect("Error saving new post");

    User {
        id: new_user.id,
        uname : new_user.uname,
    }
}