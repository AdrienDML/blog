
use yew::prelude::*;
use css_in_rust::Style;

use crate::styles::colors::Color;

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or()]
    pub on_click : Callback<yew::MouseEvent>,
    #[prop_or(Color::Fg)]
    pub text_color : Color,
    #[prop_or(Color::Bg)]
    pub bg_color : Color,
    #[prop_or(String::from("Placeholder text"))]
    pub text : String,
}

pub struct Button {
    props : ButtonProps,
    style : Style,
}

impl Button {
    pub fn get_style(props : ButtonProps) -> Result<Style, String> {
        Style::create(
            "mono-Button", 
            format!(
                r#"
                text-align : center;
                color : {text_color};
                backgound-color : {bg_color};
                "#,
                text_color = props.text_color,
                bg_color = props.bg_color,
            )
        )
    }

    pub fn default_style() -> Result<Style, String> {
        Style::create(
            "mono-Button",
            r#"
            text-align : center;
            color : #000000;
            backgound-color : #ffffff;
            "#
        )
    }
}


impl Component for Button {
    type Message = ();

    type Properties = ButtonProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = Self::get_style(props)
            .or(Self::default_style())
            .unwrap();
        Self {
            props,
            style,
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
            <button class=self.style.clone()>{self.props.text}</button>
        }
    }
}