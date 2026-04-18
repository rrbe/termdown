use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use crate::config::Config;
use crate::render::HeadingImage;
use crate::theme::Theme;

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

#[allow(dead_code)] // TODO: removed in Task 1.9 once main.rs consumes this
pub fn build(md: &str, _config: &Config, _theme: Theme) -> RenderedDoc {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(md, opts);

    let mut lines: Vec<Line> = Vec::new();
    let mut spans: Vec<Span> = Vec::new();
    let mut text_buf = String::new();
    let mut style = Style::default();

    for event in parser {
        match event {
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                flush_text(&mut text_buf, &mut spans, &style);
                lines.push(Line {
                    spans: std::mem::take(&mut spans),
                    kind: LineKind::Body,
                });
            }
            Event::Start(Tag::Strong) => {
                flush_text(&mut text_buf, &mut spans, &style);
                style.bold = true;
            }
            Event::End(TagEnd::Strong) => {
                flush_text(&mut text_buf, &mut spans, &style);
                style.bold = false;
            }
            Event::Start(Tag::Emphasis) => {
                flush_text(&mut text_buf, &mut spans, &style);
                style.italic = true;
            }
            Event::End(TagEnd::Emphasis) => {
                flush_text(&mut text_buf, &mut spans, &style);
                style.italic = false;
            }
            Event::Start(Tag::Strikethrough) => {
                flush_text(&mut text_buf, &mut spans, &style);
                style.strikethrough = true;
            }
            Event::End(TagEnd::Strikethrough) => {
                flush_text(&mut text_buf, &mut spans, &style);
                style.strikethrough = false;
            }
            Event::Text(t) => text_buf.push_str(&t),
            _ => {}
        }
    }

    RenderedDoc {
        lines,
        headings: vec![],
        images: vec![],
    }
}

/// Flush the pending plain-text buffer into a styled span and clear it.
#[allow(dead_code)]
fn flush_text(text_buf: &mut String, spans: &mut Vec<Span>, style: &Style) {
    if !text_buf.is_empty() {
        spans.push(Span::Text {
            content: std::mem::take(text_buf),
            style: style.clone(),
        });
    }
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

    use crate::config::Config;
    use crate::theme::Theme;

    fn build_plain(md: &str) -> RenderedDoc {
        let cfg = Config::default();
        super::build(md, &cfg, Theme::Dark)
    }

    #[test]
    fn build_single_paragraph() {
        let doc = build_plain("hello world\n");
        assert!(doc.lines.iter().any(
            |l| matches!(l.kind, LineKind::Body) && spans_plain_text(&l.spans) == "hello world"
        ));
    }

    fn spans_plain_text(spans: &[Span]) -> String {
        let mut out = String::new();
        for s in spans {
            match s {
                Span::Text { content, .. } => out.push_str(content),
                Span::Link { content, .. } => out.push_str(content),
                Span::HeadingImage { .. } => {}
            }
        }
        out
    }

    #[test]
    fn build_inline_bold_and_italic() {
        let doc = build_plain("hello **bold** and *it*\n");
        let line = doc
            .lines
            .iter()
            .find(|l| matches!(l.kind, LineKind::Body))
            .unwrap();

        let bold_span = line
            .spans
            .iter()
            .find(|s| matches!(s, Span::Text { style, .. } if style.bold));
        let italic_span = line
            .spans
            .iter()
            .find(|s| matches!(s, Span::Text { style, .. } if style.italic));

        assert!(matches!(bold_span, Some(Span::Text { content, .. }) if content == "bold"));
        assert!(matches!(italic_span, Some(Span::Text { content, .. }) if content == "it"));
    }

    #[test]
    fn build_inline_strikethrough() {
        let doc = build_plain("keep ~~drop~~ go\n");
        let line = doc
            .lines
            .iter()
            .find(|l| matches!(l.kind, LineKind::Body))
            .unwrap();
        let strike = line
            .spans
            .iter()
            .find(|s| matches!(s, Span::Text { style, .. } if style.strikethrough));
        assert!(matches!(strike, Some(Span::Text { content, .. }) if content == "drop"));
    }
}
