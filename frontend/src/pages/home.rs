use yew::prelude::*;
use types::task::{Task, NewTask};
use crate::apis::{get_tasks,commit_new_task,FetchResponse};
use crate::components::{TaskComponent, TaskCreator};
use yew::format::{Json};
use yew::services::fetch::{FetchTask};
use yew::services::console::{ConsoleService};

enum NewTaskComponentState {
    Closed,
    Open,
    Committing,
}

struct State {
    tasks_option: Option<Vec<Task>>,
    create_task: NewTaskComponentState,
    error_message: Option<String>,
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    fetch_tasks: Option<FetchTask>,
}

pub enum Msg {
    FetchTasks,
    RecieveTasks(Vec<Task>),
    OpenTaskCreationComponent,
    CommitNewTask(NewTask),
    NewTaskCommitted(Task),
    CancelCreateTask,
    MarkTaskCompleted(i32),
    ShowError(String),
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
                create_task: NewTaskComponentState::Closed,
                error_message: None
            },
            link,
            fetch_tasks: None
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            // Fetch the tasks that the user has saved
            Msg::FetchTasks => {
                let fetch_tasks_task = get_tasks(self.link.callback(|response: FetchResponse<Vec<Task>>| {
                    if let (_, Json(Ok(tasks))) = response.into_parts() {
                        Msg::RecieveTasks(tasks)
                    } else {
                        // TODO: show an error in this case
                        Msg::ShowError("Failed to deserialize tasks".to_string())
                    }
                }));
                // Save the fetch task in the component so it's not canceled by yew
                self.fetch_tasks = Some(fetch_tasks_task);
                false
            },
            // The message to handle the fetch of tasks coming back
            Msg::RecieveTasks(tasks) => {
                self.state.tasks_option= Some(tasks);
                true
            }
            Msg::OpenTaskCreationComponent => {
                self.state.create_task = NewTaskComponentState::Open;
                true
            },
            Msg::CommitNewTask(new_task) => {
                self.state.create_task = NewTaskComponentState::Committing;
                let task_committed_callback = self.link.callback(|response: FetchResponse<Task>| {
                    if let (_, Json(Ok(task))) = response.into_parts() {
                        Msg::NewTaskCommitted(task)
                    } else {
                        // TODO: error
                        Msg::CancelCreateTask
                    }
                });
                self.fetch_tasks = Some(commit_new_task(new_task, task_committed_callback));
                true
            },
            Msg::NewTaskCommitted(task) => {
                // The task has been added on the backend, add it to the UI now
                match &mut self.state.tasks_option {
                    // If there are already tasks, add to them
                    Some(tasks) => tasks.push(task),
                    // otherwise start a new list of tasks
                    None => self.state.tasks_option = Some(vec![task]),
                };
                self.state.create_task = NewTaskComponentState::Closed;
                true
            }
            Msg::CancelCreateTask => {
                self.state.create_task = NewTaskComponentState::Closed;
                true
            },
            Msg::MarkTaskCompleted(task_id) => {
                // Don't do anything here atm
                false
            },
            Msg::ShowError(msg) => {
                self.state.error_message = Some(msg);
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
                            <TaskComponent task={task} on_tick={self.link.callback(move |_| Msg::MarkTaskCompleted(task_id))}></TaskComponent>
                        }
                    })
                    .collect()
                }
            }
        };

        let new_task_html = match self.state.create_task {
            NewTaskComponentState::Open => {
                let on_create = self.link.callback(|task: NewTask| {Msg::CommitNewTask(task)});
                let on_cancel = self.link.callback(|_| {Msg::CancelCreateTask});
                html! {
                    <TaskCreator id={0} on_create={on_create} on_cancel={on_cancel} />
                }
            },
            NewTaskComponentState::Closed => {
                html! {
                    <button onclick={self.link.callback(|_| {Msg::OpenTaskCreationComponent})}>{"Add New Task"}</button>
                }
            },
            NewTaskComponentState::Committing => {
                html! {
                    <span>{"Committing the new task!"}</span>
                }
            },
        };

        html! {
            <>
                <div>{tasks_html}</div>
                <div>{new_task_html}</div>
            </>
        }
    }
}