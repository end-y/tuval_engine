use std::collections::HashMap;
use super::enums::{AttrName, NodeType, TagName};

//structs
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ElementData {
    pub tag_name: TagName,
    pub attributes: HashMap<AttrName, String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

impl Node {
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.node_type {
            NodeType::Element(ref element_data) => write!(f, "<{:?}>", element_data.tag_name),
            NodeType::Text(ref text) => write!(f, "{}", text),
        }
    }
}
