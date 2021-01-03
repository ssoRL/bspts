use data::reward::*;
use yew::services::{
    dialog::DialogService,
    console::{ConsoleService},
};
use yew::format::{Json};
use crate::apis::{new_reward, update_reward, delete_reward, FetchResponse};
use yew::services::fetch::{FetchTask};
use yew::prelude::*;
use crate::components::EditResult;
use data::icon::{RewardIcon, RewardCategory};
use crate::components::IconChooser;

pub struct RewardEditor {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    /// The current fetch action going on if any
    fetch_action: Option<FetchTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    /// A reward to edit, or none to create a new reward
    pub reward_to_edit: Option<Box<Reward>>,
    pub on_done: Callback<EditResult<Reward>>,
}

/// THe mode the reward editor is in: create a new reward or edit and existing
enum Mode {
    Create,
    /// Keeps track of the reward's id
    Edit(i32),
}

struct State {
    mode: Mode,
    reward: NewReward,
}

pub enum Msg {
    UpdateName(String),
    UpdatePoints(String),
    UpdateDescription(String),
    UpdateIcon(RewardIcon),
    SaveReward,
    ReturnReward(Reward),
    DeleteReward,
    RewardDeleted,
    CancelEdit,
    Noop,
}

impl Component for RewardEditor {
    type Message = Msg;
    type Properties = Props;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        let (mode, reward_to_edit) = match properties.reward_to_edit {
            None => {(
                Mode::Create,
                NewReward {
                    name: "".to_string(),
                    description: "".to_string(),
                    bspts: 0,
                    icon: RewardIcon::default(),
                }
            )}
            Some(reward) => {(
                Mode::Edit(reward.id),
                NewReward {
                    name: reward.name,
                    description: reward.description,
                    bspts: reward.bspts,
                    icon: RewardIcon::default(),
                }
            )},
        };
        Self {
            state : State{
                mode,
                reward: reward_to_edit,
            },
            props: Props {
                reward_to_edit: None,
                on_done: properties.on_done,
            },
            link,
            fetch_action: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.state.reward.name = name;
                false
            }
            Msg::UpdatePoints(bspts_as_string) => {
                if let Ok(bspts) = bspts_as_string.parse::<i32>() {
                    self.state.reward.bspts = bspts;
                }
                false
            }
            Msg::UpdateDescription(desc) => {
                self.state.reward.description = desc;
                false
            }
            Msg::UpdateIcon(icon) => {
                self.state.reward.icon = icon;
                false
            }
            Msg::SaveReward => {
                match &self.state.mode {
                    Mode::Create => {
                        let reward_committed_callback = self.link.callback(|response: FetchResponse<Reward>| {
                            if let (_, Json(Ok(reward))) = response.into_parts() {
                                Msg::ReturnReward(reward)
                            } else {
                                // TODO: error
                                ConsoleService::error("Failed to save reward");
                                Msg::CancelEdit
                            }
                        });
                        self.fetch_action = Some(new_reward(&self.state.reward, reward_committed_callback));
                    }
                    Mode::Edit(reward_id) => {
                        let reward_committed_callback = self.link.callback(|response: FetchResponse<Reward>| {
                            if let (_, Json(Ok(reward))) = response.into_parts() {
                                Msg::ReturnReward(reward)
                            } else {
                                // TODO: error
                                ConsoleService::error("Failed to save reward");
                                Msg::CancelEdit
                            }
                        });
                        self.fetch_action = Some(update_reward(*reward_id, self.state.reward.clone(), reward_committed_callback));
                    }
                };
                true
            }
            Msg::ReturnReward(reward) => {
                self.fetch_action = None;
                self.props.on_done.emit(EditResult::Return(Box::new(reward)));
                true
            }
            Msg::DeleteReward => {
                let should_delete = DialogService::confirm(format!(
                    "Are you sure you want to destroy reward {}?",
                    self.state.reward.name
                ).as_str());
                if should_delete {
                    if let Mode::Edit(id) = self.state.mode {
                        let after_reward_deleted = self.link.callback(|response: FetchResponse<()>| {
                            if let (_, Json(Ok(()))) = response.into_parts() {
                                Msg::RewardDeleted
                            } else {
                                // TODO: error, don't just leave w/out explanation
                                ConsoleService::error("Failed to delete reward");
                                Msg::CancelEdit
                            }
                        });
                        let delete_reward = delete_reward(id ,after_reward_deleted);
                        self.fetch_action = Some(delete_reward);
                    }
                    true
                } else {
                    false
                }
            }
            Msg::RewardDeleted => {
                self.props.on_done.emit(EditResult::Destroy);
                self.fetch_action = None;
                true
            }
            Msg::CancelEdit => {
                self.props.on_done.emit(EditResult::Cancel);
                true
            }
            Msg::Noop => {
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some(_) = self.fetch_action {
            // If there's something going on, block the form
            // TODO: Make this not look terrible
            return html! {"Working..."}
        }

        let edit_name = self.link.callback(|input: InputData| {Msg::UpdateName(input.value)});
        let edit_bspts = self.link.callback(|input: InputData| {Msg::UpdatePoints(input.value)});
        let edit_desc = self.link.callback(|input: InputData| {Msg::UpdateDescription(input.value)});
        let on_save = self.link.callback(|_| {Msg::SaveReward});
        let on_cancel = self.link.callback(|_| {Msg::CancelEdit});

        let delete_this_reward = if let Mode::Create = self.state.mode {
            // Don't allow destroying a reward that doesn't exist
            html! { <></> }
        } else {
            let on_destroy = self.link.callback(|_| {Msg::DeleteReward});

            html! { <div class="badge-line">
                <span class="flex-buffer" />
                <a class="delete" onclick={on_destroy}>{"Delete this reward"}</a>
            </div>}
        };

        html! {
            <div class="form">
                <div>
                    <input
                        type="text"
                        class="title-input"
                        oninput={edit_name}
                        maxlength="20"
                        placeholder="Reward Name"
                        value={self.state.reward.name.clone()}
                    />
                </div>
                <div>
                    <span class="text">{"Costs "}</span>
                    <input class="input" type="number" oninput={edit_bspts} value={self.state.reward.bspts} />
                    <span class="text">{" BS Pts."}</span>
                </div>
                <div><IconChooser<RewardIcon, RewardCategory>
                    icon={None}
                    on_change={self.link.callback(|_| {Msg::Noop})}
                /></div>
                <div><textarea
                    rows="10" cols="30"
                    class="description-input"
                    oninput={edit_desc}
                    placeholder="Optionally describe the reward"
                    value={self.state.reward.description.clone()}
                /></div>
                <div class="button-line">
                    <span class="cancel button" onclick={on_cancel}>{"Cancel"}</span>
                    <span class="flex-buffer"></span>
                    <span class="save button" onclick={on_save}>{"Save"}</span>
                </div>
                {delete_this_reward}
            </div>
        }
    }
}