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

#[allow(dead_code)]
struct ListState {
    ordered: bool,
    counter: u64,
}

#[allow(dead_code)] // TODO: removed in Task 1.9 once main.rs consumes this
pub fn build(md: &str, config: &Config, theme: Theme) -> RenderedDoc {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(md, opts);

    let mut lines: Vec<Line> = Vec::new();
    let mut spans: Vec<Span> = Vec::new();
    let mut text_buf = String::new();
    let mut style = Style::default();
    let mut pending_link_url: Option<String> = None;
    let mut heading_level: u8 = 0;
    let mut heading_text = String::new();
    let mut next_image_id: u32 = 1;
    let mut images: Vec<HeadingImage> = Vec::new();
    let mut headings: Vec<HeadingEntry> = Vec::new();
    let mut quote_depth: u8 = 0;
    let mut list_stack: Vec<ListState> = Vec::new();
    let mut in_code_block: Option<Option<String>> = None;
    let mut table_rows: Vec<Vec<Vec<Span>>> = Vec::new();
    let mut current_row: Vec<Vec<Span>> = Vec::new();
    let mut in_table_header = false;
    let mut image_url: Option<String> = None;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                heading_level = match level {
                    pulldown_cmark::HeadingLevel::H1 => 1,
                    pulldown_cmark::HeadingLevel::H2 => 2,
                    pulldown_cmark::HeadingLevel::H3 => 3,
                    pulldown_cmark::HeadingLevel::H4 => 4,
                    pulldown_cmark::HeadingLevel::H5 => 5,
                    pulldown_cmark::HeadingLevel::H6 => 6,
                };
                heading_text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                let text = std::mem::take(&mut heading_text);
                headings.push(HeadingEntry {
                    level: heading_level,
                    text: text.clone(),
                    line_index: lines.len(),
                });

                let (id_for_kind, heading_spans): (Option<u32>, Vec<Span>) = if heading_level <= 3 {
                    match crate::render::render_heading(&text, heading_level, config, theme) {
                        Some(png) => {
                            let id = next_image_id;
                            next_image_id += 1;
                            let rows = match heading_level {
                                1 => 6,
                                2 => 4,
                                _ => 3,
                            };
                            images.push(HeadingImage {
                                id,
                                png,
                                cols: 0,
                                rows,
                            });
                            (Some(id), vec![Span::HeadingImage { id, rows }])
                        }
                        None => (
                            None,
                            vec![Span::Text {
                                content: text.clone(),
                                style: Style {
                                    bold: true,
                                    ..Style::default()
                                },
                            }],
                        ),
                    }
                } else {
                    (
                        None,
                        vec![Span::Text {
                            content: text.clone(),
                            style: Style {
                                bold: true,
                                ..Style::default()
                            },
                        }],
                    )
                };

                lines.push(Line {
                    spans: heading_spans,
                    kind: LineKind::Heading {
                        level: heading_level,
                        id: id_for_kind,
                    },
                });
                heading_level = 0;
            }
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                flush_text(&mut text_buf, &mut spans, &style);
                let kind = if quote_depth > 0 {
                    LineKind::BlockQuote { depth: quote_depth }
                } else {
                    LineKind::Body
                };
                lines.push(Line {
                    spans: std::mem::take(&mut spans),
                    kind,
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
            Event::Start(Tag::Link { dest_url, .. }) => {
                flush_text(&mut text_buf, &mut spans, &style);
                pending_link_url = Some(dest_url.to_string());
            }
            Event::End(TagEnd::Link) => {
                if let Some(url) = pending_link_url.take() {
                    let content = std::mem::take(&mut text_buf);
                    spans.push(Span::Link {
                        content,
                        url,
                        style: style.clone(),
                    });
                }
            }
            Event::Code(code) => {
                flush_text(&mut text_buf, &mut spans, &style);
                let mut code_style = style.clone();
                code_style.bg = Some(Color::Indexed(236));
                code_style.fg = Some(Color::Indexed(213));
                spans.push(Span::Text {
                    content: code.to_string(),
                    style: code_style,
                });
            }
            Event::Text(t) => {
                if heading_level > 0 {
                    heading_text.push_str(&t);
                } else if let Some(lang) = in_code_block.clone() {
                    for line in t.lines() {
                        lines.push(Line {
                            spans: vec![Span::Text {
                                content: line.to_string(),
                                style: Style::default(),
                            }],
                            kind: LineKind::CodeBlock { lang: lang.clone() },
                        });
                    }
                } else {
                    text_buf.push_str(&t);
                }
            }
            Event::Start(Tag::BlockQuote(..)) => quote_depth += 1,
            Event::End(TagEnd::BlockQuote(..)) => quote_depth = quote_depth.saturating_sub(1),

            Event::Start(Tag::List(start)) => {
                list_stack.push(ListState {
                    ordered: start.is_some(),
                    counter: start.unwrap_or(1),
                });
            }
            Event::End(TagEnd::List(..)) => {
                list_stack.pop();
            }

            Event::Start(Tag::Item) => {
                // New item: clear any prior span accumulator so item content starts clean.
            }
            Event::End(TagEnd::Item) => {
                flush_text(&mut text_buf, &mut spans, &style);
                let depth = list_stack.len() as u8;
                lines.push(Line {
                    spans: std::mem::take(&mut spans),
                    kind: LineKind::ListItem { depth },
                });
            }

            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(s) if !s.is_empty() => {
                        Some(s.to_string())
                    }
                    _ => None,
                };
                in_code_block = Some(lang);
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = None;
            }

            Event::Rule => {
                lines.push(Line {
                    spans: vec![],
                    kind: LineKind::HorizontalRule,
                });
            }

            // Tables
            Event::Start(Tag::Table(..)) => {
                table_rows.clear();
                in_table_header = false;
            }
            Event::End(TagEnd::Table) => {
                emit_table(&mut lines, &table_rows);
                table_rows.clear();
            }
            Event::Start(Tag::TableHead) => {
                in_table_header = true;
                current_row.clear();
            }
            Event::End(TagEnd::TableHead) => {
                table_rows.push(std::mem::take(&mut current_row));
                in_table_header = false;
            }
            Event::Start(Tag::TableRow) => {
                current_row.clear();
            }
            Event::End(TagEnd::TableRow) => {
                table_rows.push(std::mem::take(&mut current_row));
            }
            Event::Start(Tag::TableCell) => {
                spans.clear();
            }
            Event::End(TagEnd::TableCell) => {
                flush_text(&mut text_buf, &mut spans, &style);
                if in_table_header {
                    for s in spans.iter_mut() {
                        if let Span::Text { style, .. } = s {
                            style.bold = true;
                        }
                    }
                }
                current_row.push(std::mem::take(&mut spans));
            }

            // Images
            Event::Start(Tag::Image { dest_url, .. }) => {
                flush_text(&mut text_buf, &mut spans, &style);
                image_url = Some(dest_url.to_string());
            }
            Event::End(TagEnd::Image) => {
                flush_text(&mut text_buf, &mut spans, &style);
                let alt = spans_plain_text_inline(&spans);
                spans.clear();
                let url = image_url.take().unwrap_or_default();
                let content = format!("[\u{1f5bc} {alt}]({url})");
                let dim_style = Style {
                    dim: true,
                    ..Style::default()
                };
                lines.push(Line {
                    spans: vec![Span::Text {
                        content,
                        style: dim_style,
                    }],
                    kind: LineKind::Body,
                });
            }

            // Task list marker
            Event::TaskListMarker(checked) => {
                let marker = if checked { "[\u{2713}] " } else { "[ ] " };
                if spans.is_empty() && text_buf.is_empty() {
                    text_buf.push_str(marker);
                } else if let Some(Span::Text { content, .. }) = spans.first_mut() {
                    *content = format!("{marker}{content}");
                } else {
                    text_buf = format!("{marker}{text_buf}");
                }
            }

            // HTML (block and inline)
            Event::Html(s) => {
                let dim_style = Style {
                    dim: true,
                    ..Style::default()
                };
                for line in s.lines() {
                    lines.push(Line {
                        spans: vec![Span::Text {
                            content: line.to_string(),
                            style: dim_style.clone(),
                        }],
                        kind: LineKind::Body,
                    });
                }
            }
            Event::InlineHtml(s) => {
                text_buf.push_str(&s);
            }

            // Breaks
            Event::SoftBreak | Event::HardBreak => {
                if heading_level > 0 {
                    heading_text.push(' ');
                } else {
                    text_buf.push(' ');
                }
            }

            _ => {}
        }
    }

    RenderedDoc {
        lines,
        headings,
        images,
    }
}

/// Render accumulated table rows into `LineKind::Table` lines with padding and separators.
/// Keeps the margin-less column layout the existing cat mode produces — the outer
/// "    " margin is added by `cat.rs`.
#[allow(dead_code)]
fn emit_table(lines: &mut Vec<Line>, rows: &[Vec<Vec<Span>>]) {
    if rows.is_empty() {
        return;
    }
    let cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut widths = vec![0usize; cols];
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            let w: usize = cell.iter().map(plain_width).sum();
            if let Some(slot) = widths.get_mut(i) {
                *slot = (*slot).max(w);
            }
        }
    }

    for (ri, row) in rows.iter().enumerate() {
        let mut out_spans: Vec<Span> = Vec::new();
        for (i, cell) in row.iter().enumerate() {
            for s in cell {
                out_spans.push(s.clone());
            }
            let w: usize = cell.iter().map(plain_width).sum();
            let pad = widths[i].saturating_sub(w);
            if pad > 0 {
                out_spans.push(Span::Text {
                    content: " ".repeat(pad),
                    style: Style::default(),
                });
            }
            if i < row.len() - 1 {
                let dim_style = Style {
                    dim: true,
                    ..Style::default()
                };
                out_spans.push(Span::Text {
                    content: "  \u{2502}  ".into(),
                    style: dim_style,
                });
            }
        }
        lines.push(Line {
            spans: out_spans,
            kind: LineKind::Table,
        });

        // Separator after header row.
        if ri == 0 {
            let mut sep_spans: Vec<Span> = Vec::new();
            let dim_style = Style {
                dim: true,
                ..Style::default()
            };
            for (i, &w) in widths.iter().enumerate() {
                sep_spans.push(Span::Text {
                    content: "\u{2500}".repeat(w),
                    style: dim_style.clone(),
                });
                if i < widths.len() - 1 {
                    sep_spans.push(Span::Text {
                        content: "  \u{253c}  ".into(),
                        style: dim_style.clone(),
                    });
                }
            }
            lines.push(Line {
                spans: sep_spans,
                kind: LineKind::Table,
            });
        }
    }
}

#[allow(dead_code)]
fn plain_width(span: &Span) -> usize {
    use unicode_width::UnicodeWidthStr;
    match span {
        Span::Text { content, .. } | Span::Link { content, .. } => {
            UnicodeWidthStr::width(content.as_str())
        }
        Span::HeadingImage { .. } => 0,
    }
}

#[allow(dead_code)]
fn spans_plain_text_inline(spans: &[Span]) -> String {
    let mut s = String::new();
    for sp in spans {
        match sp {
            Span::Text { content, .. } | Span::Link { content, .. } => s.push_str(content),
            Span::HeadingImage { .. } => {}
        }
    }
    s
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

    #[test]
    fn build_link_becomes_link_span() {
        let doc = build_plain("see [docs](https://example.com) now\n");
        let line = doc
            .lines
            .iter()
            .find(|l| matches!(l.kind, LineKind::Body))
            .unwrap();
        let link = line.spans.iter().find_map(|s| match s {
            Span::Link { content, url, .. } => Some((content.clone(), url.clone())),
            _ => None,
        });
        assert_eq!(link, Some(("docs".into(), "https://example.com".into())));
    }

    #[test]
    fn build_inline_code_has_styled_bg() {
        let doc = build_plain("run `ls` now\n");
        let line = doc
            .lines
            .iter()
            .find(|l| matches!(l.kind, LineKind::Body))
            .unwrap();
        let code = line.spans.iter().find_map(|s| match s {
            Span::Text { content, style } if content == "ls" && style.bg.is_some() => Some(()),
            _ => None,
        });
        assert!(code.is_some());
    }

    #[test]
    fn build_h1_emits_heading_line_and_entry() {
        let md = "# Title\n\nbody\n";
        let doc = build_plain(md);

        let heading_line = doc
            .lines
            .iter()
            .find(|l| matches!(l.kind, LineKind::Heading { level: 1, .. }))
            .expect("H1 line should exist");

        // Either image span (if fonts resolved) or text fallback — both are valid.
        let ok = heading_line
            .spans
            .iter()
            .any(|s| matches!(s, Span::HeadingImage { .. }))
            || heading_line.spans.iter().any(|s| {
                matches!(
                    s,
                    Span::Text { content, style } if content == "Title" && style.bold
                )
            });
        assert!(ok);

        let entry = doc.headings.iter().find(|h| h.level == 1);
        assert!(matches!(entry, Some(e) if e.text == "Title"));
    }

    #[test]
    fn build_h5_emits_text_only_line() {
        let doc = build_plain("##### tiny\n");
        let line = doc
            .lines
            .iter()
            .find(|l| matches!(l.kind, LineKind::Heading { level: 5, id: None }))
            .expect("H5 line should exist with id=None");
        let bold = line.spans.iter().any(|s| {
            matches!(
                s,
                Span::Text { content, style } if content == "tiny" && style.bold
            )
        });
        assert!(bold);
    }

    #[test]
    fn build_h1_h2_h3_get_unique_image_ids() {
        // Only meaningful when fonts are present; assert uniqueness *if* multiple images produced.
        let doc = build_plain("# A\n\n## B\n\n### C\n");
        let ids: Vec<u32> = doc.images.iter().map(|i| i.id).collect();
        let mut sorted = ids.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(ids.len(), sorted.len(), "image ids should be unique");
    }

    #[test]
    fn build_blockquote_carries_depth() {
        let doc = build_plain("> quoted\n");
        assert!(doc
            .lines
            .iter()
            .any(|l| matches!(l.kind, LineKind::BlockQuote { depth: 1 })));
    }

    #[test]
    fn build_unordered_list_item_has_depth() {
        let doc = build_plain("- a\n- b\n");
        let items: Vec<_> = doc
            .lines
            .iter()
            .filter(|l| matches!(l.kind, LineKind::ListItem { depth: 1 }))
            .collect();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn build_rule_emits_horizontal_rule_line() {
        let doc = build_plain("---\n");
        assert!(doc
            .lines
            .iter()
            .any(|l| matches!(l.kind, LineKind::HorizontalRule)));
    }

    #[test]
    fn build_code_block_emits_codeblock_lines_with_lang() {
        let doc = build_plain("```rust\nfn main() {}\n```\n");
        let has_lang = doc.lines.iter().any(|l| {
            matches!(
                &l.kind,
                LineKind::CodeBlock { lang: Some(s) } if s == "rust"
            )
        });
        assert!(has_lang);
    }

    #[test]
    fn build_table_emits_table_lines() {
        let doc = build_plain("| A | B |\n| - | - |\n| x | y |\n");
        let rows: Vec<_> = doc
            .lines
            .iter()
            .filter(|l| matches!(l.kind, LineKind::Table))
            .collect();
        // Header + separator + body = 3 lines minimum.
        assert!(rows.len() >= 3);
    }

    #[test]
    fn build_task_list_marker_replaces_bullet() {
        let doc = build_plain("- [x] done\n- [ ] todo\n");
        let items: Vec<_> = doc
            .lines
            .iter()
            .filter(|l| matches!(l.kind, LineKind::ListItem { .. }))
            .collect();
        assert_eq!(items.len(), 2);
        let first = spans_plain_text(&items[0].spans);
        let second = spans_plain_text(&items[1].spans);
        assert!(first.contains("[✓]") || first.contains("[x]"));
        assert!(second.contains("[ ]"));
    }

    #[test]
    fn build_image_renders_placeholder_text() {
        let doc = build_plain("![alt](https://example.com/x.png)\n");
        let placeholder = doc.lines.iter().any(|l| {
            spans_plain_text(&l.spans).contains("alt")
                && spans_plain_text(&l.spans).contains("https://example.com/x.png")
        });
        assert!(placeholder);
    }

    #[test]
    fn build_html_block_emits_body_line_per_source_line() {
        let doc = build_plain("<div>\n<p>x</p>\n</div>\n");
        let html_text: Vec<_> = doc
            .lines
            .iter()
            .filter_map(|l| {
                if matches!(l.kind, LineKind::Body) {
                    Some(spans_plain_text(&l.spans))
                } else {
                    None
                }
            })
            .collect();
        let joined = html_text.join("\n");
        assert!(joined.contains("<div>"));
        assert!(joined.contains("<p>x</p>"));
        assert!(joined.contains("</div>"));
    }
}
