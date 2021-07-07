#[derive(Clone, PartialEq)]
pub enum Color {
    Fg,
    Bg,
    Red,
    Blue,
    Green,
    Majenta,
    Yelllow,
    Cyan,
    Orange,
    White,
    Black,
}

impl Color {

    pub fn as_str(&self) -> &str {
        match self {
            Color::Fg => {"#000000"}
            Color::Bg => {"#000000"}
            Color::Red => {"#000000"}
            Color::Blue => {"#000000"}
            Color::Green => {"#000000"}
            Color::Majenta => {"#000000"}
            Color::Yelllow => {"#000000"}
            Color::Cyan => {"#000000"}
            Color::Orange => {"#000000"}
            Color::White => {"#000000"}
            Color::Black => {"#000000"}
        }
    }
}

pub trait Style {
    fn get_style() -> String;
}


