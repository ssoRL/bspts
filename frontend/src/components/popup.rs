use yew::services::console::{ConsoleService};
use yew::prelude::*;

pub struct Popup {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
}

pub enum Msg {}

impl Component for Popup {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    // This component will basically never update itself
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        // Basically just wrap the child component in hella styling
        html! {
            <div class="pop-up-screen"><div class="popup"><div class="pop-up-content">
                {self.props.children.clone()}
            </div></div></div>
        }
    }
}