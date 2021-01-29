use serde::{Deserialize, Serialize};
use crate::icon::RewardIcon;

/// The fields that must be specified when creating a new reward
/// The rest will be set to default values
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewReward {
    pub name: String,
    pub description: String,
    pub bspts: i32,
    pub icon: RewardIcon,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Reward {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub bspts: i32,
    pub icon: RewardIcon,
}

impl Into<NewReward> for Reward {
    fn into(self: Self) -> NewReward {
        NewReward {
            name: self.name.clone(),
            description: self.description.clone(),
            bspts: self.bspts,
            icon: self.icon.clone(),
        }
    }
}