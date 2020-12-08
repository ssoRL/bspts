use data::task::*;
use yew::services::console::{ConsoleService};
use yew::format::{Json};
use crate::apis::{commit_new_task, FetchResponse};
use yew::services::fetch::{FetchTask};
use yew::prelude::*;

pub struct TaskEditor {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    save_task_fetch: Option<FetchTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub task_to_edit: Option<Task>,
    pub on_create: Callback<Task>,
    pub on_cancel: Callback<()>,
}

pub enum Mode {
    Create(NewTask),
    Edit(Task),
}

pub struct State {
    pub mode: Mode,
    pub saving: bool,
}

pub enum Msg {
    UpdateName(String),
    UpdatePoints(String),
    UpdateDescription(String),
    UpdateFrequencyUnit(String),
    UpdateFrequencyEvery(String),
    UpdateFrequencyBy(String),
    SaveTask,
    ReturnTask(Task),
    CancelEdit,
}

impl Component for TaskEditor {
    type Message = Msg;
    type Properties = Props;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mode = match properties.task_to_edit {
            None => {
                Mode::Create(NewTask {
                    name: "".to_string(),
                    description: "".to_string(),
                    bspts: 0,
                    frequency: TaskInterval::Days{every: 1}
                })
            },
            Some(task) => {
                Mode::Edit(task)
            },
        };
        Self {
            state : State{
                mode,
                saving: false,
            },
            props: Props {
                task_to_edit: None,
                on_create: properties.on_create,
                on_cancel: properties.on_cancel,
            },
            link,
            save_task_fetch: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                match &mut self.state.mode {
                    Mode::Create(task) => task.name = name,
                    Mode::Edit(task) => task.name = name,
                }
                false
            }
            Msg::UpdatePoints(bspts_as_string) => {
                let bspts = bspts_as_string.parse::<i32>().unwrap();
                match &mut self.state.mode {
                    Mode::Create(task) => task.bspts = bspts,
                    Mode::Edit(task) => task.bspts = bspts,
                }
                false
            }
            Msg::UpdateDescription(desc) => {
                match &mut self.state.mode {
                    Mode::Create(task) => task.description = desc,
                    Mode::Edit(task) => task.description = desc,
                }
                false
            }
            // Change between days, weeks, months
            Msg::UpdateFrequencyUnit(time_unit) => {
                let current_freq = match &mut self.state.mode {
                    Mode::Create(task) => &task.frequency,
                    Mode::Edit(task) => &task.frequency,
                };
                let (every, by) = match current_freq {
                    TaskInterval::Days{every} => (every, &(0 as u32)),
                    TaskInterval::Weeks{every, weekday} => (every, weekday),
                    TaskInterval::Months{every, day_of_month} => (every, day_of_month),
                };
                let new_freq = match time_unit.as_str() {
                    "d" => TaskInterval::Days{every: *every},
                    "w" => TaskInterval::Weeks{every: *every, weekday: *by},
                    "m" => TaskInterval::Months{every: *every, day_of_month: *by},
                    _ => panic!("Invalid time unit for task frequency"),
                };
                match &mut self.state.mode {
                    Mode::Create(task) => task.frequency = new_freq,
                    Mode::Edit(task) => task.frequency = new_freq,
                }
                true
            }
            // Change between days, weeks, months
            Msg::UpdateFrequencyEvery(every_as_string) => {
                let new_every = every_as_string.parse::<u32>().unwrap();
                let current_freq = match &mut self.state.mode {
                    Mode::Create(task) => &task.frequency,
                    Mode::Edit(task) => &task.frequency,
                };
                let new_freq = match current_freq {
                    TaskInterval::Days{every:_} => TaskInterval::Days{every: new_every},
                    TaskInterval::Weeks{every:_, weekday} => TaskInterval::Weeks{
                        every: new_every,
                        weekday: *weekday
                    },
                    TaskInterval::Months{every:_, day_of_month} => TaskInterval::Months{
                        every: new_every,
                        day_of_month: *day_of_month
                    },
                };
                match &mut self.state.mode {
                    Mode::Create(task) => task.frequency = new_freq,
                    Mode::Edit(task) => task.frequency = new_freq,
                }
                false
            }
            // Change between days, weeks, months
            Msg::UpdateFrequencyBy(by_as_string) => {
                let new_by = by_as_string.parse::<u32>().unwrap();
                let current_freq = match &mut self.state.mode {
                    Mode::Create(task) => &task.frequency,
                    Mode::Edit(task) => &task.frequency,
                };
                let new_freq = match current_freq {
                    TaskInterval::Days{every} => TaskInterval::Days{every: *every},
                    TaskInterval::Weeks{every, weekday} => TaskInterval::Weeks{
                        every: *every,
                        weekday: new_by
                    },
                    TaskInterval::Months{every, day_of_month} => TaskInterval::Months{
                        every: *every,
                        day_of_month: new_by
                    },
                };
                match &mut self.state.mode {
                    Mode::Create(task) => task.frequency = new_freq,
                    Mode::Edit(task) => task.frequency = new_freq,
                }
                false
            }
            Msg::SaveTask => {
                match &self.state.mode {
                    Mode::Create(new_task) => {
                        self.state.saving = true;
                        let task_committed_callback = self.link.callback(|response: FetchResponse<Task>| {
                            if let (_, Json(Ok(task))) = response.into_parts() {
                                Msg::ReturnTask(task)
                            } else {
                                // TODO: error
                                ConsoleService::error("Failed to save task");
                                Msg::CancelEdit
                            }
                        });
                        self.save_task_fetch = Some(commit_new_task(new_task.clone(), task_committed_callback));
                    }
                    Mode::Edit(task) => {
                        // TODO: update on the backend, for now just return as-is
                        self.link.send_message(Msg::ReturnTask(task.clone()));
                    }
                };
                true
            }
            Msg::ReturnTask(task) => {
                self.props.on_create.emit(task);
                true
            }
            Msg::CancelEdit => {
                self.props.on_cancel.emit(());
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let edit_name = self.link.callback(|input: InputData| {Msg::UpdateName(input.value)});
        let edit_bspts = self.link.callback(|input: InputData| {Msg::UpdatePoints(input.value)});
        let edit_desc = self.link.callback(|input: InputData| {Msg::UpdateDescription(input.value)});
        let edit_every = self.link.callback(|input: InputData| {Msg::UpdateFrequencyEvery(input.value)});
        let on_save = self.link.callback(|_| {Msg::SaveTask});
        let on_cancel = self.link.callback(|_| {Msg::CancelEdit});

        let freq = match &self.state.mode {
            Mode::Create(task) => &task.frequency,
            Mode::Edit(task) => &task.frequency,
        };

        let by_when_selector = match freq {
            TaskInterval::Days{every} => html!{<></>},
            TaskInterval::Weeks{every, weekday} => {
                let edit_by = self.link.callback(|input: ChangeData| {
                    ConsoleService::log(format!("onchange data: {:#?}", input).as_str());
                    match input {
                        ChangeData::Value(val) => Msg::UpdateFrequencyBy(val),
                        _ => panic!("can't get change data value")
                    }
                    
                });
                html!{
                    <>
                        <span class="text">{" on "}</span>
                        <select onchange={edit_by}>
                            <option value="0">{"Monday"}</option>
                            <option value="1">{"Tuesday"}</option>
                            <option value="2">{"Wednesday"}</option>
                            <option value="3">{"Thursday"}</option>
                            <option value="4">{"Friday"}</option>
                            <option value="5">{"Saturday"}</option>
                            <option value="6">{"Sunday"}</option>
                        </select>
                    </>
                }
            },
            TaskInterval::Months{every:_, day_of_month:_} => {
                let edit_by = self.link.callback(|input: InputData| {Msg::UpdateFrequencyBy(input.value)});
                html!{
                    <>
                        <span class="text">{" on the "}</span>
                        <input class="input" type="number" min="1" max="5" oninput={edit_by} />
                        <span class="text">{" of the month"}</span>
                    </>
                }
            },
        };


        let edit_time_unit = self.link.callback(|input: ChangeData| {
            ConsoleService::log(format!("onchange data: {:#?}", input).as_str());
            match input {
                ChangeData::Value(val) => Msg::UpdateFrequencyUnit(val),
                _ => panic!("can't get change data value")
            }
        });
        let frequency_selector = html! {
            <div>
                <span class="text">{"Do every "}</span>
                <input class="input" type="number" oninput={edit_every} />
                <select onchange={edit_time_unit}>
                    <option value="d">{"Days"}</option>
                    <option value="w">{"Weeks"}</option>
                    <option value="m">{"Months"}</option>
                </select>
                {by_when_selector}
            </div>
        };

        html! {
            <div class="form">
                <div>
                    <input type="text" class="title-input" oninput={edit_name} maxlength="20" placeholder="Task Name" />
                </div>
                <div>
                    <span class="text">{"Is worth "}</span>
                    <input class="input" type="number" oninput={edit_bspts} />
                    <span class="text">{" bs points"}</span>
                </div>
                {frequency_selector}
                <div><textarea
                    rows="10" cols="30"
                    class="description-input"
                    oninput={edit_desc}
                    placeholder="Optionally describe the task"
                /></div>
                <div class="button-line">
                    <span class="cancel button" onclick={on_cancel}>{"Cancel"}</span>
                    <span class="flex-buffer"></span>
                    <span class="save button" onclick={on_save}>{"Save"}</span>
                </div>
            </div>
        }
    }
}