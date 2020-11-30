use yew::prelude::*;
use yew::services::fetch::{FetchTask};
use yew::services::console::{ConsoleService};

struct State {
    error_message: Option<String>,
}

pub struct SignIn {
    state: State,
    link: ComponentLink<Self>,
    fetch_tasks: Option<FetchTask>,
}

pub enum Msg {
    CreateNewAccount,
    LoginExistingUser,
}

impl Component for SignIn {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State {
                error_message: None,
            },
            link,
            fetch_tasks: None
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        false
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

        ConsoleService::log("Viewing login route");

        html! {<span>{"TODO: impl this"}</span>}
    }
}