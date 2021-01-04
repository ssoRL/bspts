use data::icon::*;

// pub fn reward_class(cat: &RewardCategory) {
//     match cat {
//         RewardCategory::Cookie => "fa-cookie";
//     }
// }

pub trait Fontable<CAT> {
    fn font_class(self: &Self) -> &'static str;
    fn make_font_class(cat: &CAT, color: &Color) -> &'static str;
}

impl Fontable<RewardCategory> for RewardIcon {
    fn font_class(self: &Self) -> &'static str {
        Self::make_font_class(&self.get_category(), &self.get_color())
    }

    fn make_font_class(cat: &RewardCategory, _: &Color) -> &'static str {
        match cat {
            RewardCategory::Cookie => "fa-cookie-bite",
            RewardCategory::Coffee => "fa-coffee",
            RewardCategory::Television => "fa-tv",
            RewardCategory::Dice => "fa-dice",
            RewardCategory::GamePad => "fa-gamepad",
            RewardCategory::Booze => "fa-glass-cheers",
            RewardCategory::Computer => "fa-laptop",
            RewardCategory::Bong => "fa-bong",
            RewardCategory::Book => "fa-book",
        }
    }
}