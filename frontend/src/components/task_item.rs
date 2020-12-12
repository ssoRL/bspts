use data::task::Task;
use yew::prelude::*;
use crate::components::{Popup, TaskEditor};

pub struct TaskItem {
    state: State,
    props: Props,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub task: Task,
    pub on_tick: Callback<()>,
}

pub struct State {
    edit_popup: bool,
}

pub enum Msg {
    EditTask,
    Update(Task),
    CancelEdit,
}

impl Component for TaskItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            edit_popup: false,
        };
        Self { state, props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EditTask => {
                self.state.edit_popup = true;
                true
            }
            Msg::Update(task) => {
                self.state.edit_popup = false;
                self.props.task = task;
                true
            }
            Msg::CancelEdit => {
                self.state.edit_popup = false;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let on_tick = self.props.on_tick.reform(|_| ());
        let task = &self.props.task;

        let is_done_class = if task.is_done {
            "completed"
         }else {
            "uncompleted"
         };

         let pts_desc = match task.bspts {
            1 => "1 pt".to_string(),
            pts => format!("{} pts", pts),
         };

         // TODO: Actually fuck with this
         //let today = Local::today().naive_local();
         //let due_in = today.signed_duration_since(task.next_reset);
        //  let due = task.next_reset.expect("Look, this can't actually be none");
        //  let do_by_desc = match 1 {
        //     days if days < 0 => "Yesterday".to_string(),
        //     0 | 1 => "today".to_string(),
        //     2 => "tomorrow".to_string(),
        //     3 ..= 7 => {
        //         // It's in the next week, therefore just say eg "next Saturday"
        //         format!("next {}", due.format("%A"))
        //     },
        //     _ => due.format("%-d %B, %C%y").to_string()
        //  };
        //  let do_by = format!("Do by {}", do_by_desc);
        let do_by = "Do by today".to_string();

        let click_edit = self.link.callback(|_| {Msg::EditTask});

        html! {
            <div
                class={format!("badge task-item {}", is_done_class)}
                onclick={on_tick}
                title={&task.description}
            >
                <div class="task-name">{&task.name}</div>
                <div class="info-line">
                    <span class="points">{pts_desc}</span>
                    <span class="flex-buffer"></span>
                    <span class="do-by">{do_by}</span>
                </div>
                <i class="thumbnail fas fa-coffee"></i>
                <div class="buttons">
                    <span class="edit button" onclick={click_edit}>{"Edit"}</span>
                    <span class="flex-buffer"></span>
                    <span class="done button">{"Done"}</span>
                </div>
                {
                    if self.state.edit_popup {
                        let save_edits = self.link.callback(|edited_task| {Msg::Update(edited_task)});
                        let cancel_edits = self.link.callback(|_| {Msg::CancelEdit});
                        html! {
                            <Popup>
                                <TaskEditor
                                    on_create={save_edits}
                                    on_cancel={cancel_edits}
                                    task_to_edit={Some(self.props.task.clone())}
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