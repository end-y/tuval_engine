use super::enums::{Value, Selector};

pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

pub type Specificity = (usize, usize, usize);

pub struct SelectorType {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Debug)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}
