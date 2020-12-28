use yew::prelude::*;
use data::task::{Task};
use crate::apis::{get_todo_tasks, get_done_tasks, sign_out_frontend, FetchResponse};
use crate::components::*;
use yew::format::{Json};
use yew::services::fetch::{FetchTask};
use yew::services::console::{ConsoleService};
use http::status::StatusCode;
use crate::data::*;
use data::user::User;
use std::cell::{RefCell};
use std::rc::Rc;

struct State {
    /// The tasks that are shown by this component.
    todo_tasks: ItemPtr<TaskList>,
    /// The tasks that are shown by this component that the user has completed
    done_tasks: ItemPtr<TaskList>,
    edit_popup: bool,
    error_message: Option<String>,
    bspts: i32,
    user_callback: StoreListener<User>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub store: Store,
}

pub struct Home {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    fetch_tasks: Option<FetchTask>,
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

        let user_callback = Rc::new(link.callback(|user: ItemPtr<User>| {
            Msg::SetPoints(user.borrow().bspts)
        }));
        let store = Rc::clone(&props.store);
        let mut store_mut = store.try_borrow_mut().expect("Could not borrow store for pts update");
        store_mut.user.subscribe(&user_callback, true);

        Self {
            state: State {
                todo_tasks: StoreItem::new_ptr(),
                done_tasks: StoreItem::new_ptr(),
                edit_popup: false,
                error_message: None,
                bspts: 0,
                user_callback,
            },
            props,
            link,
            fetch_tasks: None,
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
                    let task_list = TaskList::from_vec(tasks);
                    self.state.done_tasks = Rc::new(RefCell::new(task_list));
                } else {
                    let task_list = TaskList::from_vec(tasks);
                    self.state.todo_tasks = Rc::new(RefCell::new(task_list));
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
                self.state.todo_tasks.borrow_mut().push(task);
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
                        if self.state.todo_tasks.borrow_mut().remove(task_id).is_some() {
                            // Deleted from the todo list
                            true
                        } else if self.state.done_tasks.borrow_mut().remove(task_id).is_some() {
                            // Deleted from the done list
                            true
                        } else {
                            let err_msg = format!("Could not find task {} to remove", task_id);
                            ConsoleService::error(&err_msg);
                            false
                        }
                    }
                    MsgFromTask::Completed(task_id) => {
                        match self.state.todo_tasks.borrow_mut().remove(task_id) {
                            Some(task) => {
                                self.state.done_tasks.borrow_mut().push(task);
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

    fn view(&self) -> Html {
        if let Some(msg) = &self.state.error_message {
            return html! {
                <span>{msg}</span>
            }
        }

        let msg_handler = self.link.callback(|msg| Msg::HandleMsgFromTask(msg));
        let todo_tasks_html = if self.state.todo_tasks.borrow().is_unset() {
            html! {<span>{"Waiting for tasks to be fetched"}</span>}
        } else {
            self.state.todo_tasks.borrow().to_html(&msg_handler, self.props.store.clone())
        };

        
        let done_tasks_html = self.state.done_tasks.borrow().to_html(&msg_handler, self.props.store.clone());

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