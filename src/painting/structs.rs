use super::enums::Command;
use crate::html::enums::NodeType;
use crate::layout::structs::{LayoutBox, Rect};
use crate::css::enums::{Value, Color};
use rusttype::{point, Font, Scale};

#[derive(Debug, Default)]
pub struct DisplayList {
    pub commands: Vec<Command>,
}

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = DisplayList::default();
    render_layout_box(&mut list, layout_root);
    list
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    // Metin düğümleri için metin çizim komutlarını oluştur
    let mut default_color = Color::RGBA(0, 0, 0, 1.0);
    let mut default_font_size = 16.0;
    if let Some(styled_node) = layout_box.styled_node {
        if let Some(Value::Color(color)) = styled_node.get_property("color") {
            default_color = color.clone();
            //list.commands.push(Command::Text(styled_node.children[0].node.node_type.clone(), layout_box.dimensions.content.clone(), color.clone(), 16.0));
        }
        if let Some(Value::Length(font_size, _)) = styled_node.get_property("font-size") {
            default_font_size = font_size.clone();
        }
        if styled_node.children.len() > 0 {
            if let NodeType::Text(ref text) = styled_node.children[0].node.node_type {
                list.commands.push(Command::Text(text.to_string(), layout_box.dimensions.content.clone(), default_color.clone(), default_font_size.clone()));
            } 
        }
    }   
    // Arka plan rengini çiz
    if let Some(styled_node) = layout_box.styled_node {
        if let Some(Value::Color(color)) = styled_node.get_property("background-color") {
            list.commands.push(Command::SolidColor(color.clone(), layout_box.dimensions.border_box()));
        }
    }

    // Kenarlıkları çiz
    if layout_box.dimensions.border.top > 0.0 || layout_box.dimensions.border.bottom > 0.0 ||
       layout_box.dimensions.border.left > 0.0 || layout_box.dimensions.border.right > 0.0 {
        if let Some(styled_node) = layout_box.styled_node {
            if let Some(Value::Color(border_color)) = styled_node.get_property("border-color") {
                let border_box = layout_box.dimensions.border_box();
                // Üst kenarlık
                list.commands.push(Command::SolidColor(border_color.clone(), crate::layout::structs::Rect {
                    x: border_box.x,
                    y: border_box.y,
                    width: border_box.width,
                    height: layout_box.dimensions.border.top,
                }));
                // Alt kenarlık
                list.commands.push(Command::SolidColor(border_color.clone(), crate::layout::structs::Rect {
                    x: border_box.x,
                    y: border_box.y + border_box.height - layout_box.dimensions.border.bottom,
                    width: border_box.width,
                    height: layout_box.dimensions.border.bottom,
                }));
                // Sol kenarlık
                list.commands.push(Command::SolidColor(border_color.clone(), crate::layout::structs::Rect {
                    x: border_box.x,
                    y: border_box.y + layout_box.dimensions.border.top,
                    width: layout_box.dimensions.border.left,
                    height: border_box.height - layout_box.dimensions.border.top - layout_box.dimensions.border.bottom,
                }));
                // Sağ kenarlık
                list.commands.push(Command::SolidColor(border_color.clone(), crate::layout::structs::Rect {
                    x: border_box.x + border_box.width - layout_box.dimensions.border.right,
                    y: border_box.y + layout_box.dimensions.border.top,
                    width: layout_box.dimensions.border.right,
                    height: border_box.height - layout_box.dimensions.border.top - layout_box.dimensions.border.bottom,
                }));
            }
        }
    }

    // Çocukları render et
    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

pub fn paint_to_image(display_list: &DisplayList, width: u32, height: u32, filename: &str) {
    let mut img = image::RgbaImage::new(width, height);
    // Fontu yükle
    let font_data = include_bytes!("../../Arial.ttf"); // Kök dizinde Arial.ttf olduğunu varsayıyoruz
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Failed to load font");
    // Arka planı beyaza boya
    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
        }
    }

    for command in &display_list.commands {
        match command {
            Command::SolidColor(color, rect) => {
                let rgba_color = match color {
                    Color::RGBA(r, g, b, a) => image::Rgba([*r, *g, *b, (a * 255.0) as u8]),
                    _ => image::Rgba([0, 0, 0, 255]), // Desteklenmeyen renk tipi için varsayılan siyah
                };

                let x_start = rect.x.round() as u32;
                let y_start = rect.y.round() as u32;
                let x_end = (rect.x + rect.width).round() as u32;
                let y_end = (rect.y + rect.height).round() as u32;

                for x in x_start.min(width)..x_end.min(width) {
                    for y in y_start.min(height)..y_end.min(height) {
                        img.put_pixel(x, y, rgba_color);
                    }
                }
            }
            Command::Text(text, rect, color, font_size) => {
                let rgba_color = match color {
                    Color::RGBA(r, g, b, a) => image::Rgba([*r, *g, *b, (a * 255.0) as u8]),
                    _ => image::Rgba([0, 0, 0, 255]), // Desteklenmeyen renk tipi için varsayılan siyah
                };
                let scale = Scale::uniform(*font_size);
                let v_metrics = font.v_metrics(scale);
                let offset = point(rect.x, rect.y + v_metrics.ascent);
                for glyph in font.layout(&text, scale, offset) {
                    if let Some(bounding_box) = glyph.pixel_bounding_box() {

                        glyph.draw(|x, y, v| {
                            let x = x + bounding_box.min.x as u32;
                            let y = y + bounding_box.min.y as u32;
                            if x < width && y < height {
                                let source_alpha = rgba_color[3] as f32 / 255.0;
                                let effective_source_alpha = v * source_alpha;

                                let existing_pixel = img.get_pixel(x, y).0;
                                let target_alpha = existing_pixel[3] as f32 / 255.0;

                                // Basit alpha blending
                                let blended_r = (existing_pixel[0] as f32 * (1.0 - effective_source_alpha) + rgba_color[0] as f32 * effective_source_alpha) as u8;
                                let blended_g = (existing_pixel[1] as f32 * (1.0 - effective_source_alpha) + rgba_color[1] as f32 * effective_source_alpha) as u8;
                                let blended_b = (existing_pixel[2] as f32 * (1.0 - effective_source_alpha) + rgba_color[2] as f32 * effective_source_alpha) as u8;
                                let blended_a = (target_alpha + effective_source_alpha * (1.0 - target_alpha)) as u8 * 255;
                                // Arka plan tamamen opak olduğu için, hedef pikselin alfa değeri her zaman 255 olacak.
                                img.put_pixel(x, y, image::Rgba([blended_r, blended_g, blended_b, blended_a]));
                            }
                        });
                    }
                }
            }
        }
    }

    img.save(filename).expect("Failed to save image");
    println!("Resim {} dosyasına kaydedildi.", filename);
}
