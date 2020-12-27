use actix_web::{error, web::Json, http::StatusCode};

pub type Error = error::Error;
pub type Result<T> = actix_web::Result<T>;
pub type Rsp<T> = Result<Json<T>>;

pub fn not_found(msg: String) -> Error {
    let error = error::InternalError::new(
        msg,
        StatusCode::NOT_FOUND
    );
    error.into()
}

pub fn unauthorized(msg: String) -> Error {
    let error = error::InternalError::new(
        msg,
        StatusCode::UNAUTHORIZED
    );
    error.into()
}

pub fn conflict(msg: String) -> Error {
    let error = error::InternalError::new(
        msg,
        StatusCode::CONFLICT
    );
    error.into()
}

pub fn bad_request(msg: String) -> Error {
    let error = error::InternalError::new(
        msg,
        StatusCode::BAD_REQUEST
    );
    error.into()
}