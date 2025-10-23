//! HTML parsing enums module
//!
//! This module defines the core enums used for HTML parsing,
//! including node types, attribute names, and tag names.

/// Represents the type of an HTML node (Element or Text)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NodeType {
    Element(super::structs::ElementData),
    Text(String),
}
/// Represents HTML attribute names
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AttrName {
    Class,
    Id,
    Href,
    Target,
    Rel,
    Disabled,
    Required,
    Readonly,
    Autofocus,
    Autocomplete,
    Autoplay,
    Controls,
    Loop,
    Muted,
    Preload,
    Type,
    Name,
    Value,
    Placeholder,
    Pattern,
    Minlength,
    Maxlength,
    Min,
    Max,
    Step,
    Multiple,
    Accept,
    Action,
    Method,
    Enctype,
}

/// Represents HTML tag names
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TagName {
    Html,
    Head,
    Body,
    Div,
    P,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Title,
    Span,
    A,
    Img,
    Ul,
    Ol,
    Li,
    Table,
    Tr,
    Td,
    Th,
    Tbody,
    Thead,
    Tfoot,
    Caption,
    Colgroup,
    Col,
    Form,
    Input,
    Label,
    Button,
    Select,
    Option,
    Textarea,
    Fieldset,
    Legend,
    Datalist,
    Keygen,
    Output,
    Progress,
    Meter,
    Article,
    Aside,
    Details,
    Summary,
    Mark,
    Time,
    Ruby,
    Rt,
    Rp,
    Bdi,
    Bdo,
    Wbr,
    Samp,
    Kbd,
    Q,
    Var,
}

impl std::fmt::Display for TagName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
