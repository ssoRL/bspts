use yew::prelude::*;
use types::task::{Task, NewTask};
use crate::apis::{get_tasks,FetchResponse};
use crate::components::{TaskComponent, TaskCreator};
use yew::format::{Json,Nothing};
use yew::services::fetch::{FetchTask};
use yew::services::console::{ConsoleService};

struct State {
    tasks_option: Option<Vec<Task>>,
    show_create_task_component: bool,
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
    CancelCreateTask,
    MarkTaskCompleted(i32),
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
                show_create_task_component: false,
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
                        Msg::RecieveTasks(vec![])
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
                self.state.show_create_task_component = true;
                true
            },
            Msg::CommitNewTask(new_task) => {
                // TODO: Add code to save a new task after it's created
                // match &mut self.state.tasks_option {
                //     // If there are already tasks, add to them
                //     Some(tasks) => tasks.push(task),
                //     // otherwise start a new list of tasks
                //     None => self.state.tasks_option = Some(vec![task]),
                // }
                self.state.show_create_task_component = false;
                true
            },
            Msg::CancelCreateTask => {
                self.state.show_create_task_component = false;
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

        let new_task_html = if self.state.show_create_task_component {
            let on_create = self.link.callback(|task: NewTask| {Msg::CommitNewTask(task)});
            let on_cancel = self.link.callback(|_| {Msg::CancelCreateTask});
            html! {
                <TaskCreator id={0} on_create={on_create} on_cancel={on_cancel} />
            }
        } else {
            html! {
                <button onclick={self.link.callback(|_| {Msg::OpenTaskCreationComponent})}>{"Add New Task"}</button>
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