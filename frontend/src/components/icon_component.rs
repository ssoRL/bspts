use yew::prelude::*;
use crate::fontable::Fontable;
use data::icon::{BadgeIcon,RewardIcon};

pub struct IconComponent<BI, CAT> where
    CAT: Clone,
    BI: Clone + Fontable<CAT>,
{
    props: Props<BI, CAT>,
}

#[derive(Properties, Clone)]
pub struct Props<BI, CAT> where
    CAT: Clone,
    BI: Clone + Fontable<CAT>,
{
    pub icon: BI,
    pub classes: String,
    #[prop_or_default]
    _marker: std::marker::PhantomData<CAT>,
}

impl<BI, CAT> Component for IconComponent<BI, CAT> where
    CAT: Clone + 'static,
    BI: Clone + Fontable<CAT> + 'static,
{
    type Message = ();
    type Properties = Props<BI, CAT>;

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