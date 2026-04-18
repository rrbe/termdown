use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use crate::config::Config;
use crate::render::HeadingImage;
use crate::theme::Theme;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedDoc {
    pub lines: Vec<Line>,
    pub headings: Vec<HeadingEntry>,
    pub images: Vec<HeadingImage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub spans: Vec<Span>,
    pub kind: LineKind,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// 256-color index (what the existing style.rs already emits).
    Indexed(u8),
    /// Truecolor fallback for future use.
    #[allow(dead_code)] // Reserved for TUI pipeline.
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingEntry {
    pub level: u8,
    pub text: String,
    pub line_index: usize,
}

struct ListState {
    ordered: bool,
    counter: u64,
}

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
    let mut in_html_block = false;
    let mut html_block_lines: Vec<String> = Vec::new();
    let mut in_item = false;

    // Helper to decide whether a blank line is needed before the next block.
    // Returns true if a blank separator should be emitted.
    fn push_block_gap(lines: &mut Vec<Line>) {
        if !lines.is_empty() && !matches!(lines.last().map(|l| &l.kind), Some(LineKind::Blank)) {
            lines.push(Line {
                spans: vec![],
                kind: LineKind::Blank,
            });
        }
    }

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                push_block_gap(&mut lines);
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
            Event::Start(Tag::Paragraph) => {
                if quote_depth == 0 && !in_item {
                    push_block_gap(&mut lines);
                }
            }
            Event::End(TagEnd::Paragraph) => {
                flush_text(&mut text_buf, &mut spans, &style);
                let kind = if quote_depth > 0 {
                    LineKind::BlockQuote { depth: quote_depth }
                } else if in_item {
                    let depth = list_stack.len() as u8;
                    LineKind::ListItem { depth }
                } else {
                    LineKind::Body
                };
                if !spans.is_empty() {
                    lines.push(Line {
                        spans: std::mem::take(&mut spans),
                        kind,
                    });
                }
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
                // Mirror the legacy renderer, which padded inline code with one
                // space on each side so the colored background isn't flush against
                // surrounding text.
                spans.push(Span::Text {
                    content: format!(" {code} "),
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
            Event::Start(Tag::BlockQuote(..)) => {
                if quote_depth == 0 {
                    push_block_gap(&mut lines);
                }
                quote_depth += 1;
            }
            Event::End(TagEnd::BlockQuote(..)) => quote_depth = quote_depth.saturating_sub(1),

            Event::Start(Tag::List(start)) => {
                if list_stack.is_empty() && !in_item {
                    push_block_gap(&mut lines);
                } else if in_item {
                    // Nested list: flush the current parent-item prefix+content
                    // as its own ListItem line before emitting the sublist.
                    flush_text(&mut text_buf, &mut spans, &style);
                    if !spans.is_empty() {
                        let depth = list_stack.len() as u8;
                        lines.push(Line {
                            spans: std::mem::take(&mut spans),
                            kind: LineKind::ListItem { depth },
                        });
                    }
                }
                list_stack.push(ListState {
                    ordered: start.is_some(),
                    counter: start.unwrap_or(1),
                });
            }
            Event::End(TagEnd::List(..)) => {
                list_stack.pop();
            }

            Event::Start(Tag::Item) => {
                in_item = true;
                // Reset the per-item buffer and seed it with the marker that this
                // item needs (bullet or number). Indentation is baked in so
                // cat.rs only needs to append a margin.
                spans.clear();
                text_buf.clear();
                let depth = list_stack.len();
                let indent = "  ".repeat(depth);
                if let Some(state) = list_stack.last_mut() {
                    if state.ordered {
                        text_buf.push_str(&format!("{indent}{}. ", state.counter));
                        state.counter += 1;
                    } else {
                        text_buf.push_str(&format!("{indent}\u{2022} "));
                    }
                }
            }
            Event::End(TagEnd::Item) => {
                flush_text(&mut text_buf, &mut spans, &style);
                let depth = list_stack.len() as u8;
                // If `spans` only contains the bullet/number marker we originally
                // seeded, that means the item was entirely consumed by a nested
                // list (which already emitted its own parent line). Skip the
                // phantom empty line in that case.
                let only_marker = spans.len() == 1
                    && matches!(
                        &spans[0],
                        Span::Text { content, .. } if content.trim_end().ends_with('.')
                            || content.trim_end().ends_with('\u{2022}')
                    );
                if !spans.is_empty() && !only_marker {
                    lines.push(Line {
                        spans: std::mem::take(&mut spans),
                        kind: LineKind::ListItem { depth },
                    });
                } else {
                    spans.clear();
                }
                in_item = false;
            }

            Event::Start(Tag::CodeBlock(kind)) => {
                push_block_gap(&mut lines);
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
                push_block_gap(&mut lines);
                lines.push(Line {
                    spans: vec![],
                    kind: LineKind::HorizontalRule,
                });
            }

            // Tables
            Event::Start(Tag::Table(..)) => {
                push_block_gap(&mut lines);
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
                text_buf.clear();
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

            // Task list marker: replace the bullet that Start(Item) seeded with
            // the task-box marker, following glow's style.
            Event::TaskListMarker(checked) => {
                let marker = if checked { "[\u{2713}] " } else { "[ ] " };
                if let Some(pos) = text_buf.rfind('\u{2022}') {
                    let end = pos + '\u{2022}'.len_utf8() + " ".len();
                    text_buf.replace_range(pos..end, marker);
                }
            }

            // HTML block — buffer lines, strip comments, then emit each non-empty
            // line as a dim Body line.
            Event::Start(Tag::HtmlBlock) => {
                push_block_gap(&mut lines);
                in_html_block = true;
                html_block_lines.clear();
            }
            Event::End(TagEnd::HtmlBlock) => {
                let joined = html_block_lines.join("\n");
                let stripped = strip_html_comments(&joined);
                let dim_style = Style {
                    dim: true,
                    ..Style::default()
                };
                for line in stripped.lines().filter(|l| !l.trim().is_empty()) {
                    lines.push(Line {
                        spans: vec![Span::Text {
                            content: line.to_string(),
                            style: dim_style.clone(),
                        }],
                        kind: LineKind::Body,
                    });
                }
                in_html_block = false;
                html_block_lines.clear();
            }
            Event::Html(s) => {
                if in_html_block {
                    for line in s.lines() {
                        html_block_lines.push(line.to_string());
                    }
                } else {
                    // Orphan block-ish HTML outside an HtmlBlock tag pair.
                    // Treat like the legacy renderer and emit dim Body lines.
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
            }

            // Inline HTML — interpret the common formatting tags (<b>, <i>, <u>,
            // <s>/<del>/<strike>, <code>/<kbd>), handle `<br/>` / `<hr/>`, and
            // drop everything else (comments, unknown tags, DOCTYPE, <?xml?>…).
            Event::InlineHtml(s) => match parse_html_fragment(&s) {
                HtmlFragment::Comment | HtmlFragment::Other => {}
                HtmlFragment::SelfClose { name } => {
                    if name.eq_ignore_ascii_case("br") {
                        // Flush what we have as its own line in the current block.
                        flush_text(&mut text_buf, &mut spans, &style);
                        let kind = if quote_depth > 0 {
                            LineKind::BlockQuote { depth: quote_depth }
                        } else if in_item {
                            let depth = list_stack.len() as u8;
                            LineKind::ListItem { depth }
                        } else {
                            LineKind::Body
                        };
                        if !spans.is_empty() {
                            lines.push(Line {
                                spans: std::mem::take(&mut spans),
                                kind,
                            });
                        }
                    } else if name.eq_ignore_ascii_case("hr") {
                        flush_text(&mut text_buf, &mut spans, &style);
                        if !spans.is_empty() {
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
                        lines.push(Line {
                            spans: vec![],
                            kind: LineKind::HorizontalRule,
                        });
                    }
                }
                HtmlFragment::Open { name } => {
                    if heading_level == 0 {
                        apply_inline_tag_on(&mut text_buf, &mut spans, &mut style, name);
                    }
                }
                HtmlFragment::Close { name } => {
                    if heading_level == 0 {
                        apply_inline_tag_off(&mut text_buf, &mut spans, &mut style, name);
                    }
                }
            },

            // Breaks — a SoftBreak or HardBreak inside a paragraph flushes the
            // accumulated line as a new Body/Quote/ListItem line (mirrors the
            // legacy `flush_line` behavior).
            Event::SoftBreak | Event::HardBreak => {
                if heading_level > 0 {
                    heading_text.push(' ');
                } else if in_code_block.is_some() {
                    // no-op — the Text event already split on newlines.
                } else {
                    flush_text(&mut text_buf, &mut spans, &style);
                    let kind = if quote_depth > 0 {
                        LineKind::BlockQuote { depth: quote_depth }
                    } else if in_item {
                        let depth = list_stack.len() as u8;
                        LineKind::ListItem { depth }
                    } else {
                        LineKind::Body
                    };
                    if !spans.is_empty() {
                        lines.push(Line {
                            spans: std::mem::take(&mut spans),
                            kind,
                        });
                    }
                    // If we're inside a list item, indent the continuation so
                    // the next line lines up under the content column (after
                    // the bullet/number marker).
                    if in_item {
                        let depth = list_stack.len();
                        let indent = "  ".repeat(depth);
                        text_buf.push_str(&format!("{indent}  "));
                    }
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

// ─── Inline HTML helpers ────────────────────────────────────────────────────

enum HtmlFragment<'a> {
    Comment,
    Open { name: &'a str },
    Close { name: &'a str },
    SelfClose { name: &'a str },
    Other,
}

fn html_tag_name(s: &str) -> &str {
    let end = s
        .find(|c: char| c.is_whitespace() || c == '/' || c == '>')
        .unwrap_or(s.len());
    &s[..end]
}

fn parse_html_fragment(s: &str) -> HtmlFragment<'_> {
    let s = s.trim();
    if !s.starts_with('<') || !s.ends_with('>') {
        return HtmlFragment::Other;
    }
    if s.starts_with("<!--") {
        return HtmlFragment::Comment;
    }
    if s.starts_with("<!") || s.starts_with("<?") {
        return HtmlFragment::Other;
    }
    if let Some(inner) = s.strip_prefix("</").and_then(|v| v.strip_suffix('>')) {
        let name = html_tag_name(inner);
        if name.is_empty() {
            return HtmlFragment::Other;
        }
        return HtmlFragment::Close { name };
    }
    let inner = &s[1..s.len() - 1];
    let (inner, self_close) = match inner.strip_suffix('/') {
        Some(i) => (i.trim_end(), true),
        None => (inner, false),
    };
    let name = html_tag_name(inner);
    if name.is_empty() {
        return HtmlFragment::Other;
    }
    if self_close {
        HtmlFragment::SelfClose { name }
    } else {
        HtmlFragment::Open { name }
    }
}

/// Apply an `<open>` inline-HTML tag, converting known formatting tags to a
/// style change on the running `style`. Unknown tags are silently dropped —
/// matching the legacy renderer's behavior of stripping unsupported HTML.
fn apply_inline_tag_on(
    text_buf: &mut String,
    spans: &mut Vec<Span>,
    style: &mut Style,
    name: &str,
) {
    let n = name.to_ascii_lowercase();
    match n.as_str() {
        "b" | "strong" => {
            flush_text(text_buf, spans, style);
            style.bold = true;
        }
        "i" | "em" => {
            flush_text(text_buf, spans, style);
            style.italic = true;
        }
        "u" => {
            flush_text(text_buf, spans, style);
            style.underline = true;
        }
        "s" | "del" | "strike" => {
            flush_text(text_buf, spans, style);
            style.strikethrough = true;
        }
        "code" | "kbd" => {
            flush_text(text_buf, spans, style);
            // Emit a zero-length styled span marker by pushing an empty span
            // with the code style; subsequent Text events accumulate into
            // text_buf under the code style.
            *style = Style {
                bg: Some(Color::Indexed(236)),
                fg: Some(Color::Indexed(213)),
                ..style.clone()
            };
            // Add opening padding space (matches legacy "{code_bg}{code_fg} ...")
            text_buf.push(' ');
        }
        _ => {}
    }
}

fn apply_inline_tag_off(
    text_buf: &mut String,
    spans: &mut Vec<Span>,
    style: &mut Style,
    name: &str,
) {
    let n = name.to_ascii_lowercase();
    match n.as_str() {
        "b" | "strong" => {
            flush_text(text_buf, spans, style);
            style.bold = false;
        }
        "i" | "em" => {
            flush_text(text_buf, spans, style);
            style.italic = false;
        }
        "u" => {
            flush_text(text_buf, spans, style);
            style.underline = false;
        }
        "s" | "del" | "strike" => {
            flush_text(text_buf, spans, style);
            style.strikethrough = false;
        }
        "code" | "kbd" => {
            // Trailing space closes the padded code region.
            text_buf.push(' ');
            flush_text(text_buf, spans, style);
            style.bg = None;
            style.fg = None;
        }
        _ => {}
    }
}

/// Remove `<!-- ... -->` spans from `s`, including spans that cross newlines.
/// Unterminated comments drop the tail of the input.
fn strip_html_comments(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(start) = rest.find("<!--") {
        out.push_str(&rest[..start]);
        match rest[start + 4..].find("-->") {
            Some(end) => rest = &rest[start + 4 + end + 3..],
            None => {
                rest = "";
                break;
            }
        }
    }
    out.push_str(rest);
    out
}

/// Render accumulated table rows into `LineKind::Table` lines with padding and separators.
/// Keeps the margin-less column layout the existing cat mode produces — the outer
/// "    " margin is added by `cat.rs`.
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

fn plain_width(span: &Span) -> usize {
    match span {
        Span::Text { content, .. } | Span::Link { content, .. } => {
            crate::style::display_width(content)
        }
        Span::HeadingImage { .. } => 0,
    }
}

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
            // Layout pads inline code with a space on each side so the
            // colored background isn't flush against neighboring text.
            Span::Text { content, style } if content.trim() == "ls" && style.bg.is_some() => {
                Some(())
            }
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
