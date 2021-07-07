
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    pub on_click : Callback<yew::MouseEvent>,
}

pub struct Button {
    props : ButtonProps,
}

impl Component for Button {
    type Message = ();

    type Properties = ButtonProps;

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
        html!{
            <button></button>
        }
    }
}