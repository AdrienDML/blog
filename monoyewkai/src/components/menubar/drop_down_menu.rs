use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DdmProps {
    #[props_or(false)]
    is_open : bool,
    #[props_or()]
    bg_color : Color,
    #[props_or()]
    fg_color : Color,
    #[props_or()]
    hsize : u32,
    #[props_or()]
    wsize : u32,
}

pub enum DdmMsg {
    Toggle,
    Hover,
}


pub struct DropDownMenu {
    props : DdmProps,
}

impl Component for DropDownMenu {
    type Message;

    type Properties;

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
        todo!()
    }
}

