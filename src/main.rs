mod html;
// html modülünden gerekli öğeleri içe aktar
use html::{Node, Parser};
    
fn main() {
    let html_input = "<html><head><title>Merhaba Dunya</title></head><body><h1>Merhaba Dunya</h1><p>Bu bir paragraftir.</p></body></html>".to_string();
    let dom_tree = Parser::new(html_input).parse();
    println!("HTML ayrıştırma tamamlandı.");
    dom_tree.pretty_print(0);
    dom_tree.print_node();
}
