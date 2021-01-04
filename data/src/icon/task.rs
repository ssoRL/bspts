use crate::icon::*;
use std::str::FromStr;

#[derive(Display, EnumString, EnumIter, Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
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
    color: Color,
    category: TaskCategory,
}

impl Default for TaskIcon {
    fn default() -> Self {
        Self {
            color: Color::Yellow,
            category: TaskCategory::Writing,
        }
    }
}

impl BadgeIcon<TaskCategory> for TaskIcon {
    fn new_ptr(category: TaskCategory, color: Color) -> Box<Self> {
        Box::new(TaskIcon {category, color})
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn get_category(&self) -> TaskCategory {
        self.category
    }

    fn set_category(&mut self, category: TaskCategory) {
        self.category = category;
    }
}

impl Into<String> for TaskIcon {
    fn into(self: Self) -> String {
        let color = &self.color.to_string();
        let cat = &self.category.to_string();
        format!("{}{}", color, cat)
    }
}

impl From<String> for TaskIcon {
    fn from(serialized: String) -> Self {
        let mut color_string = serialized.clone();
        // Remove the leading character (the color info)
        let category_string = color_string.split_off(1);
        // let css : &str = &*color_string;
        let color = match Color::from_str(&color_string) {
            Ok(c) => c,
            _ => Color::Yellow,
        };
        let category = match TaskCategory::from_str(&category_string) {
            Ok(cat) => cat,
            _ => TaskCategory::Writing,
        };
        Self {color, category,}
    }
}