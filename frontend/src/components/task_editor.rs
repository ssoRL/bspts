use data::task::*;
use yew::services::{
    dialog::DialogService,
    console::{ConsoleService},
};
use yew::format::{Json};
use crate::apis::{commit_new_task, update_task, delete_task, FetchResponse};
use yew::services::fetch::{FetchTask};
use yew::prelude::*;
use crate::components::{EditResult, IconChooser};
use data::icon::{TaskIcon, TaskCategory};

pub struct TaskEditor {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    /// The current fetch action going on if any
    fetch_action: Option<FetchTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    /// A task to edit, or none to create a new task
    pub task_to_edit: Option<Task>,
    pub on_done: Callback<EditResult<Task>>,
}

/// THe mode the task editor is in: create a new task or edit and existing
pub enum Mode {
    Create,
    /// Keeps track of the task's id
    Edit(i32),
}

pub struct State {
    pub mode: Mode,
    task: NewTask,
}

pub enum Msg {
    UpdateName(String),
    UpdatePoints(String),
    UpdateDescription(String),
    UpdateFrequencyUnit(String),
    UpdateFrequencyEvery(u32),
    UpdateFrequencyBy(u32),
    UpdateIcon(TaskIcon),
    SaveTask,
    ReturnTask(Task),
    DeleteTask,
    TaskDeleted,
    CancelEdit,
    Noop,
}

impl Component for TaskEditor {
    type Message = Msg;
    type Properties = Props;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        let (mode, task_to_edit) = match properties.task_to_edit {
            None => {(
                Mode::Create,
                NewTask {
                    name: "".to_string(),
                    description: "".to_string(),
                    bspts: 0,
                    frequency: TaskInterval::Days{every: 1},
                    icon: TaskIcon::default(),
                }
            )}
            Some(task) => {(
                Mode::Edit(task.id),
                task.into(),
            )},
        };
        Self {
            state : State{
                mode,
                task: task_to_edit,
            },
            props: Props {
                task_to_edit: None,
                on_done: properties.on_done,
            },
            link,
            fetch_action: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.state.task.name = name;
                false
            }
            Msg::UpdatePoints(bspts_as_string) => {
                if let Ok(bspts) = bspts_as_string.parse::<i32>() {
                    self.state.task.bspts = bspts;
                }
                false
            }
            Msg::UpdateDescription(desc) => {
                self.state.task.description = desc;
                false
            }
            // Change between days, weeks, months
            Msg::UpdateFrequencyUnit(time_unit) => {
                let current_freq = &self.state.task.frequency;
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
                self.state.task.frequency = new_freq;
                true
            }
            // Change between days, weeks, months
            Msg::UpdateFrequencyEvery(new_every) => {
                let current_freq = &self.state.task.frequency;
                let new_freq = match current_freq {
                    TaskInterval::Days{every:_} => TaskInterval::Days{every: new_every},
                    TaskInterval::Weeks{every:_, weekday} => TaskInterval::Weeks{
                        every: new_every,
                        weekday: *weekday,
                    },
                    TaskInterval::Months{every:_, day_of_month} => TaskInterval::Months{
                        every: new_every,
                        day_of_month: *day_of_month
                    },
                };
                self.state.task.frequency = new_freq;
                false
            }
            // Change between days, weeks, months
            Msg::UpdateFrequencyBy(new_by) => {
                let current_freq = &self.state.task.frequency;
                let new_freq = match current_freq {
                    TaskInterval::Days{every} => TaskInterval::Days{every: *every},
                    TaskInterval::Weeks{every, weekday:_} => TaskInterval::Weeks{
                        every: *every,
                        weekday: new_by
                    },
                    TaskInterval::Months{every, day_of_month:_} => TaskInterval::Months{
                        every: *every,
                        day_of_month: new_by
                    },
                };
                self.state.task.frequency = new_freq;
                false
            }
            Msg::UpdateIcon(icon) => {
                ConsoleService::log(&format!("icon: {:#?}", icon));
                self.state.task.icon = icon;
                ConsoleService::log(&format!("self.icon: {:#?}", &self.state.task.icon));
                false
            }
            Msg::SaveTask => {
                match &self.state.mode {
                    Mode::Create => {
                        let task_committed_callback = self.link.callback(|response: FetchResponse<Task>| {
                            if let (_, Json(Ok(task))) = response.into_parts() {
                                Msg::ReturnTask(task)
                            } else {
                                // TODO: error
                                ConsoleService::error("Failed to save task");
                                Msg::CancelEdit
                            }
                        });
                        self.fetch_action = Some(commit_new_task(self.state.task.clone(), task_committed_callback));
                    }
                    Mode::Edit(task_id) => {
                        let task_committed_callback = self.link.callback(|response: FetchResponse<Task>| {
                            if let (_, Json(Ok(task))) = response.into_parts() {
                                Msg::ReturnTask(task)
                            } else {
                                // TODO: error
                                ConsoleService::error("Failed to save task");
                                Msg::CancelEdit
                            }
                        });
                        ConsoleService::log(&format!("save icon: {:#?}", &self.state.task.icon));
                        self.fetch_action = Some(update_task(*task_id, self.state.task.clone(), task_committed_callback));
                    }
                };
                true
            }
            Msg::ReturnTask(task) => {
                self.props.on_done.emit(EditResult::<Task>::Return(Box::new(task)));
                true
            }
            Msg::DeleteTask => {
                let should_delete = DialogService::confirm(format!(
                    "Are you sure you want to destroy task {}?",
                    self.state.task.name
                ).as_str());
                if should_delete {
                    if let Mode::Edit(id) = self.state.mode {
                        let after_task_deleted = self.link.callback(|response: FetchResponse<()>| {
                            if let (_, Json(Ok(()))) = response.into_parts() {
                                Msg::TaskDeleted
                            } else {
                                // TODO: error, don't just leave w/out explanation
                                ConsoleService::error("Failed to delete task");
                                Msg::CancelEdit
                            }
                        });
                        let delete_task = delete_task(id ,after_task_deleted);
                        self.fetch_action = Some(delete_task);
                    }
                    true
                } else {
                    false
                }
            }
            Msg::TaskDeleted => {
                self.props.on_done.emit(EditResult::<Task>::Destroy);
                self.fetch_action = None;
                true
            }
            Msg::CancelEdit => {
                self.props.on_done.emit(EditResult::<Task>::Cancel);
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
        let edit_every = self.link.callback(|input: InputData| {
            match input.value.parse::<u32>() {
                Ok(every) => Msg::UpdateFrequencyEvery(every),
                Err(_) => Msg::Noop
            }
        });
        let on_save = self.link.callback(|_| {Msg::SaveTask});
        let on_cancel = self.link.callback(|_| {Msg::CancelEdit});

        let freq = &self.state.task.frequency;

        let by_when_selector = match freq {
            TaskInterval::Days{every:_} => html!{<></>},
            TaskInterval::Weeks{every:_, weekday} => {
                ConsoleService::log("WEEK!");
                let edit_by = self.link.callback(|input: ChangeData| {
                    match input {
                        ChangeData::Select(select) => {
                            Msg::UpdateFrequencyBy(select.selected_index() as u32)
                        },
                        _ => panic!("can't get change data value")
                    }
                });
                html!{
                    <>
                        <span class="text">{" on "}</span>
                        <select onchange={edit_by}>
                            <option selected={*weekday==0}>{"Monday"}</option>
                            <option selected={*weekday==1}>{"Tuesday"}</option>
                            <option selected={*weekday==2}>{"Wednesday"}</option>
                            <option selected={*weekday==3}>{"Thursday"}</option>
                            <option selected={*weekday==4}>{"Friday"}</option>
                            <option selected={*weekday==5}>{"Saturday"}</option>
                            <option selected={*weekday==6}>{"Sunday"}</option>
                        </select>
                    </>
                }
            },
            TaskInterval::Months{every:_, day_of_month} => {
                ConsoleService::log("MONTH!");
                let edit_by = self.link.callback(|input: InputData| {
                    match input.value.parse::<u32>() {
                        Ok(day_of_month) => {
                            Msg::UpdateFrequencyBy(day_of_month)
                        }
                        Err(_) => Msg::Noop
                    }
                });
                html!{
                    <>
                        <span class="text">{" on the "}</span>
                        <input
                            class="input"
                            type="number"
                            min="1"
                            max="28"
                            oninput={edit_by}
                            value={day_of_month}
                        />
                        <span class="text">{" of the month"}</span>
                    </>
                }
            },
        };


        let edit_time_unit = self.link.callback(|input: ChangeData| {
            match input {
                ChangeData::Select(select) => Msg::UpdateFrequencyUnit(select.value()),
                _ => panic!("can't get change data value")
            }
        });
        let frequency_selector = html! {
            <div>
                <span class="text">{"Do every "}</span>
                <input
                    class="input"
                    type="number"
                    oninput={edit_every}
                    value={self.state.task.frequency.every()}
                />
                <select onchange={edit_time_unit}>
                    <option selected={self.state.task.frequency.in_days()} value="d">{"Days"}</option>
                    <option selected={self.state.task.frequency.in_weeks()} value="w">{"Weeks"}</option>
                    <option selected={self.state.task.frequency.in_months()} value="m">{"Months"}</option>
                </select>
                {by_when_selector}
            </div>
        };

        let delete_this_task = if let Mode::Create = self.state.mode {
            // Don't allow destroying a task that doesn't exist
            html! { <></> }
        } else {
            let on_destroy = self.link.callback(|_| {Msg::DeleteTask});

            html! { <div class="badge-line">
                <span class="flex-buffer" />
                <a class="delete" onclick={on_destroy}>{"Delete this task"}</a>
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
                        placeholder="Task Name"
                        value={self.state.task.name.clone()}
                    />
                </div>
                <div>
                    <span class="text">{"Is worth "}</span>
                    <input class="input" type="number" oninput={edit_bspts} value={self.state.task.bspts} />
                    <span class="text">{" bs points"}</span>
                </div>
                {frequency_selector}
                <div><IconChooser<TaskIcon, TaskCategory>
                    icon={Some(self.state.task.icon.clone())}
                    on_change={self.link.callback(|icon: Box<TaskIcon>| {Msg::UpdateIcon(*icon)})}
                /></div>
                <div><textarea
                    rows="10" cols="30"
                    class="description-input"
                    oninput={edit_desc}
                    placeholder="Optionally describe the task"
                    value={self.state.task.description.clone()}
                /></div>
                <div class="button-line">
                    <span class="cancel button" onclick={on_cancel}>{"Cancel"}</span>
                    <span class="flex-buffer"></span>
                    <span class="save button" onclick={on_save}>{"Save"}</span>
                </div>
                {delete_this_task}
            </div>
        }
    }
}