use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::*;
use crate::apis;

/// Definition of the routes for this app
#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/#signin"]
    SigninPage,
    #[to = "/#signup"]
    SignupPage,
    #[to = "/"]
    HomePage,
}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|route: Route| match apis::get_jwt() {
            None => {
                match route {
                    // If not authed, can only visit the login or sign up page
                    Route::SigninPage => html! {<SignIn/>},
                    Route::SignupPage => html! {<SignUp/>},
                    // Otherwise directed to auth
                    _ => html! {<AuthOptions/>},
                }
            },
            Some(jwt) => {
                // If authed, always go home for now
                html! {<Home/>}
            }
        });

        html! {
            <Router<Route, ()> render=render/>
        }
    }
}