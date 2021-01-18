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
use crate::components::Header;

// TODO: get naming consistent here (probably w/out Page is better)
/// Definition of the routes for this app
#[derive(Switch, Debug, Clone, Copy)]
pub enum Route {
    #[to = "/#signin"]
    SignInPage,
    #[to = "/#signup"]
    SignUpPage,
    #[to = "/#tasks"]
    Tasks,
    #[to = "/#rewards"]
    RewardsPage,
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
                ConsoleService::log("request auth");
                let store_clone = self.state.store.clone();
                let callback = self.link.callback(move |response: FetchResponse<Option<User>>| {
                    let ret = match response.into_parts() {
                        (_, Json(Ok(Some(user)))) => {
                            ConsoleService::log("got user back to app");
                            store_clone.act(StoreAction::StartSession(user));
                            Msg::DoNotRender
                        }
                        _ => {
                            Msg::DoNotRender
                        }
                    };
                    ret
                });
                self.fetch_task = Some(get_user(callback));
                false
            }
            Msg::ReceiveAuth(user) => {
                ConsoleService::log("receive auth");
                self.state.user = user;
                true
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
                let main_page = match route {
                    Route::Tasks => html!{<TasksPage store={store.clone()} />},
                    Route::RewardsPage => html!{<RewardsPage store={store.clone()} />},
                    _ => html!{<Home />}
                };
                html! {<>
                    <Header store={store.clone()} route={route} />
                    {main_page}
                </>}
            } else {
                match route {
                    // If not authenticated, can only visit the login or sign up page
                    Route::SignInPage => html! {<SignIn store={store.clone()}/>},
                    Route::SignUpPage => html! {<SignUp store={store.clone()}/>},
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