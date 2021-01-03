use yew::prelude::*;


use data::icon::{RewardCategory, RewardIcon, BadgeIcon};

pub struct RewardIconComponent {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub icon: RewardIcon,
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
        let icon_class = match self.props.icon.get_category() {
            RewardCategory::Coffee => "fa-bone",
            _ => "fa-bong",
        };

        let classes = format!("thumbnail fas {}", icon_class);

        html! { <i class={classes} /> }
    }
}