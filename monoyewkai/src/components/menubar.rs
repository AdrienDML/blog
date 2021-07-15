use yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::virtual_dom::{VChild, VComp};

use crate::styles::colors::Color;

use css_in_rust::style::Style;

mod drop_down_menu;

#[derive(Properties, Clone, PartialEq)]
pub struct BarProps {
    #[prop_or(Color::Bg)]
    bg_color : Color,
    #[prop_or(Color::Fg)]
    text_color : Color,
    #[prop_or_default]
    height : u32,
    #[prop_or_default]
    menu_names : Vec<String>,
    #[prop_or_default]
    menulinks : Vec<Strings>,
}

impl BarProps {
    
    pub fn get_style(props : ButtonProps) -> Result<Style, String> {
        Style::create(
            "mono-menubar", 
            format!(
                r#"
                "#,
            )
        )
    }

    pub fn default_style() -> Result<Style, String> {
        Style::create(
            "mono-menubar",
            r#"
            "#
        )
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


