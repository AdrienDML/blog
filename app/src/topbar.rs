use yew::prelude::*;

use crate::colors;

use super::colors::Color;

#[derive(Properties, Clone, PartialEq)]
pub struct BarProps {
    #[prop_or(Color::Bg)]
    bg_color : Color,
    #[prop_or(Color::Fg)]
    fg_color : Color,
    #[prop_or_default]
    size : u32,
}

pub struct MenuBar {
    props : MenuProps
}


impl Component for MenuBar {
    type Message;

    type Properties = BarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        todo!()
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

pub struct MenuProps {
    
}

pub enum Menu {
    Simple(),
    DropDown(),
}