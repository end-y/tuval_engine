//! Style tree module
//!
//! This module handles the creation of styled nodes by applying CSS rules to HTML nodes.

use std::collections::HashMap;
use crate::css::enums::Value;
use crate::html::structs::{Node, ElementData};
use crate::html::enums::NodeType;
use lazy_static::lazy_static;
use crate::css::enums::Display;

lazy_static! {
    static ref DEFAULT_DISPLAY_STYLES: HashMap<String, Display> = {
        let mut m = HashMap::new();
        m.insert("div".to_string(), Display::Block);
        m.insert("p".to_string(), Display::Block);
        m.insert("h1".to_string(), Display::Block);
        m.insert("span".to_string(), Display::Inline);
        // Diğer varsayılan display değerlerini buraya ekleyebilirsiniz
        m
    };
}

/// Represents a node with computed styles applied
#[derive(Debug, Clone)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub computed_values: PropertyMap, // Yeni eklendi
    pub children: Vec<StyledNode<'a>>,
}

/// Type alias for a map of CSS property names to values
pub type PropertyMap = HashMap<String, Value>;

impl<'a> StyledNode<'a> {
    pub fn new(node: &'a Node, specified_values: PropertyMap, computed_values: PropertyMap, children: Vec<StyledNode<'a>>) -> StyledNode<'a> {
        StyledNode {
            node,
            specified_values,
            computed_values,
            children,
        }
    }
    // Stil ağacındaki bir özelliği Value olarak döndürür
    pub fn get_property(&self, name: &str) -> Option<&Value> {
        // Önce doğrudan belirtilen değerlere bak
        if let Some(value) = self.specified_values.get(name) {
            Some(value)
        } else {
            // Yoksa hesaplanmış (miras alınmış) değerlere bak
            self.computed_values.get(name)
        }
    }
}

// Bir DOM düğümüne uyan tüm CSS kurallarını bulur
pub fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a crate::css::structs::StyleSheet) -> Vec<&'a crate::css::structs::Rule> {
    stylesheet.rules.iter().filter(|rule| {
        rule.selectors.iter().any(|selector| selector.matches(elem))
    }).collect()
}

// Stil ağacını DOM ağacından ve stil sayfasından oluşturan ana fonksiyon
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a crate::css::structs::StyleSheet) -> StyledNode<'a> {
    style_tree_recursive(root, stylesheet, &PropertyMap::new())
}

// Rekürsif yardımcı fonksiyon
fn style_tree_recursive<'a>(
    node: &'a Node,
    stylesheet: &'a crate::css::structs::StyleSheet,
    parent_computed_styles: &PropertyMap, // Ebeveynin hesaplanmış stilleri
) -> StyledNode<'a> {
    let specified_values = match node.node_type {
        NodeType::Element(ref elem) => calculate_style_for_element(elem, stylesheet),
        _ => HashMap::new(),
    };

    let mut computed_values = parent_computed_styles.clone(); // Ebeveyn stillerini miras al

    // Kalıtılabilir özellikleri ebeveyn'den al
    for (prop, value) in parent_computed_styles.iter() {
        if is_inheritable_property(prop) {
            computed_values.insert(prop.clone(), value.clone());
        }
    }

    // Kendi belirtilen değerleri miras alınanları ezer
    for (prop, value) in specified_values.iter() {
        computed_values.insert(prop.clone(), value.clone());
    }

    let children = node.children.iter()
        .map(|child| style_tree_recursive(child, stylesheet, &computed_values)) // Alt düğümlere computed_values'ı geçir
        .collect();

    StyledNode::new(node, specified_values, computed_values, children) // computed_values'ı ekle
}

// Bir özelliğin kalıtılabilir olup olmadığını kontrol eden yardımcı fonksiyon
fn is_inheritable_property(property_name: &str) -> bool {
    match property_name {
        "color" | "font-family" | "font-size" | "font-weight" | "line-height" => true,
        _ => false,
    }
}

// Bir element için stil özelliklerini hesaplar
fn calculate_style_for_element(elem: &ElementData, stylesheet: &crate::css::structs::StyleSheet) -> PropertyMap {
    let mut properties = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Varsayılan display stilini ekle, eğer CSS tarafından ezilmediyse
    if !properties.contains_key("display") {
        if let Some(default_display) = DEFAULT_DISPLAY_STYLES.get(&elem.tag_name.to_string()) {
            properties.insert("display".to_string(), Value::Display(default_display.clone()));
        }
    }

    // Özgüllüğe göre sırala (en özgül sona gelsin)
    rules.sort_by_key(|rule| rule.selectors[0].specificity());

    for rule in &rules {
        for declaration in &rule.declarations {
            properties.insert(declaration.property.clone(), declaration.value.clone());
        }
    }
    properties
}
