use yew::prelude::*;
use data::task::{Task};
use crate::apis::{get_tasks, sign_out_frontend, FetchResponse};
use crate::components::{TaskItem, TaskEditor, EditResult, Popup};
use yew::format::{Json};
use yew::services::fetch::{FetchTask};
use yew::services::console::{ConsoleService};
use http::status::StatusCode;

#[derive(Properties, Clone)]

struct State {
    /// The tasks that are shown by this component.
    /// None if no tasks have returned from fetch yet.
    /// Empty list if this user has no tasks.
    tasks_option: Option<Vec<Task>>,
    edit_popup: bool,
    error_message: Option<String>,
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    fetch_tasks: Option<FetchTask>,
}

pub enum Msg {
    FetchTasks,
    ReceiveTasks{tasks: Vec<Task>, are_done: bool},
    OpenTaskCreationComponent,
    NewTaskCommitted(Task),
    CancelCreateTask,
    MarkTaskCompleted(i32),
    HandleError{msg: String, code: Option<StatusCode>},
    HandleMsgFromTask(MsgFromTask),
}

/// Messages that come to this component from the tasks it holds
pub enum MsgFromTask {
    /// The task was deleted and should be removed from the flow
    Deleted(i32),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Getting tasks");

        // Get the ball rolling on getting the tasks
        link.send_message(Msg::FetchTasks);

        Self {
            state: State {
                tasks_option: None,
                edit_popup: false,
                error_message: None
            },
            link,
            fetch_tasks: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            // Fetch the tasks that the user has saved
            Msg::FetchTasks => {
                let callback = self.link.callback(|response: FetchResponse<Vec<Task>>| {
                    match response.into_parts() {
                        (_, Json(Ok(tasks))) => {
                            Msg::ReceiveTasks{tasks, are_done: false}
                        }
                        (parts, _) => {
                            Msg::HandleError{
                                msg: "Failed to get tasks".to_string(),
                                code: Some(parts.status),
                            }
                        }
                    }
                });
                let fetch_task = get_tasks(callback);
                self.fetch_tasks = Some(fetch_task);
                false
            }
            // The message to handle the fetch of tasks coming back
            Msg::ReceiveTasks{tasks, are_done} => {
                self.state.tasks_option= Some(tasks);
                true
            }
            Msg::OpenTaskCreationComponent => {
                self.state.edit_popup = true;
                true
            }
            Msg::NewTaskCommitted(task) => {
                // The task has been added on the backend, add it to the UI now
                match &mut self.state.tasks_option {
                    // If there are already tasks, add to them
                    Some(tasks) => tasks.push(task),
                    // otherwise start a new list of tasks
                    None => self.state.tasks_option = Some(vec![task]),
                };
                self.state.edit_popup = false;
                true
            }
            Msg::CancelCreateTask => {
                self.state.edit_popup = false;
                true
            }
            Msg::MarkTaskCompleted(_task_id) => {
                // Don't do anything here atm
                false
            }
            Msg::HandleError{msg, code} => {
                if let Some(StatusCode::UNAUTHORIZED) = code {
                    sign_out_frontend();
                } else {
                    self.state.error_message = Some(msg);
                }
                true
            }
            Msg::HandleMsgFromTask(msg) => {
                match msg {
                    MsgFromTask::Deleted(task_id) => {
                        if let Some(tasks) = &mut self.state.tasks_option {
                            if let Some(task_index) = tasks.iter().position(|t| t.id == task_id) {
                                tasks.remove(task_index);
                            } else {
                                ConsoleService::error(format!("No task {} to remove", task_id).as_str());
                            }
                        }
                    }
                }
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        if let Some(msg) = &self.state.error_message {
            return html! {
                <span>{msg}</span>
            }
        }

        let tasks_html = match &self.state.tasks_option {
            // Show a loading message for the time being
            // this short circuits the rendering to just show this span
            None => return html! {<span>{"Waiting for tasks to be fetched"}</span>},
            // The tasks have been fetched
            Some(tasks) =>{
                match tasks.len() {
                    0 => html! {<span>{"No tasks have been added to your list yet"}</span>},
                    _ => tasks
                    .iter()
                    .map(|task| {
                        let task_id = task.id;
                        html!{
                            <TaskItem
                                task={task}
                                msg_up={self.link.callback(|msg| Msg::HandleMsgFromTask(msg))}
                            />
                        }
                    })
                    .collect()
                }
            }
        };

        let new_task_html = if self.state.edit_popup {
            let on_done = self.link.callback(|result: EditResult| {
                match result {
                    EditResult::Return(task) => Msg::NewTaskCommitted(task),
                    _ => Msg::CancelCreateTask
                }
                
            });

            html! {
                <Popup>
                    <TaskEditor task_to_edit={None} on_done={on_done} />
                </Popup>
            }
        } else {
            html! {
                <div 
                    class="add-new-task button"
                    onclick={self.link.callback(|_| {Msg::OpenTaskCreationComponent})}
                >
                    {"Add New Task"}
                </div>
            }
        };

        html! {
            <>
                <div>{new_task_html}</div>
                <div class="badge-field">{tasks_html}</div>
            </>
        }
    }
}