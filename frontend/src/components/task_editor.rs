use data::task::*;
use yew::services::console::{ConsoleService};
use yew::format::{Json};
use crate::apis::{get_tasks, commit_new_task, FetchResponse};
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
    SaveTask,
    ReturnTask(Task),
    CancelEdit,
}

impl Component for TaskEditor {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mode = match props.task_to_edit {
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
                on_create: props.on_create,
                on_cancel: props.on_cancel,
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
            },
            Msg::UpdatePoints(bspts_as_string) => {
                let bspts = bspts_as_string.parse::<i32>().unwrap();
                match &mut self.state.mode {
                    Mode::Create(task) => task.bspts = bspts,
                    Mode::Edit(task) => task.bspts = bspts,
                }
                false
            },
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
            },
            Msg::CancelEdit => {
                self.props.on_cancel.emit(());
                true
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let edit_name = self.link.callback(|input: InputData| {Msg::UpdateName(input.value)});
        let edit_bspts = self.link.callback(|input: InputData| {Msg::UpdatePoints(input.value)});
        let on_save = self.link.callback(|_| {Msg::SaveTask});
        let on_cancel = self.link.callback(|_| {Msg::CancelEdit});

        html! {
            <div class="form">
                <div><input type="text" class="title-input" oninput={edit_name} maxlength="20" placeholder="Task Name" /></div>
                <div>
                    <span class="text">{"Is worth "}</span>
                    <input class="input" type="number" oninput={edit_bspts} />
                    <span class="text">{" bs points"}</span>
                </div>
                <div class="button-line">
                    <span class="cancel button" onclick={on_cancel}>{"Cancel"}</span>
                    <span class="flex-buffer"></span>
                    <span class="save button" onclick={on_save}>{"Save"}</span>
                </div>
            </div>
        }
    }
}