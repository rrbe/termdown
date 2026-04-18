//! Scroll state + wrap cache for the TUI body.
//!
//! v1 wrap is a no-op (one visual line per logical line). Task 4.4 replaces
//! `wrap_all` with a width-aware breaker.

use crate::layout::{Line, RenderedDoc, Span};

/// A wrapped visual line, pointing back to a logical `Line` and the byte
/// range of its content that this visual slice covers.
#[derive(Debug, Clone)]
pub struct VisualLine {
    pub logical_index: usize,
    pub byte_start: usize,
    pub byte_end: usize,
}

pub struct Viewport {
    pub top: usize,
    pub height: u16,
    pub width: u16,
    visual_lines: Vec<VisualLine>,
    cache_width: u16,
}

impl Viewport {
    pub fn new(height: u16, width: u16) -> Self {
        Self {
            top: 0,
            height,
            width,
            visual_lines: Vec::new(),
            cache_width: 0,
        }
    }

    /// (Re)compute the wrap cache if the width has changed since the last call.
    pub fn ensure_wrap(&mut self, doc: &RenderedDoc) {
        if self.cache_width == self.width && !self.visual_lines.is_empty() {
            return;
        }
        self.visual_lines = wrap_all(&doc.lines, self.width);
        self.cache_width = self.width;
        if self.visual_lines.is_empty() {
            self.top = 0;
            return;
        }
        let max_top = self.visual_lines.len().saturating_sub(self.height as usize);
        if self.top > max_top {
            self.top = max_top;
        }
    }

    /// Move `top` by `delta` visual lines, clamped to [0, max_top].
    pub fn scroll_by(&mut self, delta: i32) {
        let max_top = self.visual_lines.len().saturating_sub(self.height as usize);
        let new_top = (self.top as i32 + delta).max(0) as usize;
        self.top = new_top.min(max_top);
    }

    pub fn visible(&self) -> &[VisualLine] {
        let end = (self.top + self.height as usize).min(self.visual_lines.len());
        &self.visual_lines[self.top..end]
    }

    pub fn total_visual_lines(&self) -> usize {
        self.visual_lines.len()
    }

    /// Find the first `VisualLine` index whose logical_index matches the
    /// given logical line (the first visual row of that logical).
    pub fn visual_line_for_logical(&self, logical: usize) -> Option<usize> {
        self.visual_lines
            .iter()
            .position(|vl| vl.logical_index == logical)
    }

    /// Move the viewport to the next heading line after `after_visual`.
    /// No-op if no heading exists further in the document.
    pub fn jump_to_next_heading(&mut self, doc: &RenderedDoc, after_visual: usize) {
        let start_logical = self
            .visual_lines
            .get(after_visual)
            .map(|vl| vl.logical_index)
            .unwrap_or(0);
        let target = doc.headings.iter().find(|h| h.line_index > start_logical);
        if let Some(h) = target {
            if let Some(vi) = self
                .visual_lines
                .iter()
                .position(|vl| vl.logical_index == h.line_index)
            {
                self.top = vi;
            }
        }
    }

    /// Move the viewport to the heading line before `before_visual`.
    /// No-op if no heading exists earlier.
    pub fn jump_to_prev_heading(&mut self, doc: &RenderedDoc, before_visual: usize) {
        let start_logical = self
            .visual_lines
            .get(before_visual)
            .map(|vl| vl.logical_index)
            .unwrap_or(0);
        let target = doc
            .headings
            .iter()
            .rev()
            .find(|h| h.line_index < start_logical);
        if let Some(h) = target {
            if let Some(vi) = self
                .visual_lines
                .iter()
                .position(|vl| vl.logical_index == h.line_index)
            {
                self.top = vi;
            }
        }
    }
}

fn wrap_all(lines: &[Line], width: u16) -> Vec<VisualLine> {
    use crate::layout::LineKind;
    use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

    // Reserve margin (4 cols) to match cat-mode indent.
    let max: usize = (width as usize).saturating_sub(4);

    let mut out = Vec::with_capacity(lines.len());
    for (li, line) in lines.iter().enumerate() {
        // Never-wrap kinds: one visual line each, full byte range.
        match line.kind {
            LineKind::Blank
            | LineKind::HorizontalRule
            | LineKind::Table
            | LineKind::Heading { .. } => {
                out.push(VisualLine {
                    logical_index: li,
                    byte_start: 0,
                    byte_end: line_byte_len(line),
                });
                continue;
            }
            _ => {}
        }

        // Flatten text content for wrapping. Link content is treated as plain
        // text; HeadingImage contributes nothing.
        let text: String = line
            .spans
            .iter()
            .filter_map(|s| match s {
                crate::layout::Span::Text { content, .. }
                | crate::layout::Span::Link { content, .. } => Some(content.as_str()),
                crate::layout::Span::HeadingImage { .. } => None,
            })
            .collect::<Vec<_>>()
            .join("");

        if max == 0 || UnicodeWidthStr::width(text.as_str()) <= max {
            out.push(VisualLine {
                logical_index: li,
                byte_start: 0,
                byte_end: text.len(),
            });
            continue;
        }

        // Width-aware break: accumulate per-char display width, break when
        // next char would exceed `max`. Break at character boundaries (no
        // word-wrap awareness in v1 — simplest correct behavior).
        let mut byte_start = 0usize;
        let mut cur_width = 0usize;
        let mut cur_byte = 0usize;
        for (i, ch) in text.char_indices() {
            let cw = UnicodeWidthChar::width(ch).unwrap_or(0);
            if cur_width + cw > max && cur_byte > byte_start {
                out.push(VisualLine {
                    logical_index: li,
                    byte_start,
                    byte_end: cur_byte,
                });
                byte_start = cur_byte;
                cur_width = 0;
            }
            cur_byte = i + ch.len_utf8();
            cur_width += cw;
        }
        // Flush tail.
        if byte_start < text.len() {
            out.push(VisualLine {
                logical_index: li,
                byte_start,
                byte_end: text.len(),
            });
        } else if text.is_empty() {
            // Empty logical line (e.g. a `Body` with no content) — emit one empty visual.
            out.push(VisualLine {
                logical_index: li,
                byte_start: 0,
                byte_end: 0,
            });
        }
    }
    out
}

fn line_byte_len(line: &Line) -> usize {
    line.spans
        .iter()
        .map(|s| match s {
            Span::Text { content, .. } | Span::Link { content, .. } => content.len(),
            Span::HeadingImage { .. } => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{Line, LineKind, Span, Style};

    fn make_doc(n: usize) -> RenderedDoc {
        let lines = (0..n)
            .map(|i| Line {
                spans: vec![Span::Text {
                    content: format!("line {i}"),
                    style: Style::default(),
                }],
                kind: LineKind::Body,
            })
            .collect();
        RenderedDoc {
            lines,
            headings: vec![],
            images: vec![],
        }
    }

    #[test]
    fn scroll_respects_bounds() {
        let doc = make_doc(10);
        let mut vp = Viewport::new(4, 40);
        vp.ensure_wrap(&doc);

        assert_eq!(vp.top, 0);
        vp.scroll_by(-3);
        assert_eq!(vp.top, 0, "negative delta should clamp at 0");

        vp.scroll_by(100);
        assert_eq!(
            vp.top, 6,
            "positive delta should clamp at total - height (10 - 4)"
        );
        assert_eq!(vp.visible().len(), 4);
    }

    #[test]
    fn empty_doc_visible_is_empty() {
        let doc = make_doc(0);
        let mut vp = Viewport::new(4, 40);
        vp.ensure_wrap(&doc);
        assert!(vp.visible().is_empty());
        assert_eq!(vp.total_visual_lines(), 0);
    }

    #[test]
    fn height_exceeds_total_shows_all() {
        let doc = make_doc(3);
        let mut vp = Viewport::new(10, 40);
        vp.ensure_wrap(&doc);
        assert_eq!(vp.visible().len(), 3);
        // max_top = total - height = 3 - 10 = 0 (saturating)
        vp.scroll_by(100);
        assert_eq!(vp.top, 0);
    }

    #[test]
    fn heading_jump_moves_to_heading_line() {
        use crate::layout::{HeadingEntry, Line, LineKind, Span, Style};

        let lines: Vec<Line> = (0..10)
            .map(|i| Line {
                spans: vec![Span::Text {
                    content: format!("row {i}"),
                    style: Style::default(),
                }],
                kind: LineKind::Body,
            })
            .collect();
        let headings = vec![
            HeadingEntry {
                level: 1,
                text: "A".into(),
                line_index: 3,
            },
            HeadingEntry {
                level: 1,
                text: "B".into(),
                line_index: 7,
            },
        ];
        let doc = RenderedDoc {
            lines,
            headings,
            images: vec![],
        };
        let mut vp = Viewport::new(3, 40);
        vp.ensure_wrap(&doc);

        vp.jump_to_next_heading(&doc, 0);
        assert_eq!(vp.top, 3);

        vp.jump_to_next_heading(&doc, vp.top + 1);
        assert_eq!(vp.top, 7);

        vp.jump_to_prev_heading(&doc, 7);
        assert_eq!(vp.top, 3);
    }

    #[test]
    fn heading_jump_no_op_when_nothing_in_direction() {
        use crate::layout::{HeadingEntry, Line, LineKind, Span, Style};
        let lines = (0..5)
            .map(|i| Line {
                spans: vec![Span::Text {
                    content: format!("r{i}"),
                    style: Style::default(),
                }],
                kind: LineKind::Body,
            })
            .collect();
        let headings = vec![HeadingEntry {
            level: 1,
            text: "A".into(),
            line_index: 2,
        }];
        let doc = RenderedDoc {
            lines,
            headings,
            images: vec![],
        };
        let mut vp = Viewport::new(3, 40);
        vp.ensure_wrap(&doc);
        vp.top = 3;

        // No heading after visual line 3 — expect top unchanged.
        vp.jump_to_next_heading(&doc, 3);
        assert_eq!(vp.top, 3);

        // No heading before line 0 — expect top unchanged.
        vp.top = 0;
        vp.jump_to_prev_heading(&doc, 0);
        assert_eq!(vp.top, 0);
    }

    #[test]
    fn wrap_splits_long_body_line() {
        use crate::layout::{Line, LineKind, Span, Style};
        let doc = RenderedDoc {
            lines: vec![Line {
                spans: vec![Span::Text {
                    content: "alpha beta gamma delta epsilon zeta eta theta".into(),
                    style: Style::default(),
                }],
                kind: LineKind::Body,
            }],
            headings: vec![],
            images: vec![],
        };
        let mut vp = Viewport::new(10, 20);
        vp.ensure_wrap(&doc);
        assert!(
            vp.total_visual_lines() > 1,
            "expected multiple visual lines"
        );
    }

    #[test]
    fn wrap_leaves_table_lines_intact() {
        use crate::layout::{Line, LineKind, Span, Style};
        let doc = RenderedDoc {
            lines: vec![Line {
                spans: vec![Span::Text {
                    content: "col1 | col2 | col3 | col4 | col5 | col6".into(),
                    style: Style::default(),
                }],
                kind: LineKind::Table,
            }],
            headings: vec![],
            images: vec![],
        };
        let mut vp = Viewport::new(10, 20);
        vp.ensure_wrap(&doc);
        assert_eq!(vp.total_visual_lines(), 1, "table lines should not wrap");
    }

    #[test]
    fn wrap_cjk_accounts_for_double_width() {
        use crate::layout::{Line, LineKind, Span, Style};
        // 10 CJK characters = 20 columns of display width.
        let content: String = "你好世界测试内容断行示例".into(); // 12 chars * 2 = 24 cols
        let doc = RenderedDoc {
            lines: vec![Line {
                spans: vec![Span::Text {
                    content: content.clone(),
                    style: Style::default(),
                }],
                kind: LineKind::Body,
            }],
            headings: vec![],
            images: vec![],
        };
        let mut vp = Viewport::new(10, 20);
        vp.ensure_wrap(&doc);
        // With max width 20 - 4 (margin) = 16 cols, 24 cols should split into 2 visual lines.
        assert!(
            vp.total_visual_lines() >= 2,
            "CJK content should wrap across lines"
        );
    }
}
