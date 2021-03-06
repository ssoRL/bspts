use actix_web::{
    get,
    delete,
    post,
    put,
    web::{self, Data, Json, ServiceConfig}
};
use data::reward::*;
use crate::query::{self, reward::*};
use actix_session::{Session};
use crate::PgPool;
use crate::route::*;
use crate::error::*;

#[get("/reward")]
async fn get_all(data: Data<PgPool>, ses: Session) -> Rsp<Vec<Reward>> {
    with_auth(ses, data, |user, conn| {
        let rewards = get_rewards(user, &conn);
        Ok(Json(rewards))
    })
}

#[get("/reward/{id}")]
async fn get_by_id(web::Path(id): web::Path<i32>, data: Data<PgPool>, ses: Session) -> Rsp<Reward> {
    with_auth(ses, data, |_, conn| {
        let reward = get_reward(id, &conn)?;
        Ok(Json(reward))
    })
}

#[post("/reward")]
async fn new(payload: Json<NewReward>, data: Data<PgPool>, ses: Session) -> Rsp<Reward> {
    with_auth(ses, data, |user, conn| {
        let Json(new_reward) = payload;
        let committed_reward = commit_new_reward(new_reward, user, conn);
        Ok(Json(committed_reward))
    })
}

/// Takes the id of a reward and removes points from the user's
/// total equal to the reward's cost.
#[post("/reward/do/{id}")]
async fn did_it(
    web::Path(id): web::Path<i32>,
    data: Data<PgPool>,
    ses: Session
) -> Rsp<i32> {
    with_auth(ses, data, |user, conn| {
        let reward = get_reward(id, &conn)?;
        let cost = -reward.bspts;
        let new_pts = query::user::update_bspts(user.id, cost, &conn)?;
        Ok(Json(new_pts))
    })
}

#[put("/reward/{id}")]
async fn update(
    web::Path(id): web::Path<i32>,
    payload: Json<NewReward>,
    data: Data<PgPool>,
    ses: Session
) -> Rsp<Reward> {
    with_auth(ses, data, |_, conn| {
        let Json(reward_updates) = payload;
        let updated_reward = update_reward(id, reward_updates, &conn)?;
        Ok(Json(updated_reward))
    })
}

#[delete("/reward/{id}")]
async fn delete(
    web::Path(id): web::Path<i32>,
    data: Data<PgPool>,
    ses: Session
) -> Rsp<()> {
    with_auth(ses, data, |_, conn| {
        delete_reward(id, &conn)?;
        Ok(Json(()))
    })
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(get_all);
    config.service(get_by_id);
    config.service(new);
    config.service(did_it);
    config.service(update);
    config.service(delete);
}