use actix_web::{
    get,
    post,
    web::{Data, Json, ServiceConfig}
};
use data::user::*;
use actix_session::{Session};
use crate::{
    PgPool,
    route::*,
    error::*,
    query::{user::*, session::*}
};

#[post("/login")]
async fn sign_in(payload: Json<NewUser>, database: Data<PgPool>, ses: Session) -> Rsp<User>  {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let Json(new_user) = payload;
    let user = login_user(new_user, &conn)?;
    let new_session = start_session(&user, &conn);
    ses.set(SESSION_ID_KEY, new_session.id)?;
    Ok(Json(User {uname: user.uname, bspts: user.bspts}))
}

#[get("/user")]
async fn get_user(data: Data<PgPool>, ses: Session) -> Rsp<User> {
    with_auth(ses, data, |user, _| {
        Ok(Json(User {uname: user.uname, bspts: user.bspts}))
    })
}

#[post("/user")]
async fn sign_up(payload: Json<NewUser>, database: Data<PgPool>, ses: Session) -> Rsp<User> {
    let pool = database.get_ref().clone();
    let conn = pool.get().expect("Failed to get database connection");
    let Json(new_user) = payload;
    let user = save_new_user(&new_user, &conn)?;
    let new_session = start_session(&user, &conn);
    ses.set(SESSION_ID_KEY, new_session.id)?;
    Ok(Json(User {uname: user.uname, bspts: user.bspts}))
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(sign_in);
    config.service(sign_up);
}