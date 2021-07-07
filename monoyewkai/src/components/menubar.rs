use yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::virtual_dom::{VChild, VComp};

use crate::styles::colors::Color;

mod drop_down_menu;

#[derive(Properties, Clone, PartialEq)]
pub struct BarProps {
    #[prop_or(Color::Bg)]
    bg_color : Color,
    #[prop_or(Color::Fg)]
    fg_color : Color,
    #[prop_or_default]
    height : u32,
}

impl BarProps {
    
    fn style() -> {

    }
}

pub struct MenuBar {
    props : BarProps,
    link : ComponentLink<Self>,
}


impl Component for MenuBar {
    type Message = ();

    type Properties = BarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html!{
            <nav>
            </nav>
        }
    }
}


