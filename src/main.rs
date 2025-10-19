mod html;
mod css;
// html modülünden gerekli öğeleri içe aktar
use html::{Parser as HtmlParser};
use css::{Parser as CssParser};
    
fn main() {
    let html_input = "<html><head><title>Merhaba Dunya</title></head><body><h1>Merhaba Dunya</h1><p>Bu bir paragraftir.</p></body></html>".to_string();
    let css_input = "body { font-family: Arial, sans-serif; } h1 { color: red; } p { color: blue; } p { font-size: 16px; } #id { color: green; } h1 { font-size: 20px; } .class { color: yellow; background-color: black; font-weight: bold; }".to_string();
    let dom_tree = HtmlParser::new(html_input).parse();
    let css_tree = CssParser::new(css_input).parse();
    println!("HTML ayrıştırma tamamlandı.");
    dom_tree.pretty_print(0);
    css_tree.pretty_print(0);
}
