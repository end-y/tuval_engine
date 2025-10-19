use crate::style::structs::StyledNode;
use super::enums::LayoutBoxType;
use crate::css::enums::Value; // Value enum'unu kullanmak için eklendi

#[derive(Debug)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: LayoutBoxType,
    pub children: Vec<LayoutBox<'a>>,
    pub styled_node: Option<&'a StyledNode<'a>>, // Stil ağacındaki ilgili düğüme referans
}

#[derive(Debug, Default, Clone)] // Clone trait'i eklendi
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl Dimensions {
    // İçerik kutusunu, dolgu (padding) ile genişletir
    pub fn padding_box(&self) -> Rect {
        self.content.expanded_by(&self.padding)
    }

    // Dolgu kutusunu, kenarlık (border) ile genişletir
    pub fn border_box(&self) -> Rect {
        self.padding_box().expanded_by(&self.border)
    }

    // Kenarlık kutusunu, kenar boşluğu (margin) ile genişletir
    pub fn margin_box(&self) -> Rect {
        self.border_box().expanded_by(&self.margin)
    }
}

#[derive(Debug, Default, Clone)] // Clone trait'i eklendi
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    // Bir Rect'i EdgeSizes kadar genişletir
    pub fn expanded_by(&self, edge: &EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

#[derive(Debug, Default, Clone)] // Clone trait'i eklendi
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl<'a> LayoutBox<'a> {
    pub fn new(box_type: LayoutBoxType) -> LayoutBox<'a> {
        LayoutBox {
            dimensions: Default::default(),
            box_type,
            children: Vec::new(),
            styled_node: None,
        }
    }

    pub fn get_styled_node(&self) -> &StyledNode<'_> {
        self.styled_node.expect("LayoutBox without a styled node")
    }

    // Bu metod daha sonra kullanılacak, şimdilik sadece tanımlıyoruz
    pub fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            LayoutBoxType::BlockNode => self.layout_block(containing_block),
            LayoutBoxType::InlineNode => self.layout_inline(containing_block),
            LayoutBoxType::AnonymousBlock => self.layout_inline(containing_block),
            LayoutBoxType::LineBox => self.layout_line_box(containing_block),
        }
    }

    // Stil ağacındaki bir özelliği Value olarak döndürür
    fn get_property(&self, name: &str) -> Option<&Value> {
        self.get_styled_node().specified_values.get(name)
    }

    // CSS Value'den bir piksel değeri alır, yoksa varsayılanı döndürür
    fn get_float_value(&self, name: &str, default: f32) -> f32 {
        // Önce spesifik özelliği ara (örn: "margin-top")
        if let Some(&Value::Length(f, _)) = self.get_property(name) {
            return f;
        }

        // Eğer spesifik özellik bulunamazsa, genel özelliği ara (örn: "margin")
        let general_name = name.split('-').next().unwrap_or(name);
        if general_name != name { // Eğer spesifik bir özellik arıyorsak ve genel isim farklıysa
            if let Some(&Value::Length(f, _)) = self.get_property(general_name) {
                return f;
            }
        }
        default
    }

    // EdgeSizes'ı stil özelliklerinden hesaplar
    fn calculate_edge_sizes(&self, prefix: &str, default: f32) -> EdgeSizes {
        let all = self.get_float_value(prefix, default); // Genel değeri al

        EdgeSizes {
            top: self.get_float_value(&format!("{}-top", prefix), all), // Spesifik yoksa genel
            bottom: self.get_float_value(&format!("{}-bottom", prefix), all), // Spesifik yoksa genel
            left: self.get_float_value(&format!("{}-left", prefix), all), // Spesifik yoksa genel
            right: self.get_float_value(&format!("{}-right", prefix), all), // Spesifik yoksa genel
        }
    }

    fn layout_block(&mut self, containing_block: Dimensions) {
        self.dimensions.content.width = match self.get_property("width") {
            Some(Value::Length(w, _)) => *w,
            _ => {
                if let Some(styled_node) = self.styled_node {
                    if let crate::html::enums::NodeType::Text(ref text) = styled_node.node.node_type {
                        let font_size = self.get_float_value("font-size", 16.0);
                        text.len() as f32 * (font_size * 0.6)
                    } else {
                        containing_block.content.width - self.dimensions.margin.left - self.dimensions.margin.right - self.dimensions.border.left - self.dimensions.border.right - self.dimensions.padding.left - self.dimensions.padding.right
                    }
                } else {
                    containing_block.content.width - self.dimensions.margin.left - self.dimensions.margin.right - self.dimensions.border.left - self.dimensions.border.right - self.dimensions.padding.left - self.dimensions.padding.right
                }
            }
        };

        self.dimensions.margin = self.calculate_edge_sizes("margin", 0.0);
        self.dimensions.padding = self.calculate_edge_sizes("padding", 0.0);
        self.dimensions.border = self.calculate_edge_sizes("border", 0.0);

        self.dimensions.content.x = containing_block.content.x + self.dimensions.margin.left + self.dimensions.border.left + self.dimensions.padding.left;
        self.dimensions.content.y = containing_block.content.y + self.dimensions.margin.top + self.dimensions.border.top + self.dimensions.padding.top;

        if let Some(styled_node) = self.styled_node {
            if let crate::html::enums::NodeType::Text(ref text) = styled_node.node.node_type {
                let font_size = self.get_float_value("font-size", 16.0);
                let line_height = self.get_float_value("line-height", font_size * 1.2);
                self.dimensions.content.height = (text.len() as f32 / (self.dimensions.content.width / (font_size * 0.6))).max(1.0) * line_height;
                if self.dimensions.content.height == 0.0 && !text.is_empty() {
                    self.dimensions.content.height = line_height;
                }
            }
        }

        let mut current_y = self.dimensions.content.y;
        let mut line_children = Vec::new();
        let mut new_children = Vec::new(); // Yeni çocukları toplamak için geçici vektör

        for child in self.children.drain(..) {
            match child.box_type {
                LayoutBoxType::BlockNode => {
                    // Önceki satır içi çocukları bir satır kutusuna yerleştir ve düzenle
                    if !line_children.is_empty() {
                        let mut line_box = LayoutBox::new(LayoutBoxType::LineBox);
                        line_box.children.append(&mut line_children);
                        line_box.dimensions.content.y = current_y;

                        let line_containing_block = Dimensions {
                            content: Rect {
                                x: self.dimensions.content.x,
                                y: current_y,
                                width: self.dimensions.content.width,
                                height: 0.0,
                            },
                            ..Default::default()
                        };
                        line_box.layout(line_containing_block);
                        current_y += line_box.dimensions.margin_box().height;
                        new_children.push(line_box); // Oluşturulan satır kutusunu yeni vektöre ekle
                    }

                    // Blok çocuğu düzenle
                    let mut block_child = child;
                    block_child.dimensions.content.y = current_y;
                    let child_containing_block = Dimensions {
                        content: Rect {
                            x: self.dimensions.content.x,
                            y: current_y,
                            width: self.dimensions.content.width,
                            height: 0.0,
                        },
                        ..Default::default()
                    };
                    block_child.layout(child_containing_block);
                    current_y += block_child.dimensions.margin_box().height;
                    new_children.push(block_child); // Blok çocuğunu yeni vektöre ekle
                }
                LayoutBoxType::InlineNode | LayoutBoxType::AnonymousBlock => {
                    line_children.push(child); // Satır içi çocukları topla
                }
                _ => {} // Diğer LayoutBoxType'ları şimdilik yoksay
            }
        }

        // Kalan satır içi çocukları bir satır kutusuna yerleştir ve düzenle
        if !line_children.is_empty() {
            let mut line_box = LayoutBox::new(LayoutBoxType::LineBox);
            line_box.children.append(&mut line_children);
            line_box.dimensions.content.y = current_y;

            let line_containing_block = Dimensions {
                content: Rect {
                    x: self.dimensions.content.x,
                    y: current_y,
                    width: self.dimensions.content.width,
                    height: 0.0,
                },
                ..Default::default()
            };
            line_box.layout(line_containing_block);
            current_y += line_box.dimensions.margin_box().height;
            new_children.push(line_box); // Oluşturulan satır kutusunu yeni vektöre ekle
        }

        self.children = new_children; // self.children'ı yeni vektörle değiştir

        if let Some(styled_node) = self.styled_node {
            if let crate::html::enums::NodeType::Text(_) = styled_node.node.node_type {
            } else {
                let children_total_height = current_y - self.dimensions.content.y;
                self.dimensions.content.height = children_total_height;
            }
        } else {
            let children_total_height = current_y - self.dimensions.content.y;
            self.dimensions.content.height = children_total_height;
        }
    }

    // Satır içi düzenlemeyi yapar
    fn layout_inline(&mut self, containing_block: Dimensions) {
        self.dimensions.margin = self.calculate_edge_sizes("margin", 0.0);
        self.dimensions.padding = self.calculate_edge_sizes("padding", 0.0);
        self.dimensions.border = self.calculate_edge_sizes("border", 0.0);

        self.dimensions.content.x = containing_block.content.x + self.dimensions.margin.left + self.dimensions.border.left + self.dimensions.padding.left;
        self.dimensions.content.y = containing_block.content.y + self.dimensions.margin.top + self.dimensions.border.top + self.dimensions.padding.top;

        self.dimensions.content.width = match self.get_property("width") {
            Some(Value::Length(w, _)) => *w,
            _ => {
                if let Some(styled_node) = self.styled_node {
                    if let crate::html::enums::NodeType::Text(ref text) = styled_node.node.node_type {
                        let font_size = self.get_float_value("font-size", 16.0);
                        text.len() as f32 * (font_size * 0.6)
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            }
        };

        self.dimensions.content.height = match self.get_property("height") {
            Some(Value::Length(h, _)) => *h,
            _ => {
                if let Some(styled_node) = self.styled_node {
                    if let crate::html::enums::NodeType::Text(ref text) = styled_node.node.node_type {
                        let font_size = self.get_float_value("font-size", 16.0);
                        let line_height = self.get_float_value("line-height", font_size * 1.2);
                        (text.len() as f32 / (self.dimensions.content.width / (font_size * 0.6))).max(1.0) * line_height
                    } else {
                        self.get_float_value("font-size", 16.0) * 1.2
                    }
                } else {
                    self.get_float_value("font-size", 16.0) * 1.2
                }
            }
        };
    }

    // Satır kutularını düzenler (Inline Formatting Context)
    fn layout_line_box(&mut self, containing_block: Dimensions) {
        self.dimensions.content.x = containing_block.content.x;
        self.dimensions.content.y = containing_block.content.y;
        self.dimensions.content.width = containing_block.content.width;

        let mut current_x = self.dimensions.content.x;
        let mut max_height: f32 = 0.0;

        for child in &mut self.children {
            // Çocuğun kapsayan bloğunu, satır kutusunun mevcut konumuyla oluştur
            let child_containing_block = Dimensions {
                content: Rect {
                    x: current_x,
                    y: self.dimensions.content.y,
                    width: containing_block.content.width - (current_x - self.dimensions.content.x),
                    height: containing_block.content.height,
                },
                ..Default::default()
            };

            child.layout(child_containing_block);

            // Çocuğun boyutlarını kullanarak satır kutusunun genişliğini ve yüksekliğini güncelle
            current_x += child.dimensions.margin_box().width;
            max_height = max_height.max(child.dimensions.margin_box().height);
        }

        self.dimensions.content.width = current_x - self.dimensions.content.x;
        self.dimensions.content.height = max_height;
    }
}
