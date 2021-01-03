use yew::services::{
    dialog::DialogService,
    console::{ConsoleService},
};
use yew::format::{Json};
use crate::apis::{commit_new_task, update_task, delete_task, FetchResponse};
use yew::services::fetch::{FetchTask};
use yew::prelude::*;
use crate::components::EditResult;


use data::icon::{BadgeIcon, Color};
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;
use strum_macros::EnumString;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::icon;
use crate::icon::Fontable;

pub struct IconChooser<BI, CAT> where
    CAT: Clone + FromStr + ToString + IntoEnumIterator + 'static,
    BI: BadgeIcon<CAT> + Default + Clone + Fontable + 'static,
{
    state: State<BI>,
    props: Props<BI, CAT>,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props<BI, CAT> where
    CAT: Clone,
    BI: BadgeIcon<CAT> + Default + Clone,
{
    pub icon: Option<BI>,
    pub on_change: Callback<Box<dyn BadgeIcon<CAT>>>,
}

struct State<BI> where
    BI: Fontable,
{
    icon: Box<BI>
}

pub enum Msg {
    UpdateCategory(String),
    UpdateColor(String),
    Noop,
}

impl<BI, CAT> Component for IconChooser<BI, CAT> where
    CAT: Clone + FromStr + ToString + IntoEnumIterator + 'static,
    BI: BadgeIcon<CAT> + Default + Clone + Fontable + 'static,
{
    type Message = Msg;
    type Properties = Props<BI, CAT>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let icon: Box<BI> = match &props.icon {
            None => Box::new(BI::default()),
            Some(i) => Box::new(BI::clone(&*i)),
        };
        Self {
            state : State{ icon },
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
                        self.state.icon.set_category(cat);
                        true
                    }
                    _ => {
                        ConsoleService::error(&format!("Could not parse {} as category", cat_str));
                        false
                    }
                }
            }
            Msg::UpdateColor(color_str) => {
                match Color::from_str(&color_str) {
                    Ok(color) => {
                        ConsoleService::log(&format!("Got color {}", color_str));
                        self.state.icon.set_color(color);
                        true
                    }
                    _ => {
                        ConsoleService::error(&format!("Could not parse {} as color", color_str));
                        false
                    }
                }
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
            html!{ <option value={&cat_str}>{&cat_str}</option> }
        }).collect();

        let category_selector = html!{
            <select onchange={edit_category}>{category_options}</select>
        };

        let color_selector: Html = Color::iter().map(|color: Color| {
            let container_class = format!("{}-theme", color.to_string());
            let font_class = format!("fa {}", self.state.icon.font_class());
            html!{
                <div class={container_class}>
                    <i class={font_class} />
                </div>
            }
        }).collect();
        
        html!{<>
            {category_selector}
            {color_selector}
        </>}

        // let edit_time_unit = self.link.callback(|input: ChangeData| {
        //     match input {
        //         ChangeData::Select(select) => Msg::UpdateFrequencyUnit(select.value()),
        //         _ => panic!("can't get change data value")
        //     }
        // });
        // let frequency_selector = html! {
        //     <div>
        //         <span class="text">{"Do every "}</span>
        //         <input
        //             class="input"
        //             type="number"
        //             oninput={edit_every}
        //             value={self.state.task.frequency.every()}
        //         />
        //         <select onchange={edit_time_unit}>
        //             <option selected={self.state.task.frequency.in_days()} value="d">{"Days"}</option>
        //             <option selected={self.state.task.frequency.in_weeks()} value="w">{"Weeks"}</option>
        //             <option selected={self.state.task.frequency.in_months()} value="m">{"Months"}</option>
        //         </select>
        //         {by_when_selector}
        //     </div>
        // };

        // let delete_this_task = if let Mode::Create = self.state.mode {
        //     // Don't allow destroying a task that doesn't exist
        //     html! { <></> }
        // } else {
        //     let on_destroy = self.link.callback(|_| {Msg::DeleteTask});

        //     html! { <div class="badge-line">
        //         <span class="flex-buffer" />
        //         <a class="delete" onclick={on_destroy}>{"Delete this task"}</a>
        //     </div>}
        // };

        // html! {
        //     <div class="form">
        //         <div>
        //             <input
        //                 type="text"
        //                 class="title-input"
        //                 oninput={edit_name}
        //                 maxlength="20"
        //                 placeholder="Task Name"
        //                 value={self.state.task.name.clone()}
        //             />
        //         </div>
        //         <div>
        //             <span class="text">{"Is worth "}</span>
        //             <input class="input" type="number" oninput={edit_bspts} value={self.state.task.bspts} />
        //             <span class="text">{" bs points"}</span>
        //         </div>
        //         {frequency_selector}
        //         <div><textarea
        //             rows="10" cols="30"
        //             class="description-input"
        //             oninput={edit_desc}
        //             placeholder="Optionally describe the task"
        //             value={self.state.task.description.clone()}
        //         /></div>
        //         <div class="button-line">
        //             <span class="cancel button" onclick={on_cancel}>{"Cancel"}</span>
        //             <span class="flex-buffer"></span>
        //             <span class="save button" onclick={on_save}>{"Save"}</span>
        //         </div>
        //         {delete_this_task}
        //     </div>
        // }
    }
}