use yew::prelude::*;
use crate::types::{Task};
use crate::apis::{get_tasks, set_tasks};

struct State {
    tasks_option: Option<Vec<Task>>
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    GetTasks,
    SaveTasks
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetTasks);

        Self {
            state: State {
                tasks_option: None
            },
            link
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetTasks => {
                let tasks = get_tasks();
                self.state.tasks_option = Some(tasks);
                true
            },
            Msg::SaveTasks => {
                match &self.state.tasks_option {
                    Some(tasks) => {
                        set_tasks(tasks);
                    },
                    None => {}
                }
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let content = match &self.state.tasks_option {
            Some(tasks) => tasks
                .iter()
                .map(|task| {
                    html! {
                        <div>
                            <input type="checkbox" id={&task.id} name={&task.name} checked={task.checked} />
                            <span>{&task.name}</span>
                        </div>
                    }
                })
                .collect(),
            None => html! {<span>{"Loading..."}</span>}
        };

        let on_save = self.link.callback(move |_| {Msg::SaveTasks});

        html! {
            <>
                <div>{content}</div>
                <button onclick={on_save}>{"Save"}</button>
            </>
        }
    }
}