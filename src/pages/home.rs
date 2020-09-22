use yew::prelude::*;
use crate::types::{Task};
use crate::apis::{get_tasks, set_tasks};
use crate::components::{TaskComponent, TaskCreator};
use yew::services::console::{ConsoleService};

struct State {
    tasks: Vec<Task>,
    show_create_task: bool
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SaveTasks,
    CreateNewTask,
    CommitNewTask(Task),
    CancelCreateTask,
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

        Self {
            state: State {
                tasks: tasks,
                show_create_task: false,
            },
            link
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

        html! {
            <>
                <div>{tasks_html}</div>
                <div>{new_task_html}</div>
            </>
        }
    }
}