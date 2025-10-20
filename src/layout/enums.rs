#[derive(Debug)]
pub enum LayoutBoxType {
    BlockNode,
    #[allow(dead_code)]
    InlineNode,
    AnonymousBlock,
    LineBox,
}
