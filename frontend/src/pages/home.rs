use yew::prelude::*;
use yew::services::{
    console::ConsoleService,
};

struct State {
    error_message: Option<String>,
}

pub struct Home {
    state: State,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Creating home");
        Self {
            state: State {
                error_message: None,
            },
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        if let Some(msg) = &self.state.error_message {
            return html! {<span>{msg}</span>}
        }
        html! {
            <p>{"Welcome to BSPTS, maybe head over to tasks and set some up!"}</p>
        }
    }
}