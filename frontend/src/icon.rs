use data::icon::*;

// pub fn reward_class(cat: &RewardCategory) {
//     match cat {
//         RewardCategory::Cookie => "fa-cookie";
//     }
// }

pub trait Fontable {
    fn font_class(self: &Self) -> &'static str;
}

impl Fontable for RewardIcon {
    fn font_class(self: &Self) -> &'static str {
        match self.get_category() {
            RewardCategory::Cookie => "fa-cookie",
            RewardCategory::Coffee => "fa-coffee",
            _ => "fa-bone",
        }
    }
}