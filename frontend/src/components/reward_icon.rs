use yew::prelude::*;
use crate::icon::Fontable;
use data::icon::RewardIcon;

pub struct RewardIconComponent {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub icon: RewardIcon,
    pub classes: String,
}

impl Component for RewardIconComponent {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let classes = format!("{} fas {}", self.props.classes, self.props.icon.font_class());

        html! { <i class={classes} /> }
    }
}