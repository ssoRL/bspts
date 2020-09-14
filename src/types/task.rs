use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub checked: bool,
    pub daily_loss: i32
}