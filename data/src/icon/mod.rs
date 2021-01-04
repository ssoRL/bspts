mod reward;

use std::convert::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumString, EnumIter};

pub use reward::{RewardIcon, RewardCategory};

#[derive(Display, EnumString, EnumIter, Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    #[strum(serialize = "r")]
    Red,
    #[strum(serialize = "g")]
    Green,
    #[strum(serialize = "b")]
    Blue,
    #[strum(serialize = "y")]
    Yellow,
}

// pub fn default

pub trait BadgeIcon<CAT>
    where CAT: Clone
{
    fn new_ptr(cat: CAT, color: Color) -> Box<Self>;
    fn get_color(&self) -> Color;
    fn set_color(&mut self, color: Color);
    fn get_category(&self) -> CAT;
    fn set_category(&mut self, color: CAT);
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum TaskCategory {
    Animal,
    Exercise,
    Read,
    Clean,
    Code,
    Communicate,
    Music,
    Writing,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TaskIcon {
    pub color: Color,
    pub category: TaskCategory,
}

impl Default for TaskIcon {
    fn default() -> Self {
        Self {
            color: Color::Yellow,
            category: TaskCategory::Writing,
        }
    }
}

impl Into<String> for TaskIcon {
    fn into(self: Self) -> String {
        let color = &self.color.to_string();
        let cat = match self.category {
            TaskCategory::Animal => "PET",
            TaskCategory::Exercise => "EXERCISE",
            TaskCategory::Read => "READ",
            TaskCategory::Clean => "CLEAN",
            TaskCategory::Code => "CODE",
            TaskCategory::Communicate => "COMM",
            TaskCategory::Music => "MUSIC",
            TaskCategory::Writing => "WRITE",
        };
        // let cat = match serde_json::to_string(&self.category) {
        //     Ok(s) => s,
        //     _ => "".to_string(),
        // };
        format!("{}{}", color, cat)
    }
}

impl From<String> for TaskIcon {
    fn from(serialized: String) -> Self {
        let mut color_string = serialized.clone();
        // Remove the leading character (the color info)
        let category_string = color_string.split_off(1);
        // let css : &str = &*color_string;
        let color = Color::from_str(&color_string).unwrap();
        let category = match &*category_string {
            "PET" => TaskCategory::Animal,
            "EXERCISE" => TaskCategory::Exercise,
            "READ" => TaskCategory::Read,
            "CLEAN" => TaskCategory::Clean,
            "CODE" => TaskCategory::Code,
            "COMM" => TaskCategory::Communicate,
            "MUSIC" => TaskCategory::Music,
            _ => TaskCategory::Writing,
        };
        Self {color, category,}
    }
}