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
