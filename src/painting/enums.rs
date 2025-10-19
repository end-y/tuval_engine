use crate::css::enums::Color;
use crate::layout::structs::Rect;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    SolidColor(Color, Rect),
    Text(String, Rect, Color, f32),
}
