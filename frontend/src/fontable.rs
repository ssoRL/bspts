use data::icon::*;

pub trait Fontable {
    fn font_class(self: &Self) -> &'static str;
    // fn make_font_class(cat: &CAT, color: &Color) -> &'static str;
}

impl Fontable for RewardIcon {
    fn font_class(self: &Self) -> &'static str {
        let make_font_class = |cat: &RewardCategory| -> &'static str {
            match cat {
                RewardCategory::Cookie => "fa-cookie-bite",
                RewardCategory::Coffee => "fa-coffee",
                RewardCategory::Television => "fa-tv",
                RewardCategory::Dice => "fa-dice",
                RewardCategory::GamePad => "fa-gamepad",
                RewardCategory::Booze => "fa-glass-cheers",
                RewardCategory::Computer => "fa-laptop",
                RewardCategory::NailPolish => "fa-bong",
                RewardCategory::Book => "fa-book-open",
            }
        };

        make_font_class(&self.get_category())
    }
}

impl Fontable for TaskIcon {
    fn font_class(self: &Self) -> &'static str {
        let make_font_class = |cat: &TaskCategory, color: &Color| -> &'static str {
            match cat {
                TaskCategory::Animal => match color {
                    Color::Yellow => "fa-dog",
                    Color::Red => "fa-cat",
                    Color::Blue => "fa-crow",
                    Color::Green => "fa-dragon",
                },
                TaskCategory::Exercise => match color {
                    Color::Blue => "fa-dumbbell",
                    Color::Red => "fa-bicycle",
                    _ => "fa-running",
                },
                TaskCategory::Read => "fa-book",
                TaskCategory::Clean => match color {
                    Color::Yellow => "fa-broom",
                    // Maybe one day this is a red apple core for compost...
                    Color::Red => "fa-trash",
                    Color::Blue => "fa-recycle",
                    Color::Green => "fa-dumpster",
                },
                TaskCategory::Code => match color {
                    Color::Yellow => "fa-laptop-code",
                    Color::Red => "fa-file-code",
                    Color::Blue => "fa-code",
                    Color::Green => "fa-terminal",
                },
                TaskCategory::Communicate => match color {
                    Color::Yellow => "fa-laptop-envelope",
                    Color::Red => "fa-phone",
                    Color::Blue => "fa-reply-all",
                    Color::Green => "fa-comment-dots",
                },
                TaskCategory::Music => match color {
                    Color::Yellow => "fa-guitar",
                    Color::Red => "fa-drum",
                    Color::Blue => "fa-microphone-alt",
                    Color::Green => "fa-music",
                },
                TaskCategory::Writing => match color {
                    Color::Yellow => "fa-pencil-alt",
                    Color::Red => "fa-pen-fancy",
                    Color::Blue => "fa-i-cursor",
                    Color::Green => "fa-edit",
                },
            }
        };

        make_font_class(&self.get_category(), &self.get_color())
    }
}