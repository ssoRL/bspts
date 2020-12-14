use data::task::Task;
use yew::prelude::*;
use crate::components::{Popup, TaskEditor};

pub struct TaskItem {
    state: State,
    props: Props,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub task: Task,
    pub on_tick: Callback<()>,
}

pub struct State {
    edit_popup: bool,
}

pub enum Msg {
    EditTask,
    Update(Task),
    CancelEdit,
}

impl Component for TaskItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            edit_popup: false,
        };
        Self { state, props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EditTask => {
                self.state.edit_popup = true;
                true
            }
            Msg::Update(task) => {
                self.state.edit_popup = false;
                self.props.task = task;
                true
            }
            Msg::CancelEdit => {
                self.state.edit_popup = false;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let on_tick = self.props.on_tick.reform(|_| ());
        let task = &self.props.task;

        let is_done_class = if task.is_done {
           "completed"
        }else {
           "uncompleted"
        };

        let pts_desc = match task.bspts {
           1 => "1 pt".to_string(),
           pts => format!("{} pts", pts),
        };

        let do_by_description = match task.days_to_next_reset {
           days if days < 0 => "yesterday".to_string(),
           0 => "today".to_string(),
           1 => "tomorrow".to_string(),
           // It's in the next week, therefore just say eg "next Saturday"
           2 ..= 7 => format!("next {}", task.next_reset.format("%A")),
           // In the next year use string like "January 13"
           8 ..= 360 => task.next_reset.format("%B %-d").to_string(),
           // Not for a long time, just show iso 8601 format
           _ => task.next_reset.format("%F").to_string(),
        };
        let do_by = format!("Do by {}", do_by_description);

        let click_edit = self.link.callback(|_| {Msg::EditTask});

        html! {
            <div
                class={format!("badge task-item {}", is_done_class)}
                onclick={on_tick}
                title={&task.description}
            >
                <div class="task-name">{&task.name}</div>
                <div class="info">{pts_desc}</div>
                <div class="sub-info">{do_by}</div>
                <i class="thumbnail fas fa-coffee"></i>
                <div class="buttons">
                    <span class="edit button" onclick={click_edit}>{"Edit"}</span>
                    <span class="flex-buffer"></span>
                    <span class="done button">{"Done"}</span>
                </div>
                {
                    if self.state.edit_popup {
                        let save_edits = self.link.callback(|edited_task| {Msg::Update(edited_task)});
                        let cancel_edits = self.link.callback(|_| {Msg::CancelEdit});
                        html! {
                            <Popup>
                                <TaskEditor
                                    on_create={save_edits}
                                    on_cancel={cancel_edits}
                                    task_to_edit={Some(self.props.task.clone())}
                                />
                            </Popup>
                        }
                    } else {
                        html! {<></>}
                    }
                }
            </div>
        }
    }
}