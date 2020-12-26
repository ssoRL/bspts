use yew::prelude::*;
use data::task::{Task};
use crate::apis::{get_todo_tasks, get_done_tasks, sign_out_frontend, FetchResponse};
use crate::components::*;
use yew::format::{Json};
use yew::services::fetch::{FetchTask};
use yew::services::console::{ConsoleService};
use http::status::StatusCode;
use crate::store::Store;
use crate::data_store::StoreID;
use std::rc::Rc;
use std::sync::Mutex;
use data::user::User;

#[derive(Properties, Clone)]
struct State {
    /// The tasks that are shown by this component.
    todo_tasks: TaskList,
    /// The tasks that are shown by this component that the user has completed
    done_tasks: TaskList,
    edit_popup: bool,
    error_message: Option<String>,
    bspts: i32,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub store: Rc<Mutex<Store>>,
}

pub struct Home {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    fetch_tasks: Option<FetchTask>,
    store_id: Option<StoreID>,
}

pub enum Msg {
    FetchTasks(bool),
    ReceiveTasks{tasks: Vec<Task>, are_done: bool},
    SetPoints(i32),
    OpenTaskCreationComponent,
    NewTaskCommitted(Box<Task>),
    CancelCreateTask,
    HandleError{msg: String, code: Option<StatusCode>},
    HandleMsgFromTask(MsgFromTask),
}

/// Messages that come to this component from the tasks it holds
pub enum MsgFromTask {
    /// The task was deleted and should be removed from the flow
    Deleted(i32),
    /// The task has been completed (for now!)
    Completed(i32),
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Getting tasks");

        // Get the ball rolling on getting the tasks
        link.send_message(Msg::FetchTasks(false));

        Self {
            state: State {
                todo_tasks: TaskList::new(),
                done_tasks: TaskList::new(),
                edit_popup: false,
                error_message: None,
                bspts: 0,
            },
            props,
            link,
            fetch_tasks: None,
            store_id: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            // Fetch the tasks that the user has saved
            Msg::FetchTasks(are_done) => {
                let callback = self.link.callback(move |response: FetchResponse<Vec<Task>>| {
                    match response.into_parts() {
                        (_, Json(Ok(tasks))) => {
                            Msg::ReceiveTasks{tasks, are_done}
                        }
                        (parts, _) => {
                            Msg::HandleError{
                                msg: "Failed to get tasks".to_string(),
                                code: Some(parts.status),
                            }
                        }
                    }
                });
                let fetch_task = if are_done {
                    get_done_tasks(callback)
                } else {
                    get_todo_tasks(callback)
                };
                self.fetch_tasks = Some(fetch_task);
                false
            }
            // The message to handle the fetch of tasks coming back
            Msg::ReceiveTasks{tasks, are_done} => {
                if are_done {
                    self.state.done_tasks = TaskList::from_vec(tasks);
                } else {
                    self.state.todo_tasks = TaskList::from_vec(tasks);
                    // After getting the todo tasks, get the done tasks
                    self.link.send_message(Msg::FetchTasks(true));
                }
                true
            }
            Msg::SetPoints(bspts) => {
                if self.state.bspts == bspts {
                    false
                } else {
                    self.state.bspts = bspts;
                    true
                }
            }
            Msg::OpenTaskCreationComponent => {
                self.state.edit_popup = true;
                true
            }
            Msg::NewTaskCommitted(task) => {
                // The task has been added on the backend, add it to the UI now
                self.state.todo_tasks.push(task);
                self.state.edit_popup = false;
                true
            }
            Msg::CancelCreateTask => {
                self.state.edit_popup = false;
                true
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
                        if self.state.todo_tasks.remove(task_id).is_some() {
                            // Deleted from the todo list
                            true
                        } else if self.state.done_tasks.remove(task_id).is_some() {
                            // Deleted from the done list
                            true
                        } else {
                            let err_msg = format!("Could not find task {} to remove", task_id);
                            ConsoleService::error(&err_msg);
                            false
                        }
                    }
                    MsgFromTask::Completed(task_id) => {
                        match self.state.todo_tasks.remove(task_id) {
                            Some(task) => {
                                self.state.done_tasks.push(task);
                                true
                            }
                            None => {
                                let err_msg = format!("Could not find task {} to complete", task_id);
                                ConsoleService::error(&err_msg);
                                false
                            }
                        }
                    }
                }
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn rendered(self: &mut Self, _: bool) {
        let pts_callback = self.link.callback(|user: Rc<User>| {
            Msg::SetPoints(user.bspts)
        });
        let mut store_mut = self.props.store.lock().unwrap();
        let pts_handle = store_mut.user.subscribe(pts_callback, true);
        self.store_id = Some(pts_handle);
    }

    fn destroy(self: &mut Self) {
        if let Some(id) = self.store_id {
            let mut store_mut = self.props.store.lock().unwrap();
            store_mut.user.unsubscribe(id);
            self.store_id = None;
        }
    }

    fn view(&self) -> Html {
        if let Some(msg) = &self.state.error_message {
            return html! {
                <span>{msg}</span>
            }
        }

        let msg_handler = self.link.callback(|msg| Msg::HandleMsgFromTask(msg));
        let todo_tasks_html = if self.state.todo_tasks.is_unset() {
            html! {<span>{"Waiting for tasks to be fetched"}</span>}
        } else {
            self.state.todo_tasks.to_html(&msg_handler, Rc::clone(&self.props.store))
        };

        
        let done_tasks_html = self.state.done_tasks.to_html(&msg_handler, Rc::clone(&self.props.store));

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
                <div>{format!("pts: {}", self.state.bspts)}</div>
                <div>{new_task_html}</div>
                <div class="task-list-header">{"Things yet to do"}</div>
                <div class="badge-field">{todo_tasks_html}</div>
                <div class="task-list-header">{"Ya' did it!"}</div>
                <div class="badge-field">{done_tasks_html}</div>
            </>
        }
    }
}