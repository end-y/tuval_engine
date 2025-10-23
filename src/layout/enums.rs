//! Layout box type enums
//!
//! This module defines the types of layout boxes used in the rendering engine.

/// Represents the type of a layout box
#[derive(Debug)]
pub enum LayoutBoxType {
    BlockNode,
    #[allow(dead_code)]
    InlineNode,
    AnonymousBlock,
    LineBox,
}
