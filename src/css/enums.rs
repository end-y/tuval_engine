use super::structs::SelectorType;

#[derive(Debug)]
pub enum Value {
    Length(f32, Unit),
    Color(Color),
    Keyword(String),
}
#[derive(Debug)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
}
#[derive(Debug)]
pub enum Color {
    RGBA(u8, u8, u8, f32),
    HSLA(f32, f32, f32, f32),
}

pub enum Selector {
    Type(SelectorType),
}

