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
    _user_callback: StoreListener<User>,
    _todo_tasks_callback: StoreListener<TaskList>,
    _done_tasks_callback: StoreListener<TaskList>,
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
    /// Do nothing
    Noop,
    FetchTasks(bool),
    ReceiveTasks{tasks: ItemPtr<TaskList>, are_done: bool},
    SetPoints(i32),
    OpenTaskCreationComponent,
    NewTaskCommitted(Box<Task>),
    CancelCreateTask,
    HandleError{msg: String, code: Option<StatusCode>},
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Getting tasks");

        // Get the ball rolling on getting the tasks
        link.send_message(Msg::FetchTasks(false));

        let _user_callback = Rc::new(link.callback(|user: ItemPtr<User>| {
            Msg::SetPoints(user.borrow().bspts)
        }));
        let _todo_tasks_callback = Rc::new(link.callback(|tasks: ItemPtr<TaskList>| {
            ConsoleService::log("todo callback");
            Msg::ReceiveTasks{tasks, are_done: false}
        }));
        let _done_tasks_callback = Rc::new(link.callback(|tasks: ItemPtr<TaskList>| {
            ConsoleService::log("done callback");
            Msg::ReceiveTasks{tasks, are_done: true}
        }));
        let store = Rc::clone(&props.store);
        let mut store_mut = store.try_borrow_mut().expect("Could not borrow store for pts update");
        store_mut.user.subscribe(&_user_callback, true);
        store_mut.todo_tasks.subscribe(&_todo_tasks_callback, false);
        store_mut.done_tasks.subscribe(&_done_tasks_callback, false);

        Self {
            state: State {
                todo_tasks: StoreItem::new_ptr(),
                done_tasks: StoreItem::new_ptr(),
                edit_popup: false,
                error_message: None,
                bspts: 0,
                _user_callback,
                _todo_tasks_callback,
                _done_tasks_callback,
            },
            props,
            link,
            fetch_tasks: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::Noop => false,
            // Fetch the tasks that the user has saved
            Msg::FetchTasks(are_done) => {
                let store_clone = self.props.store.clone();
                let callback = self.link.callback(move |response: FetchResponse<Vec<Task>>| {
                    ConsoleService::log(format!("Handle fetch tasks return: {:#?}", response).as_str());
                    match response.into_parts() {
                        (_, Json(Ok(tasks))) => {
                            let mut mut_store = store_clone.try_borrow_mut().expect("Could not borrow to write tasks");
                            mut_store.act(StoreAction::SetTasks{tasks, are_done});
                            Msg::Noop
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
                    ConsoleService::log("Fetching done tasks");
                    get_done_tasks(callback)
                } else {
                    ConsoleService::log("Fetching todo tasks");
                    get_todo_tasks(callback)
                };
                self.fetch_tasks = Some(fetch_task);
                false
            }
            // The message to handle the fetch of tasks coming back
            Msg::ReceiveTasks{tasks, are_done} => {
                if are_done {
                    ConsoleService::log("recv done tasks");
                    self.state.done_tasks = tasks.clone();
                } else {
                    ConsoleService::log("recv todo tasks");
                    self.state.todo_tasks = tasks.clone();
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

        let todo_tasks_html = if self.state.todo_tasks.borrow().is_unset() {
            html! {<span>{"Waiting for tasks to be fetched"}</span>}
        } else {
            self.state.todo_tasks.borrow().to_html(self.props.store.clone())
        };

        
        let done_tasks_html = self.state.done_tasks.borrow().to_html(self.props.store.clone());

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