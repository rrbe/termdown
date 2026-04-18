use crate::render::HeadingImage;

// The following items are scaffolding for the TUI pipeline introduced in Task 1.1.
// Each #[allow(dead_code)] is intentionally temporary and should be removed as the
// corresponding consumer task wires it up (Task 1.2+ for layout/rendering consumers).

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedDoc {
    pub lines: Vec<Line>,
    pub headings: Vec<HeadingEntry>,
    pub images: Vec<HeadingImage>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub spans: Vec<Span>,
    pub kind: LineKind,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineKind {
    Body,
    Heading {
        level: u8,
        id: Option<u32>, // Some for H1-H3 (image), None for H4-H6 (text)
    },
    CodeBlock {
        lang: Option<String>,
    },
    BlockQuote {
        depth: u8,
    },
    ListItem {
        depth: u8,
    },
    Table,
    HorizontalRule,
    Blank,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub dim: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// 256-color index (what the existing style.rs already emits).
    Indexed(u8),
    /// Truecolor fallback for future use.
    Rgb(u8, u8, u8),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
