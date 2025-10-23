//! Painting command enums
//!
//! This module defines the display commands used for rendering.

use crate::css::enums::Color;
use crate::layout::structs::Rect;

/// Represents a display command to be rendered
#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    SolidColor(Color, Rect),
    Text(String, Rect, Color, f32),
}
