use yew::prelude::*;
use yew::services::console::ConsoleService;


use data::icon::{BadgeIcon, Color};
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;
use strum_macros::EnumString;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::fontable::Fontable;

pub struct IconChooser<BI, CAT> where
    CAT: Clone + Copy + FromStr + ToString + IntoEnumIterator + Eq + 'static,
    BI: BadgeIcon<CAT> + Default + Clone + Fontable<CAT> + 'static,
{
    state: State<CAT>,
    props: Props<BI>,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props<BI> where
    BI: Default + Clone,
{
    pub icon: Option<BI>,
    pub on_change: Callback<Box<BI>>,
}

struct State<CAT> where
    CAT: Clone,
{
    // icon: Box<BI>
    category: CAT,
    color_o: Option<Color>,
}

pub enum Msg {
    UpdateCategory(String),
    UpdateColor(Color),
    Noop,
}

impl<BI, CAT> Component for IconChooser<BI, CAT> where
    CAT: Clone + Copy + FromStr + ToString + IntoEnumIterator + Eq + 'static,
    BI: BadgeIcon<CAT> + Default + Clone + Fontable<CAT> + 'static,
{
    type Message = Msg;
    type Properties = Props<BI>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let (category, color_o) = match &props.icon {
            Some(i) => {
                (i.get_category(), Some(i.get_color()))
            },
            None => {
                let default = BI::default();
                (default.get_category(), None)
            },
        };
        Self {
            state : State{
                category,
                color_o,
            },
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateCategory(cat_str) => {
                match CAT::from_str(&cat_str) {
                    Ok(cat) => {
                        ConsoleService::log(&format!("Got cat {}", cat_str));
                        self.state.category = cat;
                        self.state.color_o = None;
                        true
                    }
                    _ => {
                        ConsoleService::error(&format!("BSPTS Error: Could not parse {} as category", cat_str));
                        false
                    }
                }
            }
            Msg::UpdateColor(color) => {
                self.state.color_o = Some(color);
                let icon_ptr = BadgeIcon::<CAT>::new_ptr(self.state.category, color);
                self.props.on_change.emit(icon_ptr);
                true
            }
            Noop => false
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let edit_category = self.link.callback(|change: ChangeData| {
            match change {
                ChangeData::Select(select) => {
                    Msg::UpdateCategory(select.value())
                }
                _ => Msg::Noop
            }
        });
        // let edit_color = |color: String| {self.link.callback(|_:()| {Msg::UpdateColor(color.clone()})};

        let category_options: Html = CAT::iter().map(|cat: CAT| {
            let cat_str = cat.to_string();
            let is_selected = cat == self.state.category;
            html!{ <option selected={is_selected} value={&cat_str}>{&cat_str}</option> }
        }).collect();

        let category_selector = html!{
            <select onchange={edit_category}>{category_options}</select>
        };

        // Check if a given color is the current color
        let color_matches = |color: Color| {
            if let Some(self_color) = &self.state.color_o {
                *self_color == color
            } else {
                false
            }
        };

        let color_selector: Html = Color::iter().map(|color: Color| {
            let is_selected = if color_matches(color) {"yes"} else {"no"};
            let selected_class = format!("color-selected {}", is_selected);
            let container_class = format!("color-chooser {}-theme", color.to_string());
            let font_class = format!("fa {}", BI::make_font_class(&self.state.category, &color));
            let choose_color = self.link.callback(move |_| {Msg::UpdateColor(color)});
            html!{
                <span class={selected_class}>
                    <span class={container_class} onclick={choose_color}>
                        <i class={font_class} />
                    </span>
                </span>
            }
        }).collect();
        
        html!{<>
            {category_selector}
            <div>{color_selector}</div>
        </>}
    }
}