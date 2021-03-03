use data::reward::Reward;
use data::icon::*;
use yew::prelude::*;
use yew::format::{Json};
use crate::components::{Popup, RewardEditor, EditResult, IconComponent};
use yew::services::{
    fetch::FetchTask,
    console::ConsoleService,
};
use crate::data::*;
use crate::apis::{do_reward, FetchResponse};

pub struct RewardItem {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    fetch_action: Option<FetchTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub reward: Box<Reward>,
    pub store: Store,
}

pub struct State {
    /// Show the pop up used to edit this reward
    edit_popup: bool,
}

pub enum Msg {
    TakeReward,
    EditReward,
    Update(Box<Reward>),
    CancelEdit,
    DestroySelf,
    FetchDone,
}

impl Component for RewardItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            edit_popup: false,
        };
        Self { state, props, link, fetch_action: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TakeReward => {
                let store = self.props.store.clone();
                let callback = self.link.callback(move |response: FetchResponse<i32>| {
                    match response.into_parts() {
                        (_, Json(Ok(pts))) => {
                            store.session_user.update(|user_opt| {
                                match user_opt {
                                    Some(user) => {
                                        if user.bspts != pts {
                                            user.bspts = pts;
                                            return true;
                                        }
                                        return false
                                    }
                                    None => false
                                }
                            });
                        }
                        _ => ConsoleService::error("Could not mark task as done")
                    }
                    Msg::FetchDone
                });
                self.fetch_action = Some(do_reward(self.props.reward.id, callback));
                false
            }
            Msg::EditReward => {
                self.state.edit_popup = true;
                true
            }
            Msg::Update(reward) => {
                self.state.edit_popup = false;
                self.props.reward = reward;
                true
            }
            Msg::CancelEdit => {
                self.state.edit_popup = false;
                true
            }
            Msg::DestroySelf => {
                self.state.edit_popup = false;
                self.props.store.act(StoreAction::DeleteReward(self.props.reward.id));
                true
            }
            Msg::FetchDone => {
                self.fetch_action = None;
                true
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let reward = &self.props.reward;

        let pts_desc = match reward.bspts {
           1 => "1 pt".to_string(),
           pts => format!("{} pts", pts),
        };

        let click_edit = self.link.callback(|_| {Msg::EditReward});
        let click_done = self.link.callback(|_| {Msg::TakeReward});

        let badge_class = format!("badge {}-theme", self.props.reward.icon.get_color().to_string());
        let edit_class = format!("edit button {}-theme", self.props.reward.icon.get_color().to_string());
        let done_class = format!("done button {}-theme-inv", self.props.reward.icon.get_color().to_string());

        html! {
            <div class="covered-on-load">
            <div
                class={badge_class}
                title={&reward.description}
            >
                <IconComponent<RewardIcon> icon={self.props.reward.icon.clone()} classes="on-reward-badge" />
                <div class="description">
                    <div class="name">{&reward.name}</div>
                    <div class="info">{pts_desc}</div>
                </div>
                <div class="buttons">
                    <div class={edit_class} onclick={click_edit}>{"Edit"}</div>
                    <div class={done_class} onclick={click_done}>{"Take"}</div>
                </div>
                {
                    if self.state.edit_popup {
                        let on_done = self.link.callback(|result: EditResult<Reward>| {
                            match result {
                                EditResult::Return(reward) => Msg::Update(reward),
                                EditResult::Cancel => Msg::CancelEdit,
                                EditResult::Destroy => Msg::DestroySelf,
                            }
                        });
                        
                        html! {
                            <Popup>
                                <RewardEditor
                                    reward_to_edit={Some(*self.props.reward.clone())}
                                    on_done={on_done}
                                />
                            </Popup>
                        }
                    } else {
                        html! {<></>}
                    }
                }
            </div>
            {if let Some(_) = self.fetch_action {
                html!{
                    <div class={"loading-screen"}>
                        <div class="loading-spinner fa fa-spinner" />
                    </div>
                }
            } else {
                html!{<></>}
            }}
            </div>
        }
    }
}