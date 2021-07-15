use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub enum Color {
    Fg,
    FgAlt,
    Bg,
    BgAlt,
    BgLight,
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


impl Display for  Color {
    

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Color::*;
        match self {
            Fg => write!(f, "#f8f8f2"),
            FgAlt => write!(f, "#cfcfc2"),
            Bg => write!(f, "#000000"),
            BgAlt => write!(f, "#3e3d32"),
            BgLight => write!(f, "#75715e"),
            Red => write!(f, "#f92672"),
            Blue => write!(f, "#66d9ef"),
            Green => write!(f, "#a6e22e"),
            Majenta => write!(f, "#fd5ff0"),
            Yelllow => write!(f, "#e6db74"),
            Cyan => write!(f, "#a1efe4"),
            Orange => write!(f, "#fd971f"),
            White => write!(f, "#000000"),
            Black => write!(f, "#ffffff"),
        }
    }
}



