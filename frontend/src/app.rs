use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::*;
use crate::apis;

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
        let render = Router::render(|route: Route|
            if apis::is_signed_in() {
                // If authorized, always go home for now
                html! {<Home />}
            } else {
                match route {
                    // If not authenticated, can only visit the login or sign up page
                    Route::SignInPage => html! {<SignIn/>},
                    Route::SignUpPage => html! {<SignUp/>},
                    // Otherwise directed to auth
                    _ => html! {<AuthOptions/>},
                }
            }
        );

        html! {
            <Router<Route, ()> render=render/>
        }
    }
}