#![allow(dead_code)]

use crate::render::HeadingImage;

#[derive(Debug, Clone)]
pub struct RenderedDoc {
    pub lines: Vec<Line>,
    pub headings: Vec<HeadingEntry>,
    pub images: Vec<HeadingImage>,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub spans: Vec<Span>,
    pub kind: LineKind,
}

#[derive(Debug, Clone)]
pub enum LineKind {
    Body,
    Heading { level: u8, id: Option<u32> },
    CodeBlock { lang: Option<String> },
    BlockQuote { depth: u8 },
    ListItem { depth: u8 },
    Table,
    HorizontalRule,
    Blank,
}

#[derive(Debug, Clone)]
pub enum Span {
    Text {
        content: String,
        style: Style,
    },
    HeadingImage {
        id: u32,
        rows: u16,
    },
    Link {
        content: String,
        url: String,
        style: Style,
    },
}

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub dim: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    /// 256-color index (what the existing style.rs already emits).
    Indexed(u8),
    /// Truecolor fallback for future use.
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone)]
pub struct HeadingEntry {
    pub level: u8,
    pub text: String,
    pub line_index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_compile() {
        let _ = RenderedDoc {
            lines: vec![Line {
                spans: vec![Span::Text {
                    content: "hi".into(),
                    style: Style::default(),
                }],
                kind: LineKind::Body,
            }],
            headings: vec![],
            images: vec![],
        };
    }
}
