use data::task::Task;
use yew::prelude::*;
use yew::format::{Json};
use crate::components::{Popup, TaskEditor, EditResult, IconComponent};
use crate::apis::{complete_task, FetchResponse};
use yew::services::fetch::{FetchTask};
use yew::services::ConsoleService;
use crate::data::*;
use data::icon::*;

pub struct TaskItem {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    fetch_action: Option<FetchTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub task: Box<Task>,
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
                self.props.store.session_user.update(|user_opt| {
                    if let Some(user) = user_opt {
                        user.bspts = total_points;
                        true
                    } else {
                        false
                    }
                });
                let task = self.props.task.clone();
                let id = task.id;
                self.fetch_action = None;
                let remove_task = move |tasks: &mut TaskList| {
                    tasks.remove(id).is_some()
                };
                if self.props.store.todo_tasks.update(remove_task) {
                    self.props.store.done_tasks.update(move |tasks: &mut TaskList| {
                        tasks.push(task);
                        true
                    });
                    true
                } else {
                    let err_msg = format!("Could not find task {} to complete", id);
                    ConsoleService::error(&err_msg);
                    false
                }
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
                self.props.store.act(StoreAction::DeleteTask(self.props.task.id));
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
                    class={"badge loading"}
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

        let badge_class = format!("badge {} {}-theme", is_done_class, self.props.task.icon.get_color().to_string());
        let edit_class = format!("edit button {}-theme", self.props.task.icon.get_color().to_string());
        let done_class = format!("done button {}-theme-inv", self.props.task.icon.get_color().to_string());

        html! {
            <div
                class={badge_class}
                title={&task.description}
            >
                <div class="name">{&task.name}</div>
                <div class="info">{pts_desc}</div>
                <div class="sub-info">{do_by}</div>
                // <i class="thumbnail fas fa-coffee"></i>
                <IconComponent<TaskIcon> icon={self.props.task.icon.clone()} classes="on-task-badge" />
                <div class="badge-line">
                    <span class={edit_class} onclick={click_edit}>{"Edit"}</span>
                    <span class="flex-buffer"></span>
                    {if self.props.task.is_done {
                        html!{<></>}
                    } else {
                        html!{<span class={done_class} onclick={click_done}>{"Done"}</span>}
                    }}
                    
                </div>
                {
                    if self.state.edit_popup {
                        let on_done = self.link.callback(|result: EditResult<Task>| {
                            match result {
                                EditResult::Return(task) => Msg::Update(task),
                                EditResult::Cancel => Msg::CancelEdit,
                                EditResult::Destroy => Msg::DestroySelf,
                            }
                        });
                        
                        html! {
                            <Popup>
                                <TaskEditor
                                    task_to_edit={Some((*self.props.task).clone())}
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