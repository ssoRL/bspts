use data::task::Task;
use yew::prelude::*;
use yew::format::{Json};
use crate::pages::MsgFromTask;
use crate::components::{Popup, TaskEditor, EditResult};
use crate::apis::{complete_task, FetchResponse};
use yew::services::fetch::{FetchTask};
use yew::services::ConsoleService;
use crate::data::*;

pub struct TaskItem {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    fetch_action: Option<FetchTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub task: Box<Task>,
    /// Send a message to the parent component
    pub msg_up: Callback<MsgFromTask>,
    pub store: Store,
}

pub struct State {
    /// Show the pop up used to edit this task
    edit_popup: bool,
}

pub enum Msg {
    EditTask,
    CompleteTask,
    TaskIsCompleted(i32),
    Update(Box<Task>),
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
        Self { state, props, link, fetch_action: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EditTask => {
                self.state.edit_popup = true;
                true
            }
            Msg::CompleteTask => {
                let callback = self.link.callback(|response: FetchResponse<i32>| {
                    match response.into_parts() {
                        (_, Json(Ok(pts))) => {
                            Msg::TaskIsCompleted(pts)
                        }
                        (_, _) => {
                            // TODO: Real error handling
                            ConsoleService::error("Could not mark task complete");
                            Msg::CancelEdit
                        }
                    }
                });
                let fetch = complete_task(self.props.task.id, callback);
                self.fetch_action = Some(fetch);
                true
            }
            Msg::TaskIsCompleted(total_points) => {
                ConsoleService::log(&format!("pts so far: {}", total_points));
                self.props.store.borrow_mut().user.update(|user| {
                    user.bspts = total_points;
                    true
                });
                self.fetch_action = None;
                self.props.msg_up.emit(MsgFromTask::Completed(self.props.task.id));
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
                self.state.edit_popup = false;
                self.props.msg_up.emit(MsgFromTask::Deleted(self.props.task.id));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if let Some(_) = self.fetch_action {
            // Return loading indicator
            return html!{
                <div
                    class={"badge task-item loading"}
                />
            }
        }

        let task = &self.props.task;

        let is_done_class = if task.is_done {
           "completed"
        } else {
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
        let click_done = self.link.callback(|_| {Msg::CompleteTask});

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
                    <span class="done button" onclick={click_done}>{"Done"}</span>
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