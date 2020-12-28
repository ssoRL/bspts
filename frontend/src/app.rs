use yew::prelude::*;
use yew_router::prelude::*;
use yew::services::ConsoleService;

use crate::pages::*;
use crate::data::*;
use std::rc::Rc;
use std::cell::RefCell;
use data::user::User;
use crate::apis::{get_user, FetchResponse};
use yew::services::fetch::{FetchTask};
use yew::format::{Json};

/// Definition of the routes for this app
#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/#signin"]
    SignInPage,
    #[to = "/#signup"]
    SignUpPage,
    #[to = "/"]
    HomePage,
}

#[derive(Clone)]
enum AppState {
    Auth(User),
    NoAuth,
    WaitForAuth,
}

pub struct App {
    state: AppState,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>
}

pub enum Msg {
    RequestAuth,
    ReceiveAuth(Option<User>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::RequestAuth);
        Self { 
            state: AppState::WaitForAuth,
            link,
            fetch_task: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestAuth => {
                let callback = self.link.callback(|response: FetchResponse<Option<User>>| {
                    match response.into_parts() {
                        (_, Json(Ok(is_user))) => {
                            Msg::ReceiveAuth(is_user)
                        }
                        _ => {
                            Msg::ReceiveAuth(None)
                        }
                    }
                });
                self.fetch_task = Some(get_user(callback));
                false
            }
            Msg::ReceiveAuth(is_user) => {
                self.state = match is_user {
                    Some(user) => AppState::Auth(user),
                    None => AppState::NoAuth,
                };
                self.fetch_task = None;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let store: Store = Rc::new(RefCell::new(UnwrappedStore::new()));
        let state = self.state.clone();
        // let state = AppState::NoAuth;
        let render = Router::render(move |route: Route| {
            ConsoleService::log("routing");
            if let AppState::Auth(user) = state.clone() {
                store.borrow_mut().act(StoreAction::SetUser(user.clone()));
                // store_b.user.set(user);
                // If authorized, always go home for now
                html! {<Home store={store.clone()} />}
            } else {
                match route {
                    // If not authenticated, can only visit the login or sign up page
                    Route::SignInPage => html! {<SignIn/>},
                    Route::SignUpPage => html! {<SignUp/>},
                    // Otherwise directed to auth
                    _ => html! {<AuthOptions/>},
                }
            }
        });

        html! {
            <Router<Route, ()> render=render/>
        }
    }
}