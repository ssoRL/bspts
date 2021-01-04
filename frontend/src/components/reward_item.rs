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
    Noop,
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
                                        user.bspts = pts;
                                        Some(())
                                    }
                                    None => None
                                }
                            });
                        }
                        _ => ConsoleService::error("Could not mark task as done")
                    }
                    Msg::Noop
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
            Msg::Noop => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if let Some(_) = self.fetch_action {
            // Return loading indicator
            return html!{
                <div
                    class={"badge loading"}
                />
            }
        }

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
            <div
                class={badge_class}
                title={&reward.description}
            >
                <div class="name">{&reward.name}</div>
                <div class="info">{pts_desc}</div>
                <IconComponent<RewardIcon, RewardCategory> icon={self.props.reward.icon.clone()} classes="on-reward-badge" />
                <div class="badge-line">
                    <span class={edit_class} onclick={click_edit}>{"Edit"}</span>
                    <span class="flex-buffer"></span>
                    <span class={done_class} onclick={click_done}>{"Take"}</span>
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
                                    reward_to_edit={Some(self.props.reward.clone())}
                                    on_done={on_done}
                                />
                            </Popup>
                        }
                    } else {
                        html! {<></>}
                    }
                }
            </div>
        }
    }
}