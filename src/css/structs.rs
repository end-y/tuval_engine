//! CSS data structures module
//!
//! This module defines the core data structures for representing CSS stylesheets.

use crate::css::enums::{Value, Selector};

/// Represents a complete CSS stylesheet
#[derive(Debug, Clone)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

/// Represents a CSS rule with selectors and declarations
#[derive(Debug, Clone)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

/// Represents a CSS property declaration
#[derive(Debug, Clone)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
}
