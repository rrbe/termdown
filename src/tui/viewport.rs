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
    // Task 4.4 will use these for width-aware wrap slicing.
    #[allow(dead_code)]
    pub byte_start: usize,
    #[allow(dead_code)]
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

    // Used in tests; production callers arrive in a later task.
    #[allow(dead_code)]
    pub fn total_visual_lines(&self) -> usize {
        self.visual_lines.len()
    }
}

fn wrap_all(lines: &[Line], _width: u16) -> Vec<VisualLine> {
    // v1: one visual line per logical line, byte range covers full content.
    // Task 4.4 replaces this with width-aware breaking.
    let mut out = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        let byte_len = line_byte_len(line);
        out.push(VisualLine {
            logical_index: i,
            byte_start: 0,
            byte_end: byte_len,
        });
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
}
