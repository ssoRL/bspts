use types::task::Task;
use yew::prelude::*;

pub struct TaskComponent {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub task: Task,
    pub on_tick: Callback<()>,
}

impl Component for TaskComponent {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let on_tick = self.props.on_tick.reform(|_| ());
        let task = &self.props.task;

        html! {
            <div>
                <input type="checkbox" onclick={on_tick} id={&task.id} name={&task.name} checked={task.is_done} />
                <span>{&task.name}</span>
            </div>
        }
    }
}