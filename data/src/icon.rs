use std::convert::From;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn color_to_str(color: &Color) -> &str {
    match color {
        Color::Red => "r",
        Color::Green => "g",
        Color::Blue => "b",
        Color::Yellow => "y",
    }
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
        let color = color_to_str(&self.color);
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
        format!("{}{}", color, cat)
    }
}

impl From<String> for TaskIcon {
    fn from(serialized: String) -> Self {
        let mut color_string = serialized.clone();
        // Remove the leading character (the color info)
        let category_string = color_string.split_off(1);
        // let css : &str = &*color_string;
        let color = match &*color_string {
            "r" => Color::Red,
            "g" => Color::Green,
            "b" => Color::Blue,
            // Defaults arbitrarily to yellow
            _ => Color::Yellow,
        };
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum RewardCategory {
    Bong,
    Book,
    Cookie,
    Coffee,
    GamePad,
    Dice,
    Booze,
    Television,
    Computer,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RewardIcon {
    pub color: Color,
    pub category: RewardCategory,
}

impl Default for RewardIcon {
    fn default() -> Self {
        Self {
            color: Color::Yellow,
            category: RewardCategory::Coffee,
        }
    }
}

impl Into<String> for RewardIcon {
    fn into(self: Self) -> String {
        let color = color_to_str(&self.color);
        let cat = match self.category {
            RewardCategory::Bong => "BONG",
            RewardCategory::Book => "BOOK",
            RewardCategory::Cookie => "COOKIE",
            RewardCategory::Coffee => "COFFEE",
            RewardCategory::GamePad => "GAME",
            RewardCategory::Dice => "DICE",
            RewardCategory::Booze => "BOOZE",
            RewardCategory::Television => "TV",
            RewardCategory::Computer => "COMPUTER",
        };
        format!("{}{}", color, cat)
    }
}

impl From<String> for RewardIcon {
    fn from(serialized: String) -> Self {
        let mut color_string = serialized.clone();
        // Remove the leading character (the color info)
        let category_string = color_string.split_off(1);
        // let css : &str = &*color_string;
        let color = match &*color_string {
            "r" => Color::Red,
            "g" => Color::Green,
            "b" => Color::Blue,
            // Defaults arbitrarily to yellow
            _ => Color::Yellow,
        };
        let category = match &*category_string {
            "BONG" => RewardCategory::Bong,
            "BOOK" => RewardCategory::Book,
            "COOKIE" => RewardCategory::Cookie,
            "GAME" => RewardCategory::GamePad,
            "BOOZE" => RewardCategory::Booze,
            "TV" => RewardCategory::Television,
            "COMPUTER" => RewardCategory::Computer,
            "DICE" => RewardCategory::Dice,
            _ => RewardCategory::Coffee,
        };
        Self {color, category,}
    }
}