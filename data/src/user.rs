use serde::{Deserialize, Serialize};

/// The fields that must be specified to create a new user or log in.
/// The password will be
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewUser {
    pub uname: String,
    pub password: String,
}

/// The fields that are exposed in the JWT claim for a user
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Claim {
    pub id: i32,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}