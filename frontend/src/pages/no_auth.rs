use yew::prelude::*;
use crate::app;
use yew_router::components::RouterAnchor;

type Anchor = RouterAnchor<app::Route>;

pub struct AuthOptions { }

impl Component for AuthOptions {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="no-auth-page">
                <span class="title">{"BsPts"}</span>
                <Anchor route=app::Route::SignUpPage classes="button">
                    {"Sign Up"}
                </Anchor>
                <Anchor route=app::Route::SignInPage classes="button">
                    {"Sign In"}
                </Anchor>
            </div>
        }
    }
}