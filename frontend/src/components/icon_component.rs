use yew::prelude::*;
use crate::fontable::Fontable;

pub struct IconComponent<BI> where
    BI: Clone + Fontable,
{
    props: Props<BI>,
}

#[derive(Properties, Clone)]
pub struct Props<BI> where
    BI: Clone + Fontable,
{
    pub icon: BI,
    pub classes: String,
    // #[prop_or_default]
    // _marker: std::marker::PhantomData<CAT>,
}

impl<BI> Component for IconComponent<BI> where
    BI: Clone + Fontable + 'static,
{
    type Message = ();
    type Properties = Props<BI>;

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