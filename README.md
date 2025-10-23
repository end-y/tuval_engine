# Tuval Engine

A lightweight, modular rendering engine written in Rust for rendering HTML and CSS to images.

## Overview

Tuval Engine is a browser rendering engine implementation that parses HTML and CSS, builds a DOM tree, applies styles, calculates layout using the CSS box model, and renders the final output to PNG images.

## Features

### ✅ Implemented

- **HTML Parser**
  - Complete HTML parsing with support for elements and text nodes
  - Attribute parsing (class, id, and 30+ HTML attributes)
  - Support for 40+ HTML tags (div, p, h1-h6, span, table, form elements, etc.)

- **CSS Parser**
  - CSS selector parsing (type, class, id selectors)
  - Property declarations with specificity calculation
  - Color support (RGBA, HSLA, named colors)
  - Length units (px, em, rem, vh, vw, vmin, vmax)
  - Display properties (block, inline, none)

- **Style Tree**
  - CSS rule matching and application
  - Style inheritance for inheritable properties (color, font-family, font-size, etc.)
  - Computed values with cascading support
  - Default display styles for common elements

- **Layout Engine**
  - CSS Box Model implementation (content, padding, border, margin)
  - Block and inline layout
  - Line box generation for inline content
  - Proper dimension calculation with edge sizes
  - Text layout with font metrics

- **Painting/Rendering**
  - Layered rendering with correct z-order
  - Background colors
  - Border rendering (all four sides)
  - Text rendering with TrueType font support (rusttype)
  - Alpha blending for text
  - PNG image output

- **Documentation**
  - Comprehensive module-level documentation
  - Struct and enum documentation
  - Inline code comments

## Architecture

```
HTML Input → HTML Parser → DOM Tree
                              ↓
CSS Input → CSS Parser → Stylesheet
                              ↓
                        Style Tree (DOM + CSS)
                              ↓
                        Layout Tree (Box Model)
                              ↓
                        Display List (Paint Commands)
                              ↓
                        PNG Image Output
```

## Usage

```rust
use tuval::*;

fn main() {
    let html = "<html><body><h1>Hello World</h1></body></html>";
    let css = "h1 { color: red; font-size: 50px; }";
    
    // Parse HTML and CSS
    let dom_tree = html::parser::Parser::new(html.to_string()).parse();
    let css_tree = css::parser::Parser::new(css.to_string()).parse();
    
    // Build style tree
    let styled_tree = style::structs::style_tree(&dom_tree, &css_tree);
    
    // Build layout tree
    let mut layout_tree = build_layout_tree(&styled_tree);
    
    // Calculate layout
    let viewport = Dimensions {
        content: Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 },
        ..Default::default()
    };
    layout_tree.layout(viewport);
    
    // Render to image
    let display_list = painting::structs::build_display_list(&layout_tree);
    painting::structs::paint_to_image(&display_list, 800, 600, "output.png");
}
```

## Dependencies

- `image` (0.24) - Image encoding/decoding
- `rusttype` (0.9.3) - TrueType font rendering
- `lazy_static` (1.5.0) - Lazy static initialization

## Recent Fixes

- ✅ Fixed Cargo.toml edition (2024 → 2021)
- ✅ Removed debug print statements
- ✅ Fixed RGB/RGBA color parsing
- ✅ Fixed rendering z-order (background → borders → content → text)
- ✅ Fixed layout calculation order (calculate padding before width)
- ✅ Proper box model implementation to prevent overflow

## To-Do (Future Plans)

- [ ] Support for more CSS properties (flexbox, grid, positioning)
- [ ] JavaScript engine integration
- [ ] Event handling system
- [ ] Self-closing HTML tags (img, br, meta, etc.)
- [ ] CSS pseudo-classes and pseudo-elements
- [ ] Media queries
- [ ] SVG support
- [ ] Performance optimizations
- [ ] Unit tests and integration tests

## Project Structure

```
tuval/
├── src/
│   ├── html/           # HTML parsing
│   │   ├── enums.rs    # NodeType, TagName, AttrName
│   │   ├── structs.rs  # Node, ElementData
│   │   └── parser.rs   # HTML parser
│   ├── css/            # CSS parsing
│   │   ├── enums.rs    # Value, Color, Unit, Display, Selector
│   │   ├── structs.rs  # StyleSheet, Rule, Declaration
│   │   └── parser.rs   # CSS parser
│   ├── style/          # Style tree
│   │   └── structs.rs  # StyledNode, style matching
│   ├── layout/         # Layout engine
│   │   ├── enums.rs    # LayoutBoxType
│   │   └── structs.rs  # LayoutBox, Dimensions, Rect
│   ├── painting/       # Rendering
│   │   ├── enums.rs    # Command (display commands)
│   │   └── structs.rs  # DisplayList, rendering functions
│   └── main.rs         # Example usage
├── Arial.ttf           # Font file
└── Cargo.toml
```

## License

This project is open source and available for educational purposes.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
