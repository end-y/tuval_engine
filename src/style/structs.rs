use std::collections::HashMap;
use crate::css::enums::Value;
use crate::html::structs::{Node, ElementData};
use crate::html::enums::NodeType;

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

pub type PropertyMap = HashMap<String, Value>;

impl<'a> StyledNode<'a> {
    // Stil ağacındaki bir özelliği Value olarak döndürür
    pub fn get_property(&self, name: &str) -> Option<&Value> {
        self.specified_values.get(name)
    }

    // Stil ağacını girintili bir şekilde yazdırır
    pub fn pretty_print(&self, indent: usize) {
        let indent_str = "  ".repeat(indent);
        match self.node.node_type {
            NodeType::Element(ref element_data) => {
                println!("{}<{:?}>", indent_str, element_data.tag_name);
                for (prop, value) in &self.specified_values {
                    println!("{}  {}: {:?}", indent_str, prop, value);
                }
            },
            NodeType::Text(ref text) => {
                println!("{}{}", indent_str, text);
                for (prop, value) in &self.specified_values {
                    println!("{}  {}: {:?}", indent_str, prop, value);
                }
            }
        }
        for child in &self.children {
            child.pretty_print(indent + 1);
        }
    }
}

// Bir DOM düğümüne uyan tüm CSS kurallarını bulur
pub fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a crate::css::structs::StyleSheet) -> Vec<&'a crate::css::structs::Rule> {
    let matched_rules: Vec<&crate::css::structs::Rule> = stylesheet.rules.iter().filter(|rule| {
        let is_match = rule.selectors.iter().any(|selector| selector.matches(elem));
        if is_match {
            println!("Matching rule found for element {:?}: {:?}", elem.tag_name, rule);
        }
        is_match
    }).collect();
    println!("Total matching rules for element {:?}: {}", elem.tag_name, matched_rules.len());
    matched_rules
}

// Bir elementi ve CSS kurallarını kullanarak stilize edilmiş bir düğüm oluşturur.
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a crate::css::structs::StyleSheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => calculate_style_for_element(elem, stylesheet),
            NodeType::Text(_) => HashMap::new(),
        },
        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
    }
}

// Bir element için stil özelliklerini hesaplar
fn calculate_style_for_element(elem: &ElementData, stylesheet: &crate::css::structs::StyleSheet) -> PropertyMap {
    let mut properties = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Özgüllüğe göre sırala (en özgül sona gelsin)
    rules.sort_by_key(|rule| rule.selectors[0].specificity());

    println!("Calculating style for element {:?}", elem.tag_name);
    for rule in &rules {
        println!("  Applying rule: {:?}", rule);
        for declaration in &rule.declarations {
            println!("    Inserting property: {}: {:?}", declaration.property, declaration.value);
            properties.insert(declaration.property.clone(), declaration.value.clone());
        }
    }
    println!("Final properties for element {:?}: {:?}", elem.tag_name, properties);
    properties
}
