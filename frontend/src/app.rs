use yew::prelude::*;
use yew_router::prelude::*;
use yew::services::ConsoleService;

use crate::pages::*;
use crate::data::*;
use std::rc::Rc;
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

pub struct State {
    store: Store,
    user: ItemPtr<Option<User>>,
    _user_callback: StoreListener<Option<User>>,
}

pub struct App {
    state: State,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>
}

pub enum Msg {
    DoRender,
    DoNotRender,
    RequestAuth,
    ReceiveAuth(ItemPtr<Option<User>>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let store: Store = Rc::new(UnwrappedStore::new());

        let _user_callback = store.session_user.subscribe(
            link.callback(|user| {
                Msg::ReceiveAuth(user)
            })
            , false
        );

        link.send_message(Msg::RequestAuth);

        Self { 
            state: State {
                store,
                user: StoreItem::new_ptr(),
                _user_callback,
            },
            link,
            fetch_task: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestAuth => {
                let store_clone = self.state.store.clone();
                let callback = self.link.callback(move |response: FetchResponse<Option<User>>| {
                    let ret = match response.into_parts() {
                        (_, Json(Ok(Some(user)))) => {
                            ConsoleService::log("got user back to app");
                            store_clone.act(StoreAction::StartSession(user));
                            Msg::DoRender
                        }
                        _ => {
                            Msg::DoNotRender
                        }
                    };
                    ConsoleService::log("out of resp");
                    ret
                });
                self.fetch_task = Some(get_user(callback));
                false
            }
            Msg::ReceiveAuth(user) => {
                ConsoleService::log("recv auth");
                self.state.user = user;
                false
            }
            Msg::DoRender => true,
            Msg::DoNotRender => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let user_opt = self.state.user.clone();
        let store = self.state.store.clone();

        let render = Router::render(move |route: Route| {
            ConsoleService::log("routing");
            let has_auth = {
                let uob = user_opt.borrow();
                uob.is_some()
            };
            if has_auth {
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