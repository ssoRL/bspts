use yew::prelude::*;
use yew::services::fetch::{FetchTask};
use data::user::*;
use yew::format::{Json};
use crate::apis::{sign_in_frontend, sign_in, FetchResponse};
use yew::services::ConsoleService;
use http::status::StatusCode;

struct State {
    user: NewUser,
    saving: bool,
    error_message: Option<String>,
}

pub struct SignIn {
    state: State,
    link: ComponentLink<Self>,
    fetch_tasks: Option<FetchTask>,
}

pub enum Msg {
    LoginUser,
    SaveUserName(String),
    TryAgain(String),
    UpdateUname(String),
    UpdatePassword(String),
}

impl Component for SignIn {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State {
                user: NewUser {
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
            Msg::LoginUser => {
                let callback = self.link.callback(|jwt_response: FetchResponse<User>| {
                    match jwt_response.into_parts() {
                        (head, _) if head.status == StatusCode::NOT_FOUND => {
                            Msg::TryAgain("No user was found with that username".to_string())
                        }
                        (head, _) if head.status == StatusCode::UNAUTHORIZED => {
                            Msg::TryAgain("Incorrect Password".to_string())
                        }
                        (_, Json(Ok(user))) => {
                            Msg::SaveUserName(user.uname)
                        }
                        _ => {
                            Msg::TryAgain("There was some problem, please try again".to_string())
                        }
                    }
                });
                let fetch_task = sign_in(self.state.user.clone(), callback);
                self.fetch_tasks = Some(fetch_task);
                self.state.saving = true;
                true
            },
            Msg::SaveUserName(jwt) => {
                sign_in_frontend(jwt);
                true
            },
            Msg::TryAgain(error) => {
                self.state.error_message = Some(error);
                self.state.saving = false;
                true
            },
            Msg::UpdateUname(uname) => {
                self.state.user.uname = uname;
                true
            },
            Msg::UpdatePassword(password) => {
                self.state.user.password = password;
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        ConsoleService::log("Updating Sign In");
        let error_message = match &self.state.error_message {
            Some(msg) => {
                html! {<span>{msg}</span>}
            },
            None => html! {<></>}
        };

        let edit_uname = self.link.callback(|input: InputData| {Msg::UpdateUname(input.value)});
        let edit_pw = self.link.callback(|input: InputData| {Msg::UpdatePassword(input.value)});
        let on_login = self.link.callback(|_| {Msg::LoginUser});

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
                    <span class="save button" onclick={on_login}>{"Sign In"}</span>
                </div>
            </div>
        }
    }
}