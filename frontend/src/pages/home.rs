use yew::prelude::*;
use types::task::{Task};
use crate::apis::{get_tasks, set_tasks};
use crate::components::{TaskComponent, TaskCreator};
use yew::format::{Nothing};
use yew::services::fetch::{FetchService,Request,Response,FetchTask};
use yew::services::console::{ConsoleService};

struct State {
    tasks_option: Option<Vec<Task>>,
    show_create_task: bool
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    fetch_tasks: FetchTask,
}

pub enum Msg {
    FetchTasks,
    RecieveTasks(Vec<Task>),
    StartCreatingNewTask,
    CommitNewTask(Task),
    CancelCreateTask,
    MarkTaskCompleted(i32),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Getting tasks");
        let tasks_result = get_tasks();
        // TODO: error handling
        let tasks = match tasks_result {
            Ok(tasks) => tasks,
            Err(_) => vec![]
        };

        // Send out the fetch to populate the word from hi
        let get = Request::get("/task").body(Nothing).unwrap();
        let fetch_tasks = FetchService::fetch(
            get,
            link.callback(|response: Response<Result<String, _>>| {
                if let (meta, Ok(serialized_tasks)) = response.into_parts() {
                    if meta.status.is_success() {
                        // Deserialize the message
                        let tasks: Vec<Task> = serde_json::from_str(&serialized_tasks).unwrap();
                        return Msg::RecieveTasks(tasks);
                    }
                }
                Msg::RecieveTasks(vec![])
            })
        ).unwrap();

        Self {
            state: State {
                tasks_option: None,
                show_create_task: false,
            },
            link,
            fetch_tasks
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            // Fetch the tasks that the user has saved
            Msg::FetchTasks => {
                false
            },
            // The message to handle the fetch of tasks coming back
            Msg::RecieveTasks(tasks) => {
                self.state.tasks_option= Some(tasks);
                true
            }
            Msg::StartCreatingNewTask => {
                self.state.show_create_task = true;
                true
            },
            Msg::CommitNewTask(task) => {
                match &mut self.state.tasks_option {
                    // If there are already tasks, add to them
                    Some(tasks) => tasks.push(task),
                    // otherwise start a new list of tasks
                    None => self.state.tasks_option = Some(vec![task]),
                }
                self.state.show_create_task = false;
                true
            },
            Msg::CancelCreateTask => {
                self.state.show_create_task = false;
                true
            },
            Msg::MarkTaskCompleted(task_id) => {
                // Don't do anything here atm
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let tasks_html = match &self.state.tasks_option {
            // Show a loading message for the time being, this short circuits the rendering to just show this span
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

        let new_task_html = if self.state.show_create_task {
            let on_create = self.link.callback(|task: Task| {Msg::CommitNewTask(task)});
            let on_cancel = self.link.callback(|_| {Msg::CancelCreateTask});
            html! {
                <TaskCreator id={0} on_create={on_create} on_cancel={on_cancel} />
            }
        } else {
            html! {
                <button onclick={self.link.callback(|_| {Msg::StartCreatingNewTask})}>{"Add New Task"}</button>
            }
        };

        html! {
            <>
                <div>{tasks_html}</div>
                <div>{new_task_html}</div>
            </>
        }
    }
}