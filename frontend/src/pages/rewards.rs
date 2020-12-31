use yew::prelude::*;
use data::reward::Reward;
use crate::apis::{get_rewards, sign_out_frontend, FetchResponse};
use crate::components::*;
use yew::format::{Json};
use yew::services::{
    fetch::FetchTask,
    console::ConsoleService,
};
use http::status::StatusCode;
use crate::data::*;
use std::collections::VecDeque;

type StoreCallbacks = Option<StoreListener<VecDeque<Reward>>>;

struct State {
    /// The rewards currently available
    rewards: ItemPtr<VecDeque<Reward>>,
    edit_popup: bool,
    error_message: Option<String>,
    store_callbacks: StoreCallbacks,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub store: Store,
}

pub struct RewardsPage {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    fetch_rewards: Option<FetchTask>,
}

pub enum Msg {
    FetchRewards,
    ReceiveRewards(ItemPtr<VecDeque<Reward>>),
    OpenRewardCreationComponent,
    NewRewardAdded(Box<Reward>),
    CancelCreateReward,
    HandleError{msg: String, code: Option<StatusCode>},
    Noop,
}

impl Component for RewardsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Creating rewards");

        // Get the ball rolling on getting the rewards
        link.send_message(Msg::FetchRewards);

        Self {
            state: State {
                rewards: StoreItem::new_ptr(),
                edit_popup: false,
                error_message: None,
                store_callbacks: None,
            },
            props,
            link,
            fetch_rewards: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            // Fetch the rewards that the user has saved
            Msg::FetchRewards => {
                ConsoleService::info("Getting rewards");
                let store_clone = self.props.store.clone();
                let callback = self.link.callback(move |response: FetchResponse<Vec<Reward>>| {
                    match response.into_parts() {
                        (_, Json(Ok(rewards))) => {
                            store_clone.act(StoreAction::SetRewards(rewards.into()));
                            Msg::Noop
                        }
                        (parts, _) => {
                            Msg::HandleError{
                                msg: "Failed to get rewards".to_string(),
                                code: Some(parts.status),
                            }
                        }
                    }
                });
                self.fetch_rewards = Some(get_rewards(callback));
                false
            }
            // The message to handle the fetch of rewards coming back
            Msg::ReceiveRewards(rewards)=> {
                self.state.rewards = rewards;
                true
            }
            Msg::OpenRewardCreationComponent => {
                self.state.edit_popup = true;
                true
            }
            Msg::NewRewardAdded(reward) => {
                // The reward has been added on the backend, add it to the UI now
                self.props.store.rewards.update(move |rewards| {
                    rewards.push_front(*reward.clone());
                    Some(())
                });
                true
            }
            Msg::CancelCreateReward => {
                self.state.edit_popup = false;
                true
            }
            Msg::HandleError{msg, code} => {
                if let Some(StatusCode::UNAUTHORIZED) = code {
                    sign_out_frontend();
                } else {
                    self.state.error_message = Some(msg);
                }
                true
            }
            Msg::Noop => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info("Creating rewards callbacks");
            let rewards_callback = self.props.store.rewards.subscribe(
                self.link.callback(|rewards| {
                    Msg::ReceiveRewards(rewards)
                }),
                false
            );

            self.state.store_callbacks = Some(rewards_callback);
        }
    }

    fn view(&self) -> Html {
        if let Some(msg) = &self.state.error_message {
            return html! {
                <span>{msg}</span>
            }
        }

        let rewards_html: Html = self.state.rewards.borrow().iter().map(|reward: &Reward| {
                html!{
                <RewardItem
                    // TODO: Could probably squeeze out a tiny bit of perf by holding all of
                    // the rewards in boxes that can get passed around from the get go
                    reward={Box::new(reward.clone())}
                    store={self.props.store.clone()}
                />
            }
        }).collect();

        let new_reward_html = if self.state.edit_popup {
            let on_done = self.link.callback(|result: EditResult<Reward>| {
                match result {
                    EditResult::Return(reward) => Msg::NewRewardAdded(reward),
                    _ => Msg::CancelCreateReward
                }
                
            });

            html! {
                <Popup>
                    <RewardEditor reward_to_edit={None} on_done={on_done} />
                </Popup>
            }
        } else {
            html! {
                <div 
                    class="add-new-reward button"
                    onclick={self.link.callback(|_| {Msg::OpenRewardCreationComponent})}
                >
                    {"Add New Reward"}
                </div>
            }
        };

        html! {
            <>
                <div>{new_reward_html}</div>
                <div class="badge-field">{rewards_html}</div>
            </>
        }
    }
}