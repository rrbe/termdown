use std::io::{self, Write};

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::config::Config;
use crate::render;
use crate::style::*;

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
) {
    if line_buf.is_empty() {
        return;
    }

    let prefix = if quote_depth > 0 {
        let bars: String = (0..quote_depth)
            .map(|_| format!("{QUOTE_BAR}\u{2502}  "))
            .collect();
        format!("{MARGIN}{bars}{QUOTE_TEXT}")
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

// ─── Main Renderer ──────────────────────────────────────────────────────────

pub fn render(text: &str, term_width: usize, config: &Config) {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);

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
                    match render::render_heading(&heading_text, heading_level, config) {
                        Some(png) => {
                            let _ = writeln!(out, "{MARGIN}{}", render::kitty_display(&png));
                        }
                        None => {
                            let _ =
                                writeln!(out, "{MARGIN}{BOLD_ON}{heading_text}{RESET}");
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
                flush_line(&mut out, &mut line_buf, quote_depth, term_width);
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
                flush_line(&mut out, &mut line_buf, quote_depth, term_width);
                in_item = false;
            }

            // ── Code blocks (buffered for uniform width) ──────────────
            Event::Start(Tag::CodeBlock(..)) => {
                block_gap(&mut out, &mut first_block);
                in_code_block = true;
                code_lines.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                let max_w = code_lines.iter().map(|l| display_width(l)).max().unwrap_or(0);
                for line in &code_lines {
                    let pad = max_w.saturating_sub(display_width(line));
                    let _ = writeln!(
                        out,
                        "{MARGIN}{CODE_BG}{CODE_FG} {line}{} {RESET}",
                        " ".repeat(pad)
                    );
                }
                in_code_block = false;
                code_lines.clear();
            }

            // ── Inline formatting ─────────────────────────────────────
            Event::Start(Tag::Strong) => {
                if !in_heading {
                    line_buf.push_str(BOLD_ON);
                }
            }
            Event::End(TagEnd::Strong) => {
                if !in_heading {
                    line_buf.push_str(RESET);
                }
            }
            Event::Start(Tag::Emphasis) => {
                if !in_heading {
                    line_buf.push_str(ITALIC_ON);
                }
            }
            Event::End(TagEnd::Emphasis) => {
                if !in_heading {
                    line_buf.push_str(ITALIC_OFF);
                }
            }
            Event::Start(Tag::Strikethrough) => {
                if !in_heading {
                    line_buf.push_str(STRIKETHROUGH_ON);
                }
            }
            Event::End(TagEnd::Strikethrough) => {
                if !in_heading {
                    line_buf.push_str(STRIKETHROUGH_OFF);
                }
            }

            // ── Links ─────────────────────────────────────────────────
            Event::Start(Tag::Link { dest_url, .. }) => {
                link_url = dest_url.to_string();
                line_buf.push_str(LINK_COLOR);
                line_buf.push_str(UNDERLINE_ON);
            }
            Event::End(TagEnd::Link) => {
                line_buf.push_str(UNDERLINE_OFF);
                line_buf.push_str(RESET);
                if !link_url.is_empty() {
                    line_buf.push_str(&format!(" {URL_COLOR}({link_url}){RESET}"));
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
            Event::Start(Tag::Image { dest_url, title, .. }) => {
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
                    line_buf.push_str(&format!("{CODE_BG}{CODE_FG} {code} {RESET}"));
                }
            }

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
                    flush_line(&mut out, &mut line_buf, quote_depth, term_width);
                    if in_item {
                        let depth = list_stack.len();
                        let indent = "  ".repeat(depth);
                        line_buf.push_str(&format!("{indent}  "));
                    }
                }
            }
            Event::HardBreak => {
                flush_line(&mut out, &mut line_buf, quote_depth, term_width);
            }
            Event::Rule => {
                block_gap(&mut out, &mut first_block);
                let width = term_width.min(62).saturating_sub(2);
                let _ = writeln!(out, "{MARGIN}{DIM_ON}{}{RESET}", "\u{2500}".repeat(width));
            }
            _ => {}
        }
    }

    flush_line(&mut out, &mut line_buf, 0, term_width);
    let _ = out.flush();
}
