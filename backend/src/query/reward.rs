use diesel::prelude::*;
use data::reward::*;
use chrono::{NaiveDate, Local, Duration, Datelike};
use crate::PgPooledConnection;
use crate::models::*;
use crate::error::*;
use crate::query::{atomically, user};


fn q_reward_to_reward(q: &QReward) -> Reward {
    Reward {
        id: q.id,
        name: q.name.clone(),
        description: q.description.clone(),
        user_id: q.user_id,
        bspts: q.bspts,
    }
}

/// Get all of the rewards for the user
/// * user: The user to get the rewards for
pub fn get_rewards(user: QUser, conn: &PgPooledConnection) -> Vec<Reward> {
    let q_rewards = QReward::belonging_to(&user)
        .load::<QReward>(conn)
        .expect("Error loading rewards");
        q_rewards.iter().map(q_reward_to_reward).collect()
}

fn get_q_reward(reward_id: i32, conn: &PgPooledConnection) -> Result<QReward> {
    use crate::schema::rewards::dsl::*;

    let mut q_rewards = rewards
        .filter(id.eq(reward_id))
        .load::<QReward>(conn)
        .map_err(|_| bad_request(format!("Error querying for reward {}", reward_id)))?;
    // Should be a vec of only one item, return that item
    match q_rewards.pop() {
        Some(q_reward) => Ok(q_reward),
        None => Err(not_found(format!("No reward with id {}", reward_id))),
    }
}

pub fn get_reward(reward_id: i32, conn: &PgPooledConnection) -> Result<Reward> {
    let q_reward = get_q_reward(reward_id, conn)?;
    Ok(q_reward_to_reward(&q_reward))
}

/// Add a new reward to the database
pub fn commit_new_reward(new_reward: NewReward, user: QUser, conn: PgPooledConnection) -> Reward {
    use crate::schema::rewards;

    let insert_reward = InsertableReward {
        user_id: user.id,
        name: &new_reward.name,
        description: &new_reward.description,
        bspts: new_reward.bspts,
    };
    
    let committed_reward: QReward = diesel::insert_into(rewards::table)
        .values(insert_reward)
        .get_result(&conn)
        .expect("Error saving new reward");

    q_reward_to_reward(&committed_reward)
}

/// Updates the reward with reward_id to the value q_reward and returns the updated reward
fn update_q_reward(q_reward: &QReward, conn: &PgPooledConnection) -> Result<QReward> {
    use crate::schema::rewards::dsl::rewards;

    diesel::update(rewards.find(q_reward.id))
        .set(q_reward)
        .get_result(conn)
        .map_err(|_| bad_request(format!("Error updating for reward {}", q_reward.id)))
}

/// Add a new reward to the database
pub fn update_reward(reward_id: i32, new_reward: NewReward, conn: &PgPooledConnection) -> Result<Reward> {
    let mut q_reward = get_q_reward(reward_id, &conn)?;

    q_reward.name = new_reward.name;
    q_reward.description = new_reward.description;
    q_reward.bspts = new_reward.bspts;

    let committed_reward = update_q_reward(&q_reward, conn)?;

    Ok(q_reward_to_reward(&committed_reward))
}

pub fn delete_reward(reward_id: i32, conn: &PgPooledConnection) -> Result<()> {
    use crate::schema::rewards::dsl::rewards;

    match diesel::delete(rewards.find(reward_id)).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(bad_request(format!("Could not delete reward {}", reward_id))),
    }
}