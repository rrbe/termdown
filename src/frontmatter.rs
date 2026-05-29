//! Frontmatter (YAML / TOML metadata block) heuristic parser.
//!
//! We never feed the block to a real YAML/TOML parser. The block's destination
//! is a single dim summary line (cat / TUI folded) or an inline expanded box
//! (TUI), so fidelity beyond "key = value" doesn't matter. See
//! `docs/adr/0001-metadata-block-handling.md`.

use pulldown_cmark::MetadataBlockKind;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataKind {
    Yaml,
    Toml,
}

impl From<MetadataBlockKind> for MetadataKind {
    fn from(k: MetadataBlockKind) -> Self {
        match k {
            MetadataBlockKind::YamlStyle => MetadataKind::Yaml,
            MetadataBlockKind::PlusesStyle => MetadataKind::Toml,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataInfo {
    pub kind: MetadataKind,
    /// Parsed key/value pairs. Empty when the heuristic found zero conforming
    /// lines; renderers fall back to [`Self::fallback_oneline`] in that case.
    pub pairs: Vec<(String, String)>,
    /// Raw block content joined with `, ` for fallback display.
    pub fallback_oneline: String,
}

impl MetadataInfo {
    /// True when the heuristic extracted at least one pair. False means the
    /// renderer should use [`Self::fallback_oneline`] verbatim.
    pub fn has_pairs(&self) -> bool {
        !self.pairs.is_empty()
    }
}

/// Parse a raw frontmatter block into key/value pairs. Splits each non-blank
/// line on the first `:` (YAML) or `=` (TOML), trims, and keeps lines where
/// both sides are non-empty. Comment lines (leading `#`) are skipped.
pub fn parse(raw: &str, kind: MetadataKind) -> MetadataInfo {
    let sep = match kind {
        MetadataKind::Yaml => ':',
        MetadataKind::Toml => '=',
    };

    let mut pairs = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some(idx) = trimmed.find(sep) else {
            continue;
        };
        let key = trimmed[..idx].trim();
        let raw_value = trimmed[idx + sep.len_utf8()..].trim();
        // Reject empty keys, and keys that contain whitespace — the latter
        // usually means the line was a continuation of a multi-line value
        // rather than its own field.
        if key.is_empty() || key.chars().any(char::is_whitespace) {
            continue;
        }
        // Strip surrounding quotes from TOML values (also tolerated for YAML),
        // then require a non-empty value so `key: ""` doesn't yield `key=`.
        let value = strip_quotes(raw_value);
        if value.is_empty() {
            continue;
        }
        pairs.push((key.to_string(), value.to_string()));
    }

    let fallback_oneline = raw
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(", ");

    MetadataInfo {
        kind,
        pairs,
        fallback_oneline,
    }
}

fn strip_quotes(s: &str) -> String {
    let bytes = s.as_bytes();
    if bytes.len() >= 2
        && (bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"'
            || bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\'')
    {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Compose the one-line summary text, NOT including the leading `· metadata · `
/// marker or any width truncation. Callers add those.
pub fn format_pairs_inline(info: &MetadataInfo) -> String {
    if info.has_pairs() {
        info.pairs
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        info.fallback_oneline.clone()
    }
}

/// The folded one-line summary, wrapped as `[metadata · k=v, …]`, truncated to
/// `max_cols` display columns with the closing `]` preserved after the ellipsis.
/// Shared verbatim by `--cat` and the TUI folded row so the two never diverge.
pub fn folded_summary(info: &MetadataInfo, max_cols: usize) -> String {
    let full = format!("[metadata · {}]", format_pairs_inline(info));
    truncate_keep_suffix(&full, max_cols, "]")
}

/// Truncate `s` to `max_cols` display columns. When truncation is needed,
/// append `…` followed by the literal `suffix` (e.g. `"]"`). Returns `s`
/// unchanged when it already fits, and an empty string when `max_cols == 0`.
pub fn truncate_keep_suffix(s: &str, max_cols: usize, suffix: &str) -> String {
    if max_cols == 0 {
        return String::new();
    }
    if s.width() <= max_cols {
        return s.to_string();
    }
    let budget = max_cols.saturating_sub("…".width() + suffix.width());
    let mut acc = String::new();
    let mut width = 0;
    for ch in s.chars() {
        let cw = ch.width().unwrap_or(0);
        if width + cw > budget {
            break;
        }
        acc.push(ch);
        width += cw;
    }
    acc.push('…');
    acc.push_str(suffix);
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml_happy_path() {
        let raw = "title: Hello\nauthor: shawn\ntags: [a, b]\n";
        let info = parse(raw, MetadataKind::Yaml);
        assert_eq!(
            info.pairs,
            vec![
                ("title".into(), "Hello".into()),
                ("author".into(), "shawn".into()),
                ("tags".into(), "[a, b]".into()),
            ]
        );
        assert!(info.has_pairs());
    }

    #[test]
    fn toml_happy_path() {
        let raw = "title = \"Hello\"\nauthor = 'shawn'\ncount = 42\n";
        let info = parse(raw, MetadataKind::Toml);
        assert_eq!(
            info.pairs,
            vec![
                ("title".into(), "Hello".into()),
                ("author".into(), "shawn".into()),
                ("count".into(), "42".into()),
            ]
        );
    }

    #[test]
    fn value_containing_separator() {
        // YAML: only the first `:` separates key/value.
        let raw = "url: https://example.com/path:8080\n";
        let info = parse(raw, MetadataKind::Yaml);
        assert_eq!(
            info.pairs,
            vec![("url".into(), "https://example.com/path:8080".into())]
        );
    }

    #[test]
    fn blank_and_comment_lines_skipped() {
        let raw = "# a comment\n\ntitle: Hi\n# another\nauthor: x\n";
        let info = parse(raw, MetadataKind::Yaml);
        assert_eq!(
            info.pairs,
            vec![("title".into(), "Hi".into()), ("author".into(), "x".into()),]
        );
    }

    #[test]
    fn multiline_continuation_lines_skipped() {
        // Heuristic rejects lines whose "key" contains whitespace (typical of
        // YAML multi-line value continuations).
        let raw = "description: >\n  this is a long\n  paragraph value\ntitle: T\n";
        let info = parse(raw, MetadataKind::Yaml);
        // `description: >` → key=description, value=">" (kept)
        // `  this is a long` → key="this is a long" with spaces → skipped
        // `  paragraph value` → same → skipped
        // `title: T` → kept
        assert_eq!(
            info.pairs,
            vec![
                ("description".into(), ">".into()),
                ("title".into(), "T".into()),
            ]
        );
    }

    #[test]
    fn empty_block_yields_no_pairs() {
        let info = parse("", MetadataKind::Yaml);
        assert!(!info.has_pairs());
        assert_eq!(info.fallback_oneline, "");
    }

    #[test]
    fn unparseable_block_uses_fallback() {
        // Lines with no separator at all — heuristic yields zero pairs.
        let raw = "just\nsome\nrandom text\n";
        let info = parse(raw, MetadataKind::Yaml);
        assert!(!info.has_pairs());
        assert_eq!(info.fallback_oneline, "just, some, random text");
    }

    #[test]
    fn format_pairs_inline_uses_pairs_when_present() {
        let info = parse("a: 1\nb: 2\n", MetadataKind::Yaml);
        assert_eq!(format_pairs_inline(&info), "a=1, b=2");
    }

    #[test]
    fn format_pairs_inline_falls_back_when_no_pairs() {
        let info = parse("plain\ntext\n", MetadataKind::Yaml);
        assert_eq!(format_pairs_inline(&info), "plain, text");
    }

    #[test]
    fn empty_quoted_value_skipped() {
        // `key: ""` strips to an empty value and must not yield `key=`.
        let info = parse("title: \"\"\nauthor: x\n", MetadataKind::Yaml);
        assert_eq!(info.pairs, vec![("author".into(), "x".into())]);
    }

    #[test]
    fn folded_summary_fits_unchanged() {
        let info = parse("a: 1\nb: 2\n", MetadataKind::Yaml);
        assert_eq!(folded_summary(&info, 80), "[metadata · a=1, b=2]");
    }

    #[test]
    fn folded_summary_truncates_keeping_bracket() {
        let info = parse("title: A very long value indeed\n", MetadataKind::Yaml);
        let out = folded_summary(&info, 20);
        assert_eq!(out.width(), 20);
        assert!(out.ends_with("…]"), "closing bracket must survive: {out}");
    }

    #[test]
    fn truncate_keep_suffix_respects_wide_chars() {
        // Each CJK char is two display columns wide.
        let out = truncate_keep_suffix("[中文很长很长很长]", 10, "]");
        assert!(out.width() <= 10, "got width {}: {out}", out.width());
        assert!(out.ends_with("…]"));
    }
}
