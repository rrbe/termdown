//! Stream a `RenderedDoc` to stdout as ANSI text, matching the existing
//! cat-mode visual output. Wrapping, margins, quote prefixes, list
//! indentation, and Kitty heading image emission all happen here.

use std::io::{BufWriter, Write};

use crate::layout::{Color, Line, LineKind, RenderedDoc, Span, Style};
use crate::render;
use crate::style::{
    display_width, Colors, BOLD_ON, DIM_ON, ITALIC_OFF, ITALIC_ON, MARGIN, MARGIN_WIDTH, RESET,
    STRIKETHROUGH_OFF, STRIKETHROUGH_ON, UNDERLINE_OFF, UNDERLINE_ON,
};

pub fn print(doc: &RenderedDoc, term_width: usize, colors: &Colors) {
    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut i = 0;
    while i < doc.lines.len() {
        let line = &doc.lines[i];
        // Group consecutive code-block lines so the colored background pads to
        // a uniform width (matches the legacy renderer, which buffered the
        // entire fenced block before emitting it).
        if matches!(line.kind, LineKind::CodeBlock { .. }) {
            let mut end = i;
            while end < doc.lines.len() && matches!(doc.lines[end].kind, LineKind::CodeBlock { .. })
            {
                end += 1;
            }
            emit_code_block(&mut out, &doc.lines[i..end], colors);
            i = end;
            continue;
        }

        write_line(&mut out, line, &doc.images, term_width, colors);
        i += 1;
    }
    let _ = out.flush();
}

fn write_line<W: Write>(
    out: &mut W,
    line: &Line,
    images: &[crate::render::HeadingImage],
    term_width: usize,
    colors: &Colors,
) {
    match &line.kind {
        LineKind::Blank => {
            let _ = writeln!(out);
        }
        LineKind::HorizontalRule => {
            let width = term_width.min(62).saturating_sub(2);
            let _ = writeln!(out, "{MARGIN}{DIM_ON}{}{RESET}", "\u{2500}".repeat(width));
        }
        LineKind::Heading { id, .. } => {
            if let Some(image_id) = id {
                if let Some(img) = images.iter().find(|i| i.id == *image_id) {
                    let _ = writeln!(out, "{MARGIN}{}", render::kitty_display(&img.png));
                    return;
                }
            }
            let text = render_spans_plain(&line.spans);
            let _ = writeln!(out, "{MARGIN}{BOLD_ON}{text}{RESET}");
        }
        LineKind::BlockQuote { depth } => {
            write_paragraph(out, &line.spans, *depth as usize, term_width, colors);
        }
        LineKind::Body => {
            write_paragraph(out, &line.spans, 0, term_width, colors);
        }
        LineKind::ListItem { .. } => {
            // Layout has already baked the per-depth indent and the bullet or
            // numbered marker into the first text span, so cat only needs to
            // prepend the outer margin.
            let body = render_spans_ansi(&line.spans, colors);
            let buf = format!("{MARGIN}{body}");
            wrap_and_write(out, &buf, term_width, "");
        }
        LineKind::CodeBlock { .. } => {
            // Single-line code blocks are handled via emit_code_block; this
            // branch is unreachable in practice because `print` batches them.
            let text = render_spans_plain(&line.spans);
            let _ = writeln!(
                out,
                "{MARGIN}{}{} {text} {RESET}",
                colors.code_bg, colors.code_fg
            );
        }
        LineKind::Table => {
            let rendered = render_spans_ansi(&line.spans, colors);
            let _ = writeln!(out, "{MARGIN}  {rendered}");
        }
    }
}

/// Emit a consecutive run of `LineKind::CodeBlock` lines, padding each to the
/// longest line in the group so the background renders as a clean rectangle.
fn emit_code_block<W: Write>(out: &mut W, group: &[Line], colors: &Colors) {
    let texts: Vec<String> = group.iter().map(|l| render_spans_plain(&l.spans)).collect();
    let max_w = texts.iter().map(|t| display_width(t)).max().unwrap_or(0);
    for text in &texts {
        let pad = max_w.saturating_sub(display_width(text));
        let _ = writeln!(
            out,
            "{MARGIN}{}{} {text}{} {RESET}",
            colors.code_bg,
            colors.code_fg,
            " ".repeat(pad)
        );
    }
}

fn write_paragraph<W: Write>(
    out: &mut W,
    spans: &[Span],
    quote_depth: usize,
    term_width: usize,
    colors: &Colors,
) {
    let body = render_spans_ansi(spans, colors);
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

    if max_text_width == 0 || display_width(&body) <= max_text_width {
        let _ = writeln!(out, "{prefix}{body}{suffix}");
    } else {
        for wrapped in wrap_text(&body, max_text_width) {
            let _ = writeln!(out, "{prefix}{wrapped}{suffix}");
        }
    }
}

fn wrap_and_write<W: Write>(out: &mut W, text: &str, term_width: usize, suffix: &str) {
    let max = term_width.saturating_sub(MARGIN_WIDTH);
    if max == 0 || display_width(text) <= max {
        let _ = writeln!(out, "{text}{suffix}");
        return;
    }
    for wrapped in wrap_text(text, max) {
        let _ = writeln!(out, "{wrapped}{suffix}");
    }
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

fn render_spans_plain(spans: &[Span]) -> String {
    let mut s = String::new();
    for sp in spans {
        match sp {
            Span::Text { content, .. } | Span::Link { content, .. } => s.push_str(content),
            Span::HeadingImage { .. } => {}
        }
    }
    s
}

fn render_spans_ansi(spans: &[Span], colors: &Colors) -> String {
    let mut out = String::new();
    for sp in spans {
        match sp {
            Span::Text { content, style } => {
                push_style_on(&mut out, style);
                out.push_str(content);
                push_style_off(&mut out, style);
            }
            Span::Link {
                content,
                url,
                style,
            } => {
                out.push_str(colors.link);
                out.push_str(UNDERLINE_ON);
                push_style_on(&mut out, style);
                out.push_str(content);
                push_style_off(&mut out, style);
                out.push_str(UNDERLINE_OFF);
                out.push_str(RESET);
                if !url.is_empty() {
                    out.push_str(&format!(" {}({url}){RESET}", colors.url));
                }
            }
            Span::HeadingImage { .. } => {}
        }
    }
    out
}

fn push_style_on(out: &mut String, style: &Style) {
    if style.bold {
        out.push_str(BOLD_ON);
    }
    if style.italic {
        out.push_str(ITALIC_ON);
    }
    if style.underline {
        out.push_str(UNDERLINE_ON);
    }
    if style.strikethrough {
        out.push_str(STRIKETHROUGH_ON);
    }
    if style.dim {
        out.push_str(DIM_ON);
    }
    if let Some(fg) = &style.fg {
        out.push_str(&color_fg(*fg));
    }
    if let Some(bg) = &style.bg {
        out.push_str(&color_bg(*bg));
    }
}

fn push_style_off(out: &mut String, style: &Style) {
    // Use RESET when any attribute that can't be cleanly "turned off" was set.
    if style.fg.is_some() || style.bg.is_some() || style.bold || style.dim {
        out.push_str(RESET);
    } else {
        if style.italic {
            out.push_str(ITALIC_OFF);
        }
        if style.underline {
            out.push_str(UNDERLINE_OFF);
        }
        if style.strikethrough {
            out.push_str(STRIKETHROUGH_OFF);
        }
    }
}

fn color_fg(c: Color) -> String {
    match c {
        Color::Indexed(n) => format!("\x1b[38;5;{n}m"),
        Color::Rgb(r, g, b) => format!("\x1b[38;2;{r};{g};{b}m"),
    }
}

fn color_bg(c: Color) -> String {
    match c {
        Color::Indexed(n) => format!("\x1b[48;5;{n}m"),
        Color::Rgb(r, g, b) => format!("\x1b[48;2;{r};{g};{b}m"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{display_width, strip_ansi, BOLD_ON, RESET};

    // ── wrap_text tests (migrated from markdown.rs) ──────────────────────────

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

    // ── write_paragraph wrapping (adapted from markdown::flush_line) ─────────
    // The old `flush_line` took a `&mut String` directly; the new equivalent is
    // `write_paragraph` which takes a `&[Span]`.  We test the same invariant:
    // quoted content that overflows is word-wrapped and the buffer is consumed.

    #[test]
    fn write_paragraph_wraps_quoted_content() {
        use crate::layout::{Span, Style};
        use crate::style::{Colors, MARGIN};
        use crate::theme::Theme;

        let colors = Colors::for_theme(Theme::Dark);
        let mut out: Vec<u8> = Vec::new();

        let spans = vec![Span::Text {
            content: "alpha beta gamma".into(),
            style: Style::default(),
        }];

        // width=12, quote_depth=1 → prefix width = MARGIN_WIDTH(2) + 1*3 = 5,
        // so max_text_width = 12 - 5 = 7. "alpha" fits (5), "beta" fits (4 → 9
        // > 7 alone? No: 5+1+4=10 > 7), so lines: "alpha", "beta", "gamma".
        write_paragraph(&mut out, &spans, 1, 12, &colors);

        let got = String::from_utf8(out).unwrap();
        let prefix = format!(
            "{MARGIN}{}\u{2502}  {}",
            colors.quote_bar, colors.quote_text
        );
        // Each wrapped word should appear on its own prefixed line.
        assert!(got.contains(&format!("{prefix}alpha{RESET}")));
        assert!(got.contains(&format!("{prefix}beta{RESET}")));
        assert!(got.contains(&format!("{prefix}gamma{RESET}")));
    }
}
