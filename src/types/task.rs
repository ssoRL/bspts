use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub checked: bool,
    pub daily_loss: i32
}

impl Task {
    pub fn new(id: i32) -> Self {
        Task {
            id,
            name: String::from(""),
            description: String::from(""),
            checked: false,
            daily_loss: 0
        }
    }
}