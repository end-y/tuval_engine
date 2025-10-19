use crate::html::structs::ElementData;
use crate::html::enums::AttrName;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Value {
    Length(f32, Unit),
    Color(Color),
    Keyword(String),
    Display(Display), // Yeni eklendi
}
#[derive(Debug, Clone)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Color {
    RGBA(u8, u8, u8, f32),
    HSLA(f32, f32, f32, f32),
}

#[derive(Debug, Clone, PartialEq)] // PartialEq eklendi
pub enum Display {
    Inline,
    Block,
    None,
    // Diğer display tipleri eklenebilir (inline-block, flex, grid vb.)
}

impl Default for Display {
    fn default() -> Self {
        Display::Block // Varsayılan display tipi block
    }
}

#[derive(Debug, Clone)]
pub enum Selector {
    Type(SelectorType),
}

#[derive(Debug, Clone)]
pub struct SelectorType {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

impl Selector {
    pub fn matches(&self, element: &ElementData) -> bool {
        let Selector::Type(s) = self;
        s.matches(element)
    }

    // Özgüllük değerini hesaplar
    pub fn specificity(&self) -> (usize, usize, usize) {
        let Selector::Type(selector) = self;
        let mut specificity = (0, 0, 0);
        if let Some(_) = &selector.id {
            specificity.0 += 1;
        }
        for _class in &selector.class {
            specificity.1 += 1;
        }
        if let Some(_) = &selector.tag_name {
            specificity.2 += 1;
        }
        specificity
    }
}

impl SelectorType {
    pub fn matches(&self, element: &ElementData) -> bool {
        // Etiket adı eşleşmesi
        if let Some(tag_name) = &self.tag_name {
            if element.tag_name.to_string().to_lowercase() != *tag_name {
                return false;
            }
        }

        // ID eşleşmesi
        if let Some(id) = &self.id {
            if element.attributes.get(&AttrName::Id).map_or(true, |attr_id| attr_id != id) {
                return false;
            }
        }

        // Sınıf eşleşmeleri
        if !self.class.is_empty() {
            if let Some(element_classes_str) = element.attributes.get(&AttrName::Class) {
                let element_classes: Vec<&str> = element_classes_str.split_whitespace().collect();
                for class_selector in &self.class {
                    if !element_classes.contains(&class_selector.as_str()) {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }
        true // Tüm seçici parçaları eşleşti
    }
}

