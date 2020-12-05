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
pub struct User {
    pub uname: String,
}