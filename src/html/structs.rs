//! HTML data structures module
//!
//! This module defines the core data structures for representing HTML documents.

use std::collections::HashMap;
use super::enums::{AttrName, NodeType, TagName};

/// Represents an HTML element with its tag name and attributes
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ElementData {
    pub tag_name: TagName,
    pub attributes: HashMap<AttrName, String>,
}

/// Represents a node in the HTML DOM tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}


impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.node_type {
            NodeType::Element(ref element_data) => write!(f, "<{:?}>", element_data.tag_name),
            NodeType::Text(ref text) => write!(f, "{}", text),
        }
    }
}
