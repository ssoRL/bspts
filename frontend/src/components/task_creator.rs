use crate::types::Task;
use yew::services::console::{ConsoleService};
use yew::prelude::*;

pub struct TaskCreator {
    state: State,
    props: Props,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i32,
    pub on_create: Callback<Task>,
    pub on_cancel: Callback<()>,
}

pub struct State {
    pub task: Task,
}

pub enum Msg {
    UpdateName(String),
    CreateTask,
    CancelCreate,
}

impl Component for TaskCreator {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state : State{
                task: Task::new(props.id),
            },
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.state.task.name = name;
                ConsoleService::info(&(format!("Name is {}", self.state.task.name)));
                false
            },
            Msg::CreateTask => {
                self.props.on_create.emit(self.state.task.clone());
                true
            },
            Msg::CancelCreate => {
                self.props.on_cancel.emit(());
                true
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let input_name = self.link.callback(|input: InputData| {Msg::UpdateName(input.value)});
        let on_save = self.link.callback(|_| {Msg::CreateTask});
        let on_cancel = self.link.callback(|_| {Msg::CancelCreate});

        html! {
            <div>
                <input type="text" oninput={input_name} />
                <button onclick={on_save}>{"Save"}</button>
                <button onclick={on_cancel}>{"Cancel"}</button>
            </div>
        }
    }
}