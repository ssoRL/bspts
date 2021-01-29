use yew::prelude::*;
use yew::services::fetch::{FetchTask};
use data::user::*;
use yew::format::{Json};
use crate::apis::{sign_in, FetchResponse};
use http::status::StatusCode;
use crate::data::*;

struct State {
    user: NewUser,
    saving: bool,
    error_message: Option<String>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub store: Store,
}

pub struct SignIn {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    fetch_tasks: Option<FetchTask>,
}

pub enum Msg {
    LoginUser,
    SaveUser(User),
    TryAgain(String),
    UpdateUname(String),
    UpdatePassword(String),
}

impl Component for SignIn {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State {
                user: NewUser {
                    uname: "".to_string(),
                    password: "".to_string(),
                },
                saving: false,
                error_message: None,
            },
            props,
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
                            Msg::SaveUser(user)
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
            Msg::SaveUser(user) => {
                self.props.store.act(StoreAction::StartSession(user));
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