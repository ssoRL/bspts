use data::task::Task;
use yew::prelude::*;
use crate::pages::MsgFromTask;
use crate::components::{Popup, TaskEditor, EditResult};

pub struct TaskItem {
    state: State,
    props: Props,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub task: Task,
    /// Send a message to the parent component
    pub msg_up: Callback<MsgFromTask>,
}

pub struct State {
    /// Show the pop up used to edit this task
    edit_popup: bool,
}

pub enum Msg {
    EditTask,
    Update(Task),
    CancelEdit,
    DestroySelf,
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
            Msg::DestroySelf => {
                self.props.msg_up.emit(MsgFromTask::Deleted(self.props.task.id));
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
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
                // TODO: allow the user to complete tasks
                //onclick={on_tick}
                title={&task.description}
            >
                <div class="task-name">{&task.name}</div>
                <div class="info">{pts_desc}</div>
                <div class="sub-info">{do_by}</div>
                <i class="thumbnail fas fa-coffee"></i>
                <div class="badge-line">
                    <span class="edit button" onclick={click_edit}>{"Edit"}</span>
                    <span class="flex-buffer"></span>
                    <span class="done button">{"Done"}</span>
                </div>
                {
                    if self.state.edit_popup {
                        let on_done = self.link.callback(|result: EditResult| {
                            match result {
                                EditResult::Return(task) => Msg::Update(task),
                                EditResult::Cancel => Msg::CancelEdit,
                                EditResult::Destroy => Msg::DestroySelf,
                            }
                        });
                        
                        html! {
                            <Popup>
                                <TaskEditor
                                    task_to_edit={Some(self.props.task.clone())}
                                    on_done={on_done}
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