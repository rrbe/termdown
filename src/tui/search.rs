//! TUI search state — smart-case literal substring matching across
//! `RenderedDoc.lines`. Regex support is deferred to v2.

use std::ops::Range;

use crate::layout::{Line, RenderedDoc, Span};

#[derive(Debug, Clone)]
pub struct MatchPos {
    pub line_index: usize,
    pub byte_range: Range<usize>,
}

pub struct SearchState {
    pub matches: Vec<MatchPos>,
    pub current: Option<usize>,
}

impl SearchState {
    pub fn new(query: &str, doc: &RenderedDoc) -> Self {
        Self {
            matches: find_all(query, doc),
            current: None,
        }
    }
}

/// Scan the document and return every match position (line index + byte range).
pub fn find_all(query: &str, doc: &RenderedDoc) -> Vec<MatchPos> {
    if query.is_empty() {
        return Vec::new();
    }
    let case_insensitive = !query.chars().any(|c| c.is_uppercase());
    let mut out = Vec::new();
    for (i, line) in doc.lines.iter().enumerate() {
        let haystack = line_text(line);
        find_in_line(&haystack, query, case_insensitive, i, &mut out);
    }
    out
}

fn line_text(line: &Line) -> String {
    let mut s = String::new();
    for sp in &line.spans {
        match sp {
            Span::Text { content, .. } | Span::Link { content, .. } => s.push_str(content),
            Span::HeadingImage { .. } => {}
        }
    }
    s
}

fn find_in_line(
    haystack: &str,
    needle: &str,
    case_insensitive: bool,
    line_index: usize,
    out: &mut Vec<MatchPos>,
) {
    if case_insensitive {
        let lower = haystack.to_lowercase();
        let nlow = needle.to_lowercase();
        // NOTE: for ASCII, to_lowercase preserves byte positions so match
        // offsets in `lower` map 1-to-1 to offsets in the original haystack.
        // For non-ASCII, to_lowercase can change byte lengths (e.g. 'Ä' → 'ä'
        // differs in byte length), so reported ranges may be off. This is a
        // known v1 limitation; revisit if snapshot tests reveal issues.
        find_all_offsets(&lower, &nlow, line_index, needle.len(), out);
    } else {
        find_all_offsets(haystack, needle, line_index, needle.len(), out);
    }
}

fn find_all_offsets(
    haystack: &str,
    needle: &str,
    line_index: usize,
    needle_len: usize,
    out: &mut Vec<MatchPos>,
) {
    let mut start = 0usize;
    while let Some(off) = haystack[start..].find(needle) {
        let abs = start + off;
        out.push(MatchPos {
            line_index,
            byte_range: abs..abs + needle_len,
        });
        start = abs + needle_len.max(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{Line, LineKind, RenderedDoc, Span, Style};

    fn doc_with(lines: &[&str]) -> RenderedDoc {
        RenderedDoc {
            lines: lines
                .iter()
                .map(|t| Line {
                    spans: vec![Span::Text {
                        content: (*t).into(),
                        style: Style::default(),
                    }],
                    kind: LineKind::Body,
                })
                .collect(),
            headings: vec![],
            images: vec![],
        }
    }

    #[test]
    fn smart_case_lowercase_query_is_case_insensitive() {
        let doc = doc_with(&["Hello World", "hello there"]);
        let m = find_all("hello", &doc);
        assert_eq!(m.len(), 2);
        assert_eq!(m[0].line_index, 0);
        assert_eq!(m[1].line_index, 1);
    }

    #[test]
    fn mixed_case_query_is_case_sensitive() {
        let doc = doc_with(&["Hello World", "hello there"]);
        let m = find_all("Hello", &doc);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].line_index, 0);
    }

    #[test]
    fn multiple_matches_on_same_line() {
        let doc = doc_with(&["foo bar foo baz foo"]);
        let m = find_all("foo", &doc);
        assert_eq!(m.len(), 3);
        // Byte ranges should cover 0..3, 8..11, 16..19.
        assert_eq!(m[0].byte_range, 0..3);
        assert_eq!(m[1].byte_range, 8..11);
        assert_eq!(m[2].byte_range, 16..19);
    }

    #[test]
    fn empty_query_returns_no_matches() {
        let doc = doc_with(&["anything"]);
        assert!(find_all("", &doc).is_empty());
    }

    #[test]
    fn case_insensitive_byte_range_uses_original_needle_length() {
        // In case-insensitive mode we search a lowercased haystack but the
        // reported byte range must still refer to the original haystack.
        // For ASCII queries like this, lowercased and original positions align.
        let doc = doc_with(&["HELLO"]);
        let m = find_all("hello", &doc);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].byte_range, 0..5);
    }
}
