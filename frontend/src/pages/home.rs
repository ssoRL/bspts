use yew::prelude::*;
use crate::apis::{sign_out_frontend};
use yew::services::{
    console::ConsoleService,
};
use http::status::StatusCode;

struct State {
    error_message: Option<String>,
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    HandleError{msg: String, code: Option<StatusCode>},
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Creating home");

        ConsoleService::info("Creating self");
        Self {
            state: State {
                error_message: None,
            },
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
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
            return html! {<span>{msg}</span>}
        }
        html! {
            <p>{"Welcome to BSPTS, maybe head over to tasks and set some up!"}</p>
        }
    }
}