//! Scroll state + wrap cache for the TUI body.
//!
//! v1 wrap is a no-op (one visual line per logical line). Task 4.4 replaces
//! `wrap_all` with a width-aware breaker.

use crate::layout::{Line, RenderedDoc, Span};

/// Sentinel `logical_index` for VisualLines that don't map to any `doc.lines`
/// entry — the metadata block's rows and its trailing blank. Using `usize::MAX`
/// (no real line ever reaches it) keeps logical→visual lookups used by search
/// and heading navigation from matching real line 0.
pub const NO_LOGICAL: usize = usize::MAX;

/// A wrapped visual line, pointing back to a logical `Line` and the byte
/// range of its content that this visual slice covers.
#[derive(Debug, Clone)]
pub struct VisualLine {
    pub logical_index: usize,
    pub byte_start: usize,
    pub byte_end: usize,
    /// True for the extra rows emitted below a heading's main line so the
    /// heading image's cell footprint matches the viewport's row count.
    /// These rows render as blank and do not carry image placements.
    pub is_spacer: bool,
    /// Set on rows that visualize the document's frontmatter metadata block.
    /// `logical_index` is [`NO_LOGICAL`] for these rows — `draw()` consults
    /// `doc.metadata` instead. See `docs/adr/0001-metadata-block-handling.md`.
    pub metadata_row: Option<MetadataVisualRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataVisualRow {
    /// Single dim line when collapsed.
    Folded,
    /// Top border of the expanded box.
    ExpandedTop,
    /// `usize` is the index into `MetadataInfo.pairs` (or 0 when falling back).
    ExpandedField(usize),
    /// Bottom border of the expanded box.
    ExpandedBottom,
}

pub struct Viewport {
    pub top: usize,
    pub height: u16,
    pub width: u16,
    visual_lines: Vec<VisualLine>,
    cache_width: u16,
    cache_metadata_expanded: bool,
    cache_metadata_shown: bool,
}

impl Viewport {
    pub fn new(height: u16, width: u16) -> Self {
        Self {
            top: 0,
            height,
            width,
            visual_lines: Vec::new(),
            cache_width: 0,
            cache_metadata_expanded: false,
            cache_metadata_shown: false,
        }
    }

    /// (Re)compute the wrap cache if width, metadata visibility, or fold state
    /// have changed since the last call. `show_metadata` and `expanded` come
    /// from `Config.metadata` and `DocEntry.metadata_expanded` respectively.
    pub fn ensure_wrap(&mut self, doc: &RenderedDoc, show_metadata: bool, expanded: bool) {
        if self.cache_width == self.width
            && self.cache_metadata_expanded == expanded
            && self.cache_metadata_shown == show_metadata
            && !self.visual_lines.is_empty()
        {
            return;
        }
        let mut lines = Vec::new();
        if show_metadata {
            if let Some(meta) = &doc.metadata {
                lines.extend(metadata_visual_lines(meta, expanded));
            }
        }
        lines.extend(wrap_all(&doc.lines, self.width));
        self.visual_lines = lines;
        self.cache_width = self.width;
        self.cache_metadata_expanded = expanded;
        self.cache_metadata_shown = show_metadata;
        if self.visual_lines.is_empty() {
            self.top = 0;
            return;
        }
        let max_top = self.visual_lines.len().saturating_sub(self.height as usize);
        if self.top > max_top {
            self.top = max_top;
        }
    }

    /// Drop the wrap cache so the next `ensure_wrap` rebuilds it. Used when
    /// state outside `width` changes (e.g. metadata fold toggle).
    pub fn invalidate_wrap(&mut self) {
        self.visual_lines.clear();
        self.cache_width = 0;
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
            .map(|vl| vl.logical_index);
        // On a metadata/sentinel row we're above the body, so the first heading
        // is "next"; otherwise the first heading strictly after the current line.
        let target = match start_logical {
            Some(l) if l != NO_LOGICAL => doc.headings.iter().find(|h| h.line_index > l),
            _ => doc.headings.first(),
        };
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
            .map(|vl| vl.logical_index);
        // A metadata/sentinel row sits above every heading, so nothing precedes it.
        let target = match start_logical {
            Some(l) if l != NO_LOGICAL => doc.headings.iter().rev().find(|h| h.line_index < l),
            _ => None,
        };
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

/// Build the VisualLines that visualize a document's frontmatter at the top
/// of the viewport. One row when folded, top-border + N-field + bottom-border
/// when expanded. When the heuristic produced no pairs, expanded still emits
/// a single field row carrying the fallback string.
fn metadata_visual_lines(
    meta: &crate::frontmatter::MetadataInfo,
    expanded: bool,
) -> Vec<VisualLine> {
    // All metadata rows carry the sentinel logical index — they don't map to
    // any `doc.lines` entry. See [`NO_LOGICAL`].
    let row = |metadata_row: Option<MetadataVisualRow>, is_spacer: bool| VisualLine {
        logical_index: NO_LOGICAL,
        byte_start: 0,
        byte_end: 0,
        is_spacer,
        metadata_row,
    };

    let mut out = Vec::new();
    if !expanded {
        out.push(row(Some(MetadataVisualRow::Folded), false));
    } else {
        let row_count = if meta.has_pairs() {
            meta.pairs.len()
        } else {
            1
        };
        out.reserve(row_count + 2);
        out.push(row(Some(MetadataVisualRow::ExpandedTop), false));
        for i in 0..row_count {
            out.push(row(Some(MetadataVisualRow::ExpandedField(i)), false));
        }
        out.push(row(Some(MetadataVisualRow::ExpandedBottom), false));
    }
    // Trailing blank row: separate the metadata block from whatever heading
    // or paragraph follows. Rendered as empty via the existing `is_spacer`
    // branch in `draw()`.
    out.push(row(None, true));
    out
}

fn wrap_all(lines: &[Line], width: u16) -> Vec<VisualLine> {
    use crate::layout::LineKind;
    use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

    let max: usize = width as usize;

    let mut out = Vec::with_capacity(lines.len());
    for (li, line) in lines.iter().enumerate() {
        // Headings occupy N screen rows (N = image_rows for H1-H3 with images,
        // 1 otherwise). Emit N VisualLines — a main one plus N-1 spacers — so
        // the viewport's scroll math sees the same row count that draw() and
        // desired_image_placements use. Without this, max_top undercounts and
        // the last headings become unreachable + images overflow body area.
        if let LineKind::Heading { .. } = line.kind {
            let end = line_byte_len(line);
            let rows = heading_row_count(line);
            out.push(VisualLine {
                logical_index: li,
                byte_start: 0,
                byte_end: end,
                is_spacer: false,
                metadata_row: None,
            });
            for _ in 1..rows {
                out.push(VisualLine {
                    logical_index: li,
                    byte_start: 0,
                    byte_end: 0,
                    is_spacer: true,
                    metadata_row: None,
                });
            }
            continue;
        }
        match line.kind {
            LineKind::Blank | LineKind::HorizontalRule | LineKind::Table => {
                out.push(VisualLine {
                    logical_index: li,
                    byte_start: 0,
                    byte_end: line_byte_len(line),
                    is_spacer: false,
                    metadata_row: None,
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
                is_spacer: false,
                metadata_row: None,
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
                    is_spacer: false,
                    metadata_row: None,
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
                is_spacer: false,
                metadata_row: None,
            });
        } else if text.is_empty() {
            // Empty logical line (e.g. a `Body` with no content) — emit one empty visual.
            out.push(VisualLine {
                logical_index: li,
                byte_start: 0,
                byte_end: 0,
                is_spacer: false,
                metadata_row: None,
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

/// How many screen rows a heading line occupies. For H1-H3 with image spans,
/// this is the span's `rows` field (already refined by
/// `tui::refine_image_rows` once the real cell pixel height is known). For
/// text-only headings (H4-H6 or when font loading failed), it's 1.
fn heading_row_count(line: &Line) -> u16 {
    let mut rows: u16 = 1;
    for span in &line.spans {
        if let Span::HeadingImage { rows: r, .. } = span {
            rows = rows.max(*r);
        }
    }
    rows
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
            metadata: None,
        }
    }

    #[test]
    fn scroll_respects_bounds() {
        let doc = make_doc(10);
        let mut vp = Viewport::new(4, 40);
        vp.ensure_wrap(&doc, false, false);

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
        vp.ensure_wrap(&doc, false, false);
        assert!(vp.visible().is_empty());
        assert_eq!(vp.total_visual_lines(), 0);
    }

    #[test]
    fn height_exceeds_total_shows_all() {
        let doc = make_doc(3);
        let mut vp = Viewport::new(10, 40);
        vp.ensure_wrap(&doc, false, false);
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
            metadata: None,
        };
        let mut vp = Viewport::new(3, 40);
        vp.ensure_wrap(&doc, false, false);

        vp.jump_to_next_heading(&doc, 0);
        assert_eq!(vp.top, 3);

        vp.jump_to_next_heading(&doc, vp.top + 1);
        assert_eq!(vp.top, 7);

        vp.jump_to_prev_heading(&doc, 7);
        assert_eq!(vp.top, 3);
    }

    #[test]
    fn heading_jump_accounts_for_metadata_rows() {
        use crate::frontmatter::{MetadataInfo, MetadataKind};
        use crate::layout::{HeadingEntry, Line, LineKind, Span, Style};

        // A doc whose very first body line (logical 0) is a heading, preceded
        // by a shown frontmatter block. The folded metadata occupies visual
        // rows 0 (summary) + 1 (trailing blank), so the heading at logical 0
        // lands on visual row 2 — jumps must resolve to the real heading, not
        // a metadata row that also (formerly) carried logical_index 0.
        let lines: Vec<Line> = (0..5)
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
                line_index: 0,
            },
            HeadingEntry {
                level: 1,
                text: "B".into(),
                line_index: 3,
            },
        ];
        let doc = RenderedDoc {
            lines,
            headings,
            images: vec![],
            metadata: Some(MetadataInfo {
                kind: MetadataKind::Yaml,
                pairs: vec![("title".into(), "T".into())],
                fallback_oneline: String::new(),
            }),
        };
        let mut vp = Viewport::new(3, 40);
        vp.ensure_wrap(&doc, true, false);

        // Two metadata visual rows precede the body.
        assert!(matches!(
            vp.visual_lines[0].metadata_row,
            Some(MetadataVisualRow::Folded)
        ));
        assert_eq!(vp.visual_lines[0].logical_index, NO_LOGICAL);

        // From the metadata top row, "next heading" finds the first heading
        // (logical 0), which sits at visual row 2.
        vp.jump_to_next_heading(&doc, 0);
        assert_eq!(vp.top, 2);

        // Next from there moves to heading B (logical 3) at visual row 5.
        vp.jump_to_next_heading(&doc, vp.top);
        assert_eq!(vp.top, 5);

        // Prev from a metadata/sentinel top row is a no-op (nothing above).
        vp.top = 0;
        vp.jump_to_prev_heading(&doc, 0);
        assert_eq!(vp.top, 0);
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
            metadata: None,
        };
        let mut vp = Viewport::new(3, 40);
        vp.ensure_wrap(&doc, false, false);
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
            metadata: None,
        };
        let mut vp = Viewport::new(10, 20);
        vp.ensure_wrap(&doc, false, false);
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
            metadata: None,
        };
        let mut vp = Viewport::new(10, 20);
        vp.ensure_wrap(&doc, false, false);
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
            metadata: None,
        };
        let mut vp = Viewport::new(10, 20);
        vp.ensure_wrap(&doc, false, false);
        // With max width 20 cols, 24 cols should split into 2 visual lines.
        assert!(
            vp.total_visual_lines() >= 2,
            "CJK content should wrap across lines"
        );
    }
}
