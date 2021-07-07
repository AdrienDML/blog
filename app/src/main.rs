extern crate yew;
use yew::prelude::*;

// #[derive(Routetable, PartialEq, Clone, Debug)]
// pub enum Route {
//     // #[at("/articles")]
//     // Articles,
//     #[at("/")]
//     Home,
//     #[at("/404")]
//     NotFound,
// }

// impl Route {
//     pub fn switch(&self) -> Html {
//         use Route::*;
//         match self {
//             Home => html!{},
//             NotFound => html!{},
//         }
//     }
// }



use monoyewkai::{
    components::containers::page::{Page, PageProps},
    styles::colors::Color,
};


struct Model {
    _link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            _link : link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let pageprops = yew::props!( PageProps {
            bg_color : Color::Bg,
        });
        html! {
        }
    }
}


fn main() {
    yew::start_app::<Model>();
}