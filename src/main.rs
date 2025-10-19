mod html;
mod css;
mod style;
mod layout;

// html modülünden gerekli öğeleri içe aktar
use html::parser::Parser as HtmlParser;
use css::parser::Parser as CssParser;
use style::structs::{style_tree, StyledNode};
use layout::structs::{LayoutBox, Dimensions};
use layout::enums::LayoutBoxType;
use html::enums::NodeType; // Düzeltilen import

fn main() {
    let html_input = "<html><head><title>Merhaba Dunya</title></head><body><h1>Merhaba Dunya</h1><p class=\"class\">Bu bir paragraftir.</p></body></html>".to_string();
    let css_input = "body { font-family: Arial, sans-serif; } h1 { color: red; display: inline; padding: 5px; border-width: 1px; margin: 10px; } p { color: blue; font-size: 16px; margin: 10px; padding: 5px; border-width: 1px; border-style: solid; border-color: black; } #id { color: green; } h1 { font-size: 20px; } .class { color: yellow; background-color: black; font-weight: bold; margin-top: 20px; }".to_string(); // h1 için padding, border, margin eklendi
    let dom_tree = HtmlParser::new(html_input).parse();
    let css_tree = CssParser::new(css_input).parse();
    println!("HTML ayrıştırma tamamlandı.");
    dom_tree.pretty_print(0);
    println!("CSS ayrıştırma tamamlandı.");
    css_tree.pretty_print(0);

    // Stil ağacını oluştur ve yazdır
    let styled_tree = style_tree(&dom_tree, &css_tree);
    println!("Stil ağacı oluşturuldu.");
    styled_tree.pretty_print(0);

    // Düzen ağacını oluştur
    let mut layout_tree = build_layout_tree(&styled_tree);
    println!("Düzen ağacı oluşturuldu.");

    // Düzen hesaplamasını başlat
    let initial_containing_block = Dimensions {
        content: crate::layout::structs::Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 }, // Varsayılan pencere boyutları
        ..Default::default()
    };
    layout_tree.layout(initial_containing_block);

    println!("{:#?}", layout_tree); // Düzen ağacını yazdır
}

// Stil ağacından düzen ağacını oluşturan yardımcı fonksiyon
fn build_layout_tree<'a>(styled_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    let display = styled_node.get_property("display")
                             .and_then(|v| {
                                 if let crate::css::enums::Value::Display(d) = v {
                                     Some(d.clone()) // Display değerini klonla
                                 } else {
                                     None
                                 }
                             })
                             .unwrap_or_else(|| crate::css::enums::Display::Block); // Varsayılan olarak Block

    let mut layout_box = LayoutBox::new(match styled_node.node.node_type { // Bu satırda styled_node.node.node_type kullanıldığı için NodeType import'u gerekli
        NodeType::Element(_) => { // `ref data` yerine `_` kullanıldı
            match display {
                crate::css::enums::Display::Block => LayoutBoxType::BlockNode,
                crate::css::enums::Display::Inline => LayoutBoxType::InlineNode,
                crate::css::enums::Display::None => LayoutBoxType::BlockNode,
            }
        },
        NodeType::Text(_) => LayoutBoxType::AnonymousBlock, // Metin düğümleri için AnonymousBlock
    });
    layout_box.styled_node = Some(styled_node);

    for child_styled_node in &styled_node.children {
        layout_box.children.push(build_layout_tree(child_styled_node));
    }

    println!("Layout box: {:#?}", layout_box);
    layout_box
}
