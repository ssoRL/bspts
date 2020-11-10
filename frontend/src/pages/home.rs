use yew::prelude::*;
use crate::types::{Task};
use crate::apis::{get_tasks, set_tasks};
use crate::components::{TaskComponent, TaskCreator};
use yew::format::{Nothing};
use yew::services::fetch::{FetchService,Request,Response,FetchTask};
use yew::services::console::{ConsoleService};

struct State {
    tasks: Vec<Task>,
    show_create_task: bool,
    word_from_hi_api: Option<String>,
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    fetch_hi_task: FetchTask,
}

pub enum Msg {
    SaveTasks,
    CreateNewTask,
    CommitNewTask(Task),
    CancelCreateTask,
    SetHiWord(String)
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
        let get = Request::get("/hi").body(Nothing).unwrap();
        let fetch_hi_task = FetchService::fetch(
            get,
            link.callback(|response: Response<Result<String, _>>| {
                if let (meta, Ok(word)) = response.into_parts() {
                    if meta.status.is_success() {
                        return Msg::SetHiWord(word);
                    }
                }
                Msg::SetHiWord("Oh No!".to_string())
            })
        ).unwrap();

        Self {
            state: State {
                tasks: tasks,
                show_create_task: false,
                word_from_hi_api: None
            },
            link,
            fetch_hi_task
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::SaveTasks => {
                ConsoleService::info("Saving tasks");
                set_tasks(&self.state.tasks);
                false
            },
            Msg::CreateNewTask => {
                self.state.show_create_task = true;
                true
            },
            Msg::CommitNewTask(task) => {
                self.state.tasks.push(task);
                self.state.show_create_task = false;
                true
            },
            Msg::CancelCreateTask => {
                self.state.show_create_task = false;
                true
            },
            Msg::SetHiWord(word) => {
                self.state.word_from_hi_api = Some(word);
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let tasks_html = match self.state.tasks.len() {
            0 => html! {<span>{"No elements at this time :("}</span>},
            _ => self.state.tasks
                .iter()
                .map(|task| {
                    html!{
                        <TaskComponent task={task} on_tick={self.link.callback(|_| Msg::SaveTasks)}></TaskComponent>
                    }
                })
                .collect()
        };

        let new_task_html = if self.state.show_create_task {
            let on_create = self.link.callback(|task: Task| {Msg::CommitNewTask(task)});
            let on_cancel = self.link.callback(|_| {Msg::CancelCreateTask});
            html! {
                <TaskCreator id={0} on_create={on_create} on_cancel={on_cancel} />
            }
        } else {
            html! {
                <button onclick={self.link.callback(|_| {Msg::CreateNewTask})}>{"Add New Task"}</button>
            }
        };

        let word_from_hi_html = match &self.state.word_from_hi_api {
            None => html!{<p>{"No word yet"}</p>},
            Some(word) => html!{<p>{word}</p>}
        };

        html! {
            <>
                <div>{word_from_hi_html}</div>
                <div>{tasks_html}</div>
                <div>{new_task_html}</div>
            </>
        }
    }
}