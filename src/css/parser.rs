use super::enums::{Value, Selector, Color, Unit, Display};
use super::structs::{StyleSheet, Rule, Declaration, Specificity};
use crate::css::enums::{SelectorType};

pub struct Parser {
    pos: usize,
    input: String,
}
fn valid_identifier(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')
}
fn is_color_value(value: &str) -> bool {
    let color_names = vec!["red", "green", "blue", "yellow", "purple", "orange", "pink", "brown", "gray", "black", "white", "transparent"];
    for color_name in color_names {
        if value.starts_with(color_name) {
            return true;
        }
    }
    value.starts_with("rgb(") || value.starts_with("rgba(") || value.starts_with("hsl(") || value.starts_with("hsla(")
}
fn is_length_value(value: &str) -> bool {
    value.ends_with("px") || value.ends_with("em") || value.ends_with("rem") || value.ends_with("vh") || value.ends_with("vw") || value.ends_with("vmin") || value.ends_with("vmax")
}
impl Parser {
    pub fn new(input: String) -> Parser {
        Parser { pos: 0, input }
    }
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].chars();
        let cur_char = iter.next().unwrap();
        self.pos += cur_char.len_utf8();
        cur_char
    }
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
    fn parse_selector(&mut self) -> SelectorType {
        let mut selector = SelectorType { tag_name: None, id: None, class: vec![] };

        // İlk olarak etiket adı veya evrensel seçiciyi kontrol et
        if !self.eof() && (valid_identifier(self.next_char()) || self.next_char() == '*') {
            selector.tag_name = Some(self.parse_identifier());
        }

        // Ardından ID ve sınıf seçicilerini döngüde kontrol et
        loop {
            if self.eof() {
                break;
            }
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                // Eğer hala bir tag_name ayarlanmadıysa ve geçerli bir tanımlayıcıysa
                c if selector.tag_name.is_none() && valid_identifier(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => {
                    break; // Diğer karakterlerde döngüyü sonlandır (örn: boşluk, {, ,)
                }
            }
        }
        selector
    }
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = vec![];
        loop {
            selectors.push(Selector::Type(self.parse_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    continue;
                }
                '{' => break,
                _ => panic!("Unexpected character: {}", self.next_char()),
            }
        }
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        selectors
    }
    fn parse_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while !self.eof() && valid_identifier(self.next_char()) {
            identifier.push(self.consume_char());
        }
        identifier
    }
    fn consume_whitespace(&mut self) {
        while !self.eof() && self.next_char().is_whitespace() {
            self.consume_char();
        }
    }
    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = vec![];
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                break;
            }
            declarations.push(self.parse_declaration());
            self.consume_whitespace();
        }
        declarations
    }
    fn parse_declaration(&mut self) -> Declaration {
        let property = self.parse_identifier();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ':');
        self.consume_whitespace();
        let value = self.parse_value(&property);
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ';');
        Declaration { property, value }
    }
    fn parse_value(&mut self, property: &str) -> Value {
        let mut value = String::new();
        while !self.eof() && (self.next_char() != ';' && self.next_char() != '}') {
            value.push(self.consume_char());
        }

        let trimmed_value = value.trim().to_lowercase();

        if property == "display" {
            match trimmed_value.as_str() {
                "block" => return Value::Display(Display::Block),
                "inline" => return Value::Display(Display::Inline),
                "none" => return Value::Display(Display::None),
                _ => {},
            }
        }

        if is_color_value(&value) {
            return Value::Color(self.parse_color(value));
        }
        if is_length_value(&value) {
            let length = self.parse_length_value(value.trim_end_matches(|c: char| !c.is_numeric()).to_string());
            let unit = self.parse_unit_value(value.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == '-').to_string());
            return Value::Length(length, unit);
        }

        Value::Keyword(value.trim().to_string())
    }
    fn parse_color(&mut self, color: String) -> Color {
        if color.starts_with("rgb(") || color.starts_with("rgba(") {
            let rgb = self.parse_rgb(color);
            return Color::RGBA(rgb.0, rgb.1, rgb.2, rgb.3);
        }
        if color.starts_with("hsl(") || color.starts_with("hsla(") {
            let hsl = self.parse_hsl();
            return Color::HSLA(hsl.0, hsl.1, hsl.2, hsl.3);
        }
        match color.trim().to_lowercase().as_str() {
            "red" => return Color::RGBA(255, 0, 0, 1.0),
            "green" => return Color::RGBA(0, 128, 0, 1.0),
            "blue" => return Color::RGBA(0, 0, 255, 1.0),
            "yellow" => return Color::RGBA(255, 255, 0, 1.0),
            "purple" => return Color::RGBA(128, 0, 128, 1.0),
            "orange" => return Color::RGBA(255, 165, 0, 1.0),
            "pink" => return Color::RGBA(255, 192, 203, 1.0),
            "brown" => return Color::RGBA(165, 42, 42, 1.0),
            "gray" => return Color::RGBA(128, 128, 128, 1.0),
            "black" => return Color::RGBA(0, 0, 0, 1.0),
            "white" => return Color::RGBA(255, 255, 255, 1.0),
            "transparent" => return Color::RGBA(0, 0, 0, 0.0),
            _ => {}
        }
        panic!("Invalid color value: {}", color);
    }
    fn parse_rgb(&mut self, color: String) -> (u8, u8, u8, f32) {
        let color = color.trim_matches(|c: char| !c.is_numeric() || c == '.' || c == '-');
        let rgb = color.split(',').map(|x| x.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        println!("RGB: {:?}", rgb);
        if rgb.len() == 3 {
            return (rgb[0], rgb[1], rgb[2], 1.0);
        } else if rgb.len() == 4 {
            return (rgb[0], rgb[1], rgb[2], rgb[3] as f32);
        } else {
            panic!("Invalid RGB value: {}", color);
        }
    }
    fn parse_length_value(&mut self, length: String) -> f32 {
        length.parse::<f32>().unwrap_or(0.0)
    }
    fn parse_unit_value(&mut self, unit: String) -> Unit {
        match unit.to_lowercase().as_str() {
            "px" => Unit::Px,
            "em" => Unit::Em,
            "rem" => Unit::Rem,
            "vh" => Unit::Vh,
            "vw" => Unit::Vw,
            "vmin" => Unit::Vmin,
            "vmax" => Unit::Vmax,
            _ => Unit::Px,
        }
    }
    fn parse_hsl(&mut self) -> (f32, f32, f32, f32) {
        let mut hsl = String::new();
        while !self.eof() && (self.next_char() != ';' && self.next_char() != '}') {
            hsl.push(self.consume_char());
        }
        let hsl = hsl.split(',').map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>();
        (hsl[0], hsl[1], hsl[2], hsl[3])
    }
    fn parse_rule(&mut self) -> Rule {
        let selectors = self.parse_selectors();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), '{');
        let declarations = self.parse_declarations();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), '}');
        Rule {
            selectors,
            declarations,
        }
    }
    pub fn parse(&mut self) -> StyleSheet {
        self.consume_whitespace();
        let mut rules = vec![];
        while !self.eof() {
            rules.push(self.parse_rule());
            self.consume_whitespace();
        }
        StyleSheet { rules }
    }
}
impl StyleSheet {
    pub fn pretty_print(&self, indent: usize) {
        for rule in &self.rules {
            rule.pretty_print(indent);
        }
    }
}
impl Rule {
    pub fn pretty_print(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}Rule:", indent_str);
        println!("{}  Selectors:", indent_str);
        for selector in &self.selectors {
            selector.pretty_print(indent + 4);
        }
        println!("{}  Declarations:", indent_str);
        for declaration in &self.declarations {
            declaration.pretty_print(indent + 4);
        }
    }
}
impl Selector {
    pub fn specificity(&self) -> Specificity {
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
    pub fn pretty_print(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        let Selector::Type(selector) = self;
        println!("{}Selector:", indent_str);
        if let Some(tag_name) = &selector.tag_name {
            println!("{}  Tag Name: {}", indent_str, tag_name);
        }
        if let Some(id) = &selector.id {
            println!("{}  Id: {}", indent_str, id);
        }
        for class in &selector.class {
            println!("{}  Class: {}", indent_str, class);
        }
    }
}
impl Declaration {
    pub fn pretty_print(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}Declaration: {}: {:?}", indent_str, self.property, self.value);
    }
}
