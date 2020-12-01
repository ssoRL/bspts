use yew::prelude::*;
use yew::services::fetch::{FetchTask};
use data::user::*;
use yew::format::{Json};
use crate::apis::{set_jwt, signup, FetchResponse};
use yew_router::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
use crate::app;

struct State {
    new_user: NewUser,
    saving: bool,
    error_message: Option<String>,
}

pub struct SignUp {
    state: State,
    link: ComponentLink<Self>,
    fetch_tasks: Option<FetchTask>,
}

pub enum Msg {
    CreateNewUser,
    SaveJwt(String),
    TryAgain(String),
    UpdateUname(String),
    UpdatePassword(String),
}

impl Component for SignUp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State {
                new_user: NewUser {
                    uname: "".to_string(),
                    password: "".to_string(),
                },
                saving: false,
                error_message: None,
            },
            link,
            fetch_tasks: None
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::CreateNewUser => {
                let callback = self.link.callback(|jwt_response: FetchResponse<String>| {
                    if let (_, Json(Ok(jwt))) = jwt_response.into_parts() {
                        Msg::SaveJwt(jwt)
                    } else {
                        Msg::TryAgain("There was an issue creating that user".to_string())
                    }
                });
                let fetch_task = signup(self.state.new_user.clone(), callback);
                self.fetch_tasks = Some(fetch_task);
                self.state.saving = true;
                true
            },
            Msg::SaveJwt(jwt) => {
                set_jwt(jwt);
                let mut agent_dispatch: RouteAgentDispatcher<()> = RouteAgentDispatcher::default();
                agent_dispatch.send(ChangeRoute(app::Route::HomePage.into()));
                true
            },
            Msg::TryAgain(error) => {
                self.state.error_message = Some(error);
                self.state.saving = false;
                true
            },
            Msg::UpdateUname(uname) => {
                self.state.new_user.uname = uname;
                true
            },
            Msg::UpdatePassword(password) => {
                self.state.new_user.password = password;
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let error_message = match &self.state.error_message {
            Some(msg) => {
                html! {<span>{msg}</span>}
            },
            None => html! {<></>}
        };

        let edit_uname = self.link.callback(|input: InputData| {Msg::UpdateUname(input.value)});
        let edit_pw = self.link.callback(|input: InputData| {Msg::UpdatePassword(input.value)});
        let on_save = self.link.callback(|_| {Msg::CreateNewUser});

        html! {
            <div class="form">
                {error_message}
                <div>
                    <span class="text">{"User Name: "}</span>
                    <input class="input" type="text" oninput={edit_uname} />
                </div>
                <div>
                    <span class="text">{"Password "}</span>
                    <input class="input" type="password" oninput={edit_pw} />
                </div>
                <div class="button-line">
                    <span class="flex-buffer"></span>
                    <span class="save button" onclick={on_save}>{"Save"}</span>
                </div>
            </div>
        }
    }
}