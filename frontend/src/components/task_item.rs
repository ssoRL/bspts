use types::task::Task;
use yew::prelude::*;
use chrono::{Local};
use std::fmt::Display;
use yew::services::ConsoleService;

pub struct TaskItem {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub task: Task,
    pub on_tick: Callback<()>,
}

impl Component for TaskItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let on_tick = self.props.on_tick.reform(|_| ());
        let task = &self.props.task;

        ConsoleService::log(&format!("making component for task {}", task.id));

        let is_done_class = if task.is_done {
            "completed"
         }else {
            "uncompleted"
         };

         ConsoleService::log(&format!("It is {}", is_done_class));

         let pts_desc = match task.bspts {
            1 => "1 pt".to_string(),
            pts => format!("{} pts", pts),
         };

         ConsoleService::log(&format!("Worth {}", pts_desc));

         // TODO: Actually fuck with this
         //let today = Local::today().naive_local();
         //let due_in = today.signed_duration_since(task.next_reset);
         let do_by_desc = match 1 {
            days if days < 0 => "Yesterday".to_string(),
            0 | 1 => "today".to_string(),
            2 => "tomorrow".to_string(),
            3 ..= 7 => {
                // It's in the next week, therefore just say eg "next Saturday"
                format!("next {}", task.next_reset.format("%A"))
            },
            _ => task.next_reset.format("%-d %B, %C%y").to_string()
         };
         let do_by = format!("Do by {}", do_by_desc);

         ConsoleService::log(&do_by);

        html! {
            <div class={format!("checkbox {}", is_done_class)} onclick={on_tick}>
                <div class="task-name">{&task.name}</div>
                <div class="info-line">
                    <span class="points">{pts_desc}</span>
                    <span class="do-by">{do_by}</span>
                </div>
                <i class="thumbnail fas fa-coffee"></i>
            </div>
        }
    }
}