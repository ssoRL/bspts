use crate::icon::*;
use std::str::FromStr;

#[derive(Display, EnumString, EnumIter, Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RewardCategory {
    Cookie,
    Coffee,
    Television,
    Dice,
    GamePad,
    Booze,
    Computer,
    NailPolish,
    Book,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RewardIcon {
    color: Color,
    category: RewardCategory,
}

impl Default for RewardIcon {
    fn default() -> Self {
        Self {
            color: Color::Yellow,
            category: RewardCategory::Coffee,
        }
    }
}

impl BadgeIcon<RewardCategory> for RewardIcon {
    fn new_ptr(category: RewardCategory, color: Color) -> Box<Self> {
        Box::new(RewardIcon {category, color})
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn get_category(&self) -> RewardCategory {
        self.category
    }

    fn set_category(&mut self, category: RewardCategory) {
        self.category = category;
    }
}

impl Into<String> for RewardIcon {
    fn into(self: Self) -> String {
        let color = &self.color.to_string();
        let cat = &self.category.to_string();
        format!("{}{}", color, cat)
    }
}

impl From<String> for RewardIcon {
    fn from(serialized: String) -> Self {
        let mut color_string = serialized.clone();
        // Remove the leading character (the color info)
        let category_string = color_string.split_off(1);
        // let css : &str = &*color_string;
        let color = match Color::from_str(&color_string) {
            Ok(c) => c,
            _ => Color::Yellow,
        };
        let category = match RewardCategory::from_str(&category_string) {
            Ok(cat) => cat,
            _ => RewardCategory::Coffee,
        };
        Self {color, category,}
    }
}