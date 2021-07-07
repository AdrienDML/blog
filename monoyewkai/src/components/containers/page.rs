use yew::prelude::*;

use crate::styles::colors::Color;

#[derive(Properties, Clone, PartialEq)]
pub struct PageProps {
    #[prop_or(Color::Bg)]
    pub bg_color : Color,
    #[prop_or_default]
    pub children : Children,
}

pub struct Page {
    props : PageProps,
}

impl Component for Page {
    type Message = ();

    type Properties = PageProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <body style={ format!("color:{};", self.props.bg_color.as_str())}>
                { for self.props.children.iter() }
            </body>
        }
    }
}