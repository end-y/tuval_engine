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
    pub fn pretty_print(&self, indent: usize) {
        let indent_str = "  ".repeat(indent);
        match self.node_type {
            NodeType::Element(ref element_data) => {
                println!("{}<{:?}>", indent_str, element_data.tag_name);
                for (attr_name, attr_value) in &element_data.attributes {
                    println!("{}  {:?}=\"{}\"", indent_str, attr_name, attr_value);
                }
            },
            NodeType::Text(ref text) => {
                println!("{}{}", indent_str, text);
            }
        }
        for child in &self.children {
            child.pretty_print(indent + 1);
        }
    }
}
