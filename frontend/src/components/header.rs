use yew::prelude::*;
use yew::services::{
    console::ConsoleService,
};
use crate::data::*;
use data::user::User;
use crate::app::Route;
use yew_router::components::{RouterAnchor};

type Callbacks = Option<StoreListener<Option<User>>>;

struct State {
    bspts: i32,
    callbacks: Callbacks,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub store: Store,
    pub route: Route,
}

pub struct Header {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SetPoints(i32),
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Creating home");

        ConsoleService::info("Creating self");
        Self {
            state: State {
                bspts: 0,
                callbacks: None,
            },
            props,
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::SetPoints(bspts) => {
                if self.state.bspts == bspts {
                    false
                } else {
                    self.state.bspts = bspts;
                    true
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info("Creating callbacks");
            let user_callback = self.props.store.session_user.subscribe(
                self.link.callback(|is_user: ItemPtr<Option<User>>| {
                    ConsoleService::log("Setting user on home");
                    let pts = match &*is_user.borrow() {
                        Some(user) => user.bspts,
                        None => 0,
                    };
                    Msg::SetPoints(pts)
                }),
                true
            );

            self.state.callbacks = Some(user_callback);
        }
    }

    fn view(&self) -> Html {
        let (tasks_class, rewards_class) = match self.props.route {
            Route::Tasks => ("selected", ""),
            _ => ("", ""),
        };
        html! {
            <>
                <div class="header">
                    <div class="line">
                        <RouterAnchor<Route> classes="home-link" route={Route::HomePage} >
                            <span class="fa fa-3x fa-home" />
                        </RouterAnchor<Route>>
                        <span class="flex-buffer" />
                        <span class="bspts">{format!("BSPTS: {}", self.state.bspts)}</span>
                        <span class="flex-buffer" />
                    </div>
                    <div class="line routes">
                        <RouterAnchor<Route> classes={tasks_class} route={Route::Tasks} >{"Tasks"}</RouterAnchor<Route>>
                        // <span class="flex-buffer" />
                        <a classes={rewards_class} href="#">{"Rewards"}</a>
                    </div>
                </div>
            </>
        }
    }
}