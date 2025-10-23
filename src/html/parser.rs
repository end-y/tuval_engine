//! HTML parser module
//!
//! This module provides functionality for parsing HTML strings into a DOM tree.

use std::collections::HashMap;
use super::enums::{AttrName, NodeType, TagName};
use super::structs::{ElementData, Node};

/// HTML parser that converts HTML strings into a DOM tree
pub struct Parser {
    pub pos: usize,
    pub input: String,
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
    // read while function : it reads the string while the test function is true
    fn read_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let start = self.pos;
        while !self.eof() && test(self.next_char()) {
            self.consume_char(); // self.pos'u doğru şekilde ilerletmek için consume_char kullanın
        }
        self.input[start..self.pos].to_string()
    }
    // end of file check
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn parse(&mut self) -> Node {
        self.parse_node()
    }
    fn parse_children(&mut self) -> Vec<Node> {
        let mut children = vec![];
        loop {
            self.consume_whitespace();

            if self.eof() {
                break;
            }

            // Kapanış etiketi mi, kontrol et. Örn: </div>
            if self.next_char() == '<' && (self.pos + 1 < self.input.len()) && self.input[self.pos + 1..].starts_with("/") {
                // Kapanış etiketini tüketmiyoruz, parse_element'ın yapması gerekiyor
                break;
            }

            // Normal bir düğüm (element veya metin) ayrıştırmaya çalış
            children.push(self.parse_node());
        }
        children
    }
    fn consume_whitespace(&mut self) {
        while !self.eof() && self.next_char().is_whitespace() {
            self.consume_char();
        }
    }
    pub fn parse_node(&mut self) -> Node {
        self.consume_whitespace();
        if self.eof() {
            panic!("Beklenmedik dosya sonu");
        }
        if self.next_char() == '<' {
            self.parse_element()
        } else {
            Node { node_type: NodeType::Text(self.parse_text()), children: vec![] }
        }
    }
    fn parse_tag_name(&mut self) -> TagName {
        self.consume_whitespace(); // Etiket adından önce olası boşlukları tüket
        let tag_name_str = self.read_while(|c| c.is_alphanumeric() || c == '-');
        self.convert_string_to_tag_name(tag_name_str)
    }
    fn parse_attributes(&mut self) -> HashMap<AttrName, String> {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.next_char() == '>' || self.next_char() == '/' {
                break; // Niteliklerin sonu
            }

            // Nitelik adını bulmak için
            let attr_name = self.parse_attr_name();

            if !self.eof() && self.next_char() == '=' {
                self.consume_char(); // '=' karakterini tüket
                let attr_value = self.parse_attr_value();
                attributes.insert(attr_name, attr_value);
            } else {
                // Nitelik değeri olmayan bir nitelik (örn. <input disabled>)
                attributes.insert(attr_name, String::new());
            }
        }
        attributes
    }

    fn parse_attr_name(&mut self) -> AttrName {
        let attr_name_str = self.read_while(|c| c.is_alphanumeric() || c == '-');
        self.convert_string_to_attr_name(attr_name_str)
    }

    fn parse_attr_value(&mut self) -> String {
        if !self.eof() && (self.next_char() == '"' || self.next_char() == '\'') {
            let open_quote = self.consume_char(); // Açılış tırnak işaretini tüket
            let attr_value_str = self.read_while(|c| c != open_quote);
            if !self.eof() {
                self.consume_char(); // Kapanış tırnak işaretini tüket
            }
            attr_value_str
        } else {
            self.read_while(|c| !c.is_whitespace() && c != '>' && c != '/' && c != '=')
        }
    }

    fn parse_element(&mut self) -> Node {
        self.consume_char(); // '<' karakterini tüket
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();

        self.consume_whitespace(); // Niteliklerden sonra gelen boşlukları tüket

        let children;
        // Kendini kapatan etiket (örn. <img />) durumunu kontrol et
        if self.next_char() == '/' {
            self.consume_char(); // '/' tüket
            self.consume_char(); // '>' tüket
            children = vec![];
        } else {
            self.consume_char(); // '>' tüket (açılış etiketinin kapanışı)
            children = self.parse_children();
            // Alt düğümler ayrıştırıldıktan sonra, bu elementin kapanış etiketini ayrıştırmalıyız.
            self.parse_closing_tag(&tag_name); 
        }

        Node { node_type: NodeType::Element(ElementData { tag_name, attributes }), children }
    }
    fn parse_text(&mut self) -> String {
        let mut text = String::new();
        while !self.eof() && self.next_char() != '<' {
            text.push(self.consume_char());
        }
        text
    }
    fn parse_closing_tag(&mut self, expected_tag_name: &TagName) {
        self.consume_whitespace(); // Kapanış etiketinden önce olası boşlukları tüket
        self.consume_char(); // '<' tüket
        self.consume_char(); // '/' tüket
        let closing_tag_name_str = self.read_while(|c| c.is_alphanumeric());
        self.consume_whitespace(); // Kapanış etiket adından sonra olası boşlukları tüket
        self.consume_char(); // '>' tüket

        let actual_closing_tag_name = self.convert_string_to_tag_name(closing_tag_name_str);
        if &actual_closing_tag_name != expected_tag_name {
            panic!("Mismatched closing tag: expected {:?}, got {:?}", expected_tag_name, actual_closing_tag_name);
        }
    }

    fn convert_string_to_tag_name(&mut self, string: String) -> TagName {
        match string.as_str() {
            "html" => TagName::Html,
            "head" => TagName::Head,
            "body" => TagName::Body,
            "div" => TagName::Div,
            "p" => TagName::P,
            "h1" => TagName::H1,
            "h2" => TagName::H2,
            "h3" => TagName::H3,
            "h4" => TagName::H4,
            "h5" => TagName::H5,
            "h6" => TagName::H6,
            "title" => TagName::Title,
            "span" => TagName::Span,
            "a" => TagName::A,
            "img" => TagName::Img,
            "ul" => TagName::Ul,
            "ol" => TagName::Ol,
            "li" => TagName::Li,
            "table" => TagName::Table,
            "tr" => TagName::Tr,
            "td" => TagName::Td,
            "th" => TagName::Th,
            "tbody" => TagName::Tbody,
            "thead" => TagName::Thead,
            "tfoot" => TagName::Tfoot,
            "caption" => TagName::Caption,
            "colgroup" => TagName::Colgroup,
            "col" => TagName::Col,
            "form" => TagName::Form,
            "input" => TagName::Input,
            "label" => TagName::Label,
            "button" => TagName::Button,
            "select" => TagName::Select,
            "option" => TagName::Option,
            "textarea" => TagName::Textarea,
            "fieldset" => TagName::Fieldset,
            "legend" => TagName::Legend,
            "datalist" => TagName::Datalist,
            "keygen" => TagName::Keygen,
            "output" => TagName::Output,
            "progress" => TagName::Progress,
            "meter" => TagName::Meter,
            "article" => TagName::Article,
            "aside" => TagName::Aside,
            "details" => TagName::Details,
            "summary" => TagName::Summary,
            "mark" => TagName::Mark,
            "time" => TagName::Time,
            "ruby" => TagName::Ruby,
            "rt" => TagName::Rt,
            "rp" => TagName::Rp,
            "bdi" => TagName::Bdi,
            "bdo" => TagName::Bdo,
            "wbr" => TagName::Wbr,
            "samp" => TagName::Samp,
            "kbd" => TagName::Kbd,
            "q" => TagName::Q,
            "var" => TagName::Var,
            _ => TagName::Html,
        }
    }
    fn convert_string_to_attr_name(&mut self, string: String) -> AttrName {
        match string.as_str() {
            "class" => AttrName::Class,
            "id" => AttrName::Id,
            "href" => AttrName::Href,
            "target" => AttrName::Target,
            "rel" => AttrName::Rel,
            "disabled" => AttrName::Disabled,
            "required" => AttrName::Required,
            "readonly" => AttrName::Readonly,
            "autofocus" => AttrName::Autofocus,
            "autocomplete" => AttrName::Autocomplete,
            "autoplay" => AttrName::Autoplay,
            "controls" => AttrName::Controls,
            "loop" => AttrName::Loop,
            "muted" => AttrName::Muted,
            "preload" => AttrName::Preload,
            "type" => AttrName::Type,
            "name" => AttrName::Name,
            "value" => AttrName::Value,
            "placeholder" => AttrName::Placeholder,
            "pattern" => AttrName::Pattern,
            "minlength" => AttrName::Minlength,
            "maxlength" => AttrName::Maxlength,
            "min" => AttrName::Min,
            "max" => AttrName::Max,
            "step" => AttrName::Step,
            "multiple" => AttrName::Multiple,
            "accept" => AttrName::Accept,
            "action" => AttrName::Action,
            "method" => AttrName::Method,
            "enctype" => AttrName::Enctype,
            _ => AttrName::Class,
        }
    }
}
