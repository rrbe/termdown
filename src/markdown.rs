use std::io::{self, Write};

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::config::Config;
use crate::render;
use crate::style::{
    display_width, Colors, BOLD_ON, DIM_ON, ITALIC_OFF, ITALIC_ON, MARGIN, MARGIN_WIDTH, RESET,
    STRIKETHROUGH_OFF, STRIKETHROUGH_ON, UNDERLINE_OFF, UNDERLINE_ON,
};
use crate::theme::Theme;

// ─── Helpers ────────────────────────────────────────────────────────────────

fn level_to_u8(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

struct ListState {
    ordered: bool,
    counter: u64,
}

fn block_gap(out: &mut impl Write, first_block: &mut bool) {
    if !*first_block {
        let _ = writeln!(out);
    }
    *first_block = false;
}

// ─── Line Output ────────────────────────────────────────────────────────────

fn flush_line(
    out: &mut impl Write,
    line_buf: &mut String,
    quote_depth: usize,
    term_width: usize,
    colors: &Colors,
) {
    if line_buf.is_empty() {
        return;
    }

    let prefix = if quote_depth > 0 {
        let bars: String = (0..quote_depth)
            .map(|_| format!("{}\u{2502}  ", colors.quote_bar))
            .collect();
        format!("{MARGIN}{bars}{}", colors.quote_text)
    } else {
        MARGIN.to_string()
    };
    let suffix = if quote_depth > 0 { RESET } else { "" };

    let prefix_visual_width = MARGIN_WIDTH + quote_depth * 3;
    let max_text_width = term_width.saturating_sub(prefix_visual_width);

    if max_text_width == 0 || display_width(line_buf) <= max_text_width {
        let _ = writeln!(out, "{prefix}{line_buf}{suffix}");
    } else {
        for wrapped in wrap_text(line_buf, max_text_width) {
            let _ = writeln!(out, "{prefix}{wrapped}{suffix}");
        }
    }
    line_buf.clear();
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut current_width: usize = 0;

    for word in text.split_inclusive(' ') {
        let w = display_width(word);
        if current_width + w > max_width && !current.is_empty() {
            lines.push(current.trim_end().to_string());
            current = String::new();
            current_width = 0;
        }
        current.push_str(word);
        current_width += w;
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(text.to_string());
    }
    lines
}

// ─── Table Rendering ────────────────────────────────────────────────────────

fn render_table(out: &mut impl Write, rows: &[Vec<String>]) {
    if rows.is_empty() {
        return;
    }
    let cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut widths = vec![0usize; cols];
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(display_width(cell));
        }
    }
    for (row_idx, row) in rows.iter().enumerate() {
        let mut line = format!("{MARGIN}  ");
        for (i, cell) in row.iter().enumerate() {
            let w = widths.get(i).copied().unwrap_or(0);
            let pad = w.saturating_sub(display_width(cell));
            line.push_str(cell);
            line.push_str(&" ".repeat(pad));
            if i < row.len() - 1 {
                line.push_str(&format!("  {DIM_ON}\u{2502}{RESET}  "));
            }
        }
        let _ = writeln!(out, "{line}");
        // Separator after header row
        if row_idx == 0 {
            let mut sep = format!("{MARGIN}  ");
            for (i, &w) in widths.iter().enumerate() {
                sep.push_str(&format!("{DIM_ON}{}{RESET}", "\u{2500}".repeat(w)));
                if i < widths.len() - 1 {
                    sep.push_str(&format!("  {DIM_ON}\u{253c}{RESET}  "));
                }
            }
            let _ = writeln!(out, "{sep}");
        }
    }
}

// ─── HTML ───────────────────────────────────────────────────────────────────

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

fn inline_tag_on(name: &str, colors: &Colors) -> Option<String> {
    let n = name.to_ascii_lowercase();
    match n.as_str() {
        "b" | "strong" => Some(BOLD_ON.to_string()),
        "i" | "em" => Some(ITALIC_ON.to_string()),
        "u" => Some(UNDERLINE_ON.to_string()),
        "s" | "del" | "strike" => Some(STRIKETHROUGH_ON.to_string()),
        "code" | "kbd" => Some(format!("{}{} ", colors.code_bg, colors.code_fg)),
        _ => None,
    }
}

fn inline_tag_off(name: &str) -> Option<&'static str> {
    let n = name.to_ascii_lowercase();
    match n.as_str() {
        "b" | "strong" => Some(RESET),
        "i" | "em" => Some(ITALIC_OFF),
        "u" => Some(UNDERLINE_OFF),
        "s" | "del" | "strike" => Some(STRIKETHROUGH_OFF),
        "code" | "kbd" => Some(" \x1b[0m"),
        _ => None,
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

fn flush_html_block(out: &mut impl Write, lines: &[String]) {
    for line in lines {
        let _ = writeln!(out, "{MARGIN}{DIM_ON}{line}{RESET}");
    }
}

fn handle_inline_break(
    out: &mut impl Write,
    line_buf: &mut String,
    list_depth: usize,
    in_item: bool,
    quote_depth: usize,
    term_width: usize,
    colors: &Colors,
) {
    flush_line(out, line_buf, quote_depth, term_width, colors);
    if in_item {
        let depth = list_depth.saturating_sub(1);
        let indent = "  ".repeat(depth);
        line_buf.push_str(&format!("{indent}  "));
    }
}

fn handle_inline_rule(
    out: &mut impl Write,
    line_buf: &mut String,
    quote_depth: usize,
    term_width: usize,
    colors: &Colors,
) {
    flush_line(out, line_buf, quote_depth, term_width, colors);
    let width = term_width.min(62).saturating_sub(2);
    let _ = writeln!(out, "{MARGIN}{DIM_ON}{}{RESET}", "\u{2500}".repeat(width));
}

// ─── Main Renderer ──────────────────────────────────────────────────────────

pub fn render(text: &str, term_width: usize, config: &Config, theme: Theme, colors: &Colors) {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(text, opts);
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // ── State ──
    let mut in_heading = false;
    let mut heading_level: u8 = 0;
    let mut heading_text = String::new();
    let mut line_buf = String::new();
    let mut first_block = true;
    let mut in_item = false;
    let mut list_stack: Vec<ListState> = Vec::new();
    let mut quote_depth: usize = 0;
    let mut in_code_block = false;
    let mut code_lines: Vec<String> = Vec::new();
    let mut link_url = String::new();
    let mut image_url = String::new();
    let mut image_title = String::new();
    let mut table_row: Vec<String> = Vec::new();
    let mut table_header = false;
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut in_html_block = false;
    let mut html_block_lines: Vec<String> = Vec::new();

    for event in parser {
        match event {
            // ── Headings ──────────────────────────────────────────────
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                heading_level = level_to_u8(level);
                heading_text.clear();
            }
            Event::End(TagEnd::Heading(..)) => {
                block_gap(&mut out, &mut first_block);
                if heading_level <= 3 {
                    match render::render_heading(&heading_text, heading_level, config, theme) {
                        Some(png) => {
                            let _ = writeln!(out, "{MARGIN}{}", render::kitty_display(&png));
                        }
                        None => {
                            let _ = writeln!(out, "{MARGIN}{BOLD_ON}{heading_text}{RESET}");
                        }
                    }
                } else {
                    let _ = writeln!(out, "{MARGIN}{BOLD_ON}{heading_text}{RESET}");
                }
                in_heading = false;
            }

            // ── Paragraphs ────────────────────────────────────────────
            Event::Start(Tag::Paragraph) => {
                if quote_depth == 0 {
                    block_gap(&mut out, &mut first_block);
                }
                line_buf.clear();
            }
            Event::End(TagEnd::Paragraph) => {
                flush_line(&mut out, &mut line_buf, quote_depth, term_width, colors);
            }

            // ── Blockquotes ───────────────────────────────────────────
            Event::Start(Tag::BlockQuote(..)) => {
                if quote_depth == 0 {
                    block_gap(&mut out, &mut first_block);
                }
                quote_depth += 1;
            }
            Event::End(TagEnd::BlockQuote(..)) => {
                quote_depth = quote_depth.saturating_sub(1);
            }

            // ── Lists (ordered + unordered) ───────────────────────────
            Event::Start(Tag::List(start)) => {
                if list_stack.is_empty() {
                    block_gap(&mut out, &mut first_block);
                } else {
                    // Flush parent item text before starting nested list
                    flush_line(&mut out, &mut line_buf, quote_depth, term_width, colors);
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
                line_buf.clear();
                let depth = list_stack.len();
                let indent = "  ".repeat(depth);
                if let Some(state) = list_stack.last_mut() {
                    if state.ordered {
                        line_buf.push_str(&format!("{indent}{}. ", state.counter));
                        state.counter += 1;
                    } else {
                        line_buf.push_str(&format!("{indent}\u{2022} "));
                    }
                }
            }
            Event::End(TagEnd::Item) => {
                flush_line(&mut out, &mut line_buf, quote_depth, term_width, colors);
                in_item = false;
            }

            // ── Task list checkboxes ─────────────────────────────────
            Event::TaskListMarker(checked) => {
                // Replace the bullet "• " that was already appended by Start(Item)
                // with the checkbox marker, following glow's style: [✓] / [ ]
                let marker = if checked { "[\u{2713}] " } else { "[ ] " };
                if let Some(pos) = line_buf.rfind('\u{2022}') {
                    line_buf.replace_range(pos..pos + '\u{2022}'.len_utf8() + " ".len(), marker);
                }
            }

            // ── Code blocks (buffered for uniform width) ──────────────
            Event::Start(Tag::CodeBlock(..)) => {
                block_gap(&mut out, &mut first_block);
                in_code_block = true;
                code_lines.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                let max_w = code_lines
                    .iter()
                    .map(|l| display_width(l))
                    .max()
                    .unwrap_or(0);
                for line in &code_lines {
                    let pad = max_w.saturating_sub(display_width(line));
                    let _ = writeln!(
                        out,
                        "{MARGIN}{}{} {line}{} {RESET}",
                        colors.code_bg,
                        colors.code_fg,
                        " ".repeat(pad)
                    );
                }
                in_code_block = false;
                code_lines.clear();
            }

            // ── HTML block ───────────────────────────────────────────
            Event::Start(Tag::HtmlBlock) => {
                block_gap(&mut out, &mut first_block);
                in_html_block = true;
                html_block_lines.clear();
            }
            Event::End(TagEnd::HtmlBlock) => {
                let joined = html_block_lines.join("\n");
                let stripped = strip_html_comments(&joined);
                let lines: Vec<String> = stripped
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .map(|l| l.to_string())
                    .collect();
                flush_html_block(&mut out, &lines);
                in_html_block = false;
                html_block_lines.clear();
            }
            Event::Html(s) if in_html_block => {
                for line in s.lines() {
                    html_block_lines.push(line.to_string());
                }
            }

            // ── Inline formatting ─────────────────────────────────────
            Event::Start(Tag::Strong) if !in_heading => {
                line_buf.push_str(BOLD_ON);
            }
            Event::End(TagEnd::Strong) if !in_heading => {
                line_buf.push_str(RESET);
            }
            Event::Start(Tag::Emphasis) if !in_heading => {
                line_buf.push_str(ITALIC_ON);
            }
            Event::End(TagEnd::Emphasis) if !in_heading => {
                line_buf.push_str(ITALIC_OFF);
            }
            Event::Start(Tag::Strikethrough) if !in_heading => {
                line_buf.push_str(STRIKETHROUGH_ON);
            }
            Event::End(TagEnd::Strikethrough) if !in_heading => {
                line_buf.push_str(STRIKETHROUGH_OFF);
            }

            // ── Links ─────────────────────────────────────────────────
            Event::Start(Tag::Link { dest_url, .. }) => {
                link_url = dest_url.to_string();
                line_buf.push_str(colors.link);
                line_buf.push_str(UNDERLINE_ON);
            }
            Event::End(TagEnd::Link) => {
                line_buf.push_str(UNDERLINE_OFF);
                line_buf.push_str(RESET);
                if !link_url.is_empty() {
                    line_buf.push_str(&format!(" {}({link_url}){RESET}", colors.url));
                }
                link_url.clear();
            }

            // ── Tables ────────────────────────────────────────────────
            Event::Start(Tag::Table(..)) => {
                block_gap(&mut out, &mut first_block);
                table_rows.clear();
                table_header = false;
            }
            Event::End(TagEnd::Table) => {
                render_table(&mut out, &table_rows);
            }
            Event::Start(Tag::TableHead) => {
                table_header = true;
                table_row.clear();
            }
            Event::End(TagEnd::TableHead) => {
                table_rows.push(table_row.clone());
                table_row.clear();
                table_header = false;
            }
            Event::Start(Tag::TableRow) => {
                table_row.clear();
            }
            Event::End(TagEnd::TableRow) => {
                table_rows.push(table_row.clone());
                table_row.clear();
            }
            Event::Start(Tag::TableCell) => {
                line_buf.clear();
                if table_header {
                    line_buf.push_str(BOLD_ON);
                }
            }
            Event::End(TagEnd::TableCell) => {
                if table_header {
                    line_buf.push_str(RESET);
                }
                table_row.push(line_buf.clone());
                line_buf.clear();
            }

            // ── Images ────────────────────────────────────────────────
            Event::Start(Tag::Image {
                dest_url, title, ..
            }) => {
                image_url = dest_url.to_string();
                image_title = title.to_string();
            }
            Event::End(TagEnd::Image) => {
                let alt = if !line_buf.is_empty() {
                    line_buf.clone()
                } else {
                    image_title.clone()
                };
                let _ = writeln!(out, "{MARGIN}{DIM_ON}[\u{1f5bc} {alt}]({image_url}){RESET}");
                line_buf.clear();
                image_url.clear();
                image_title.clear();
            }

            // ── Inline code ───────────────────────────────────────────
            Event::Code(code) => {
                if in_heading {
                    heading_text.push_str(&code);
                } else {
                    line_buf.push_str(&format!(
                        "{}{} {code} {RESET}",
                        colors.code_bg, colors.code_fg
                    ));
                }
            }

            // ── Inline HTML ───────────────────────────────────────────
            Event::InlineHtml(s) => match parse_html_fragment(&s) {
                HtmlFragment::Comment | HtmlFragment::Other => {}
                HtmlFragment::SelfClose { name } => {
                    if name.eq_ignore_ascii_case("br") {
                        handle_inline_break(
                            &mut out,
                            &mut line_buf,
                            list_stack.len(),
                            in_item,
                            quote_depth,
                            term_width,
                            colors,
                        );
                    } else if name.eq_ignore_ascii_case("hr") {
                        handle_inline_rule(
                            &mut out,
                            &mut line_buf,
                            quote_depth,
                            term_width,
                            colors,
                        );
                    }
                }
                HtmlFragment::Open { name } => {
                    if !in_heading {
                        if let Some(on) = inline_tag_on(name, colors) {
                            line_buf.push_str(&on);
                        }
                    }
                }
                HtmlFragment::Close { name } => {
                    if !in_heading {
                        if let Some(off) = inline_tag_off(name) {
                            line_buf.push_str(off);
                        }
                    }
                }
            },

            // ── Text ──────────────────────────────────────────────────
            Event::Text(t) => {
                if in_heading {
                    heading_text.push_str(&t);
                } else if in_code_block {
                    for line in t.lines() {
                        code_lines.push(line.to_string());
                    }
                } else {
                    line_buf.push_str(&t);
                }
            }

            // ── Breaks ────────────────────────────────────────────────
            Event::SoftBreak => {
                if in_heading {
                    heading_text.push(' ');
                } else if !in_code_block {
                    flush_line(&mut out, &mut line_buf, quote_depth, term_width, colors);
                    if in_item {
                        let depth = list_stack.len();
                        let indent = "  ".repeat(depth);
                        line_buf.push_str(&format!("{indent}  "));
                    }
                }
            }
            Event::HardBreak => {
                flush_line(&mut out, &mut line_buf, quote_depth, term_width, colors);
            }
            Event::Rule => {
                block_gap(&mut out, &mut first_block);
                let width = term_width.min(62).saturating_sub(2);
                let _ = writeln!(out, "{MARGIN}{DIM_ON}{}{RESET}", "\u{2500}".repeat(width));
            }
            _ => {}
        }
    }

    flush_line(&mut out, &mut line_buf, 0, term_width, colors);
    let _ = out.flush();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::strip_ansi;

    #[test]
    fn wrap_text_keeps_single_overlong_word_intact() {
        assert_eq!(
            wrap_text("supercalifragilistic", 5),
            vec!["supercalifragilistic"]
        );
    }

    #[test]
    fn wrap_text_uses_display_width_when_ansi_and_wide_chars_are_present() {
        let text = format!("{BOLD_ON}你好{RESET} world");
        let lines = wrap_text(&text, 6);

        assert_eq!(lines.len(), 2);
        assert_eq!(strip_ansi(&lines[0]), "你好");
        assert_eq!(strip_ansi(&lines[1]), "world");
        assert!(lines.iter().all(|line| display_width(line) <= 6));
    }

    #[test]
    fn flush_line_wraps_quoted_content_and_clears_buffer() {
        let dark = Colors::for_theme(crate::theme::Theme::Dark);
        let mut out = Vec::new();
        let mut line = String::from("alpha beta gamma");

        flush_line(&mut out, &mut line, 1, 12, &dark);

        let prefix = format!("{MARGIN}{}\u{2502}  {}", dark.quote_bar, dark.quote_text);
        let expected = format!("{prefix}alpha{RESET}\n{prefix}beta{RESET}\n{prefix}gamma{RESET}\n");

        assert_eq!(String::from_utf8(out).unwrap(), expected);
        assert!(line.is_empty());
    }

    #[test]
    fn parse_html_fragment_recognizes_every_shape() {
        assert!(matches!(
            parse_html_fragment("<!-- hi -->"),
            HtmlFragment::Comment
        ));
        assert!(matches!(
            parse_html_fragment("<?xml?>"),
            HtmlFragment::Other
        ));
        assert!(matches!(
            parse_html_fragment("<!DOCTYPE html>"),
            HtmlFragment::Other
        ));
        assert!(matches!(
            parse_html_fragment("<br/>"),
            HtmlFragment::SelfClose { name } if name == "br"
        ));
        assert!(matches!(
            parse_html_fragment("<br />"),
            HtmlFragment::SelfClose { name } if name == "br"
        ));
        assert!(matches!(
            parse_html_fragment("<b>"),
            HtmlFragment::Open { name } if name == "b"
        ));
        assert!(matches!(
            parse_html_fragment("<span style=\"color:red\">"),
            HtmlFragment::Open { name } if name == "span"
        ));
        assert!(matches!(
            parse_html_fragment("</STRONG>"),
            HtmlFragment::Close { name } if name.eq_ignore_ascii_case("strong")
        ));
        assert!(matches!(
            parse_html_fragment("not a tag"),
            HtmlFragment::Other
        ));
    }

    #[test]
    fn inline_tag_on_off_maps_known_format_tags() {
        let colors = Colors::for_theme(crate::theme::Theme::Dark);
        assert_eq!(inline_tag_on("b", &colors).unwrap(), BOLD_ON);
        assert_eq!(inline_tag_on("STRONG", &colors).unwrap(), BOLD_ON);
        assert_eq!(inline_tag_on("i", &colors).unwrap(), ITALIC_ON);
        assert_eq!(inline_tag_on("u", &colors).unwrap(), UNDERLINE_ON);
        assert_eq!(inline_tag_on("s", &colors).unwrap(), STRIKETHROUGH_ON);
        assert_eq!(inline_tag_on("del", &colors).unwrap(), STRIKETHROUGH_ON);
        assert!(inline_tag_on("code", &colors)
            .unwrap()
            .contains(colors.code_bg));
        assert!(inline_tag_on("span", &colors).is_none());

        assert_eq!(inline_tag_off("strong"), Some(RESET));
        assert_eq!(inline_tag_off("em"), Some(ITALIC_OFF));
        assert_eq!(inline_tag_off("u"), Some(UNDERLINE_OFF));
        assert_eq!(inline_tag_off("strike"), Some(STRIKETHROUGH_OFF));
        assert_eq!(inline_tag_off("code"), Some(" \x1b[0m"));
        assert!(inline_tag_off("div").is_none());
    }

    #[test]
    fn strip_html_comments_handles_inline_and_multiline() {
        assert_eq!(
            strip_html_comments("a <!-- x --> b <!-- y --> c"),
            "a  b  c"
        );
        assert_eq!(
            strip_html_comments("pre\n<!-- block\ncomment -->\npost"),
            "pre\n\npost"
        );
        assert_eq!(strip_html_comments("head <!-- unterminated"), "head ");
        assert_eq!(strip_html_comments("no comments here"), "no comments here");
    }

    #[test]
    fn flush_html_block_prefixes_margin_and_dims_lines() {
        let mut out = Vec::new();
        let lines = vec![
            String::from("<div>"),
            String::from("  text"),
            String::from("</div>"),
        ];
        flush_html_block(&mut out, &lines);
        let got = String::from_utf8(out).unwrap();
        assert_eq!(
            got,
            format!(
                "{MARGIN}{DIM_ON}<div>{RESET}\n\
                 {MARGIN}{DIM_ON}  text{RESET}\n\
                 {MARGIN}{DIM_ON}</div>{RESET}\n"
            )
        );
    }

    #[test]
    fn handle_inline_break_flushes_and_indents_for_list_item() {
        let colors = Colors::for_theme(crate::theme::Theme::Dark);
        let mut out = Vec::new();
        let mut line = String::from("line one");
        handle_inline_break(&mut out, &mut line, 2, true, 0, 80, &colors);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            format!("{MARGIN}line one\n")
        );
        // Nested (depth 2) → indent = "  " * (2-1) + "  " = "    "
        assert_eq!(line, "    ");
    }

    #[test]
    fn render_table_aligns_columns_using_visual_width() {
        let mut out = Vec::new();
        let rows = vec![
            vec![format!("{BOLD_ON}标题{RESET}"), String::from("B")],
            vec![String::from("x"), String::from("long")],
        ];

        render_table(&mut out, &rows);

        let rendered = String::from_utf8(out).unwrap();
        let lines: Vec<_> = rendered.lines().collect();

        assert_eq!(lines.len(), 3);
        assert_eq!(
            lines[0],
            format!("{MARGIN}  {BOLD_ON}标题{RESET}  {DIM_ON}\u{2502}{RESET}  B   ")
        );
        assert_eq!(
            lines[1],
            format!(
                "{MARGIN}  {DIM_ON}\u{2500}\u{2500}\u{2500}\u{2500}{RESET}  {DIM_ON}\u{253c}{RESET}  {DIM_ON}\u{2500}\u{2500}\u{2500}\u{2500}{RESET}"
            )
        );
        assert_eq!(
            lines[2],
            format!("{MARGIN}  x     {DIM_ON}\u{2502}{RESET}  long")
        );
        assert_eq!(display_width(lines[0]), display_width(lines[2]));
    }
}
