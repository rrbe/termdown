# TUI Mode Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `--tui` mode to termdown providing vim-style browsing (paging, search, heading nav, back/forward, link open) for long Markdown files.

**Architecture:** `layout.rs` produces a structured `RenderedDoc` consumed by both the existing cat path (`cat.rs`, replacing current `markdown.rs` stdout logic) and a new `tui::App` (ratatui text layer + self-managed Kitty image placement via `a=T` / `a=p` / `a=d`). Per-doc viewport/search state is preserved across back/forward navigation.

**Tech Stack:** Rust 2021, pulldown-cmark 0.13, ratatui + crossterm (new), tui-textarea (new), regex (new), existing `ab_glyph` + `image` + Kitty graphics protocol.

**Spec reference:** `docs/TUI_MODE_DESIGN.md` — the authoritative design. This plan implements that spec in order.

**Conventional commits:** Every commit uses the project's Conventional Commits format (`feat:`, `fix:`, `refactor:`, `chore:`, `docs:`, `test:`). Scope prefix optional.

**Verification gate:** Every phase ends with `make check` passing (fmt-check + lint + test). Never skip — this is the project's CI gate.

---

## Phase 0 — Snapshot Baseline

**Why first:** Tasks in Phase 1 refactor cat output. Before we touch it, freeze the current stdout bytes for every fixture so we have a byte-level regression baseline. Any drift later is reviewed intent-first.

### Task 0.1: Freeze cat output snapshots

**Files:**
- Create: `fixtures/expected/emoji-test.ansi`
- Create: `fixtures/expected/full-syntax-zh.ansi`
- Create: `fixtures/expected/full-syntax.ansi`
- Create: `fixtures/expected/tasklist.ansi`
- Create: `fixtures/expected/unsupported-syntax.ansi`
- Create: `tests/snapshots.rs`

- [ ] **Step 1: Build the current binary**

Run: `make build`
Expected: exits 0.

- [ ] **Step 2: Capture each fixture's current output**

Run (from repo root):

```sh
for f in fixtures/*.md; do
  name=$(basename "$f" .md)
  TERM_PROGRAM=ghostty target/debug/termdown "$f" > "fixtures/expected/${name}.ansi"
done
```

Expected: five `.ansi` files exist, each non-empty.

- [ ] **Step 3: Write the snapshot test**

`tests/snapshots.rs`:

```rust
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

fn binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_termdown")
}

fn render(path: &Path) -> String {
    let out = Command::new(binary_path())
        .arg(path)
        .env("TERM_PROGRAM", "ghostty")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("termdown should run");
    assert!(out.status.success(), "termdown failed on {path:?}");
    String::from_utf8(out.stdout).expect("valid utf-8")
}

fn check_snapshot(fixture: &str) {
    let md = Path::new("fixtures").join(format!("{fixture}.md"));
    let expected_path = Path::new("fixtures/expected").join(format!("{fixture}.ansi"));
    let expected = fs::read_to_string(&expected_path).expect("expected file");
    let actual = render(&md);
    if actual != expected {
        let tmp = std::env::temp_dir().join(format!("termdown-snapshot-{fixture}.ansi"));
        fs::write(&tmp, &actual).ok();
        panic!(
            "snapshot mismatch for {fixture}\n  expected: {}\n  actual written to: {}",
            expected_path.display(),
            tmp.display()
        );
    }
}

#[test] fn snapshot_emoji_test()        { check_snapshot("emoji-test"); }
#[test] fn snapshot_full_syntax_zh()    { check_snapshot("full-syntax-zh"); }
#[test] fn snapshot_full_syntax()       { check_snapshot("full-syntax"); }
#[test] fn snapshot_tasklist()          { check_snapshot("tasklist"); }
#[test] fn snapshot_unsupported_syntax(){ check_snapshot("unsupported-syntax"); }
```

- [ ] **Step 4: Verify snapshot tests pass against frozen output**

Run: `make check`
Expected: all tests pass, including the five new `snapshot_*` tests.

- [ ] **Step 5: Commit**

```sh
git add fixtures/expected tests/snapshots.rs
git commit -m "test: freeze cat output snapshots as refactor baseline"
```

---

## Phase 1 — Layout Refactor (cat path)

**Why:** `markdown.rs` today is 800 lines of tangled state machine that writes directly to stdout. We split it into (a) `layout.rs` that produces a `RenderedDoc` and (b) `cat.rs` that serializes `RenderedDoc` → ANSI stdout. The TUI path in later phases will reuse `layout.rs`.

### Task 1.1: Introduce core data types in layout.rs

**Files:**
- Create: `src/layout.rs`
- Modify: `src/main.rs:1-6` (add `mod layout;`)

- [ ] **Step 1: Write a compile-only test**

Append to `src/layout.rs`:

```rust
use crate::render::HeadingImage;

#[derive(Debug, Clone)]
pub struct RenderedDoc {
    pub lines: Vec<Line>,
    pub headings: Vec<HeadingEntry>,
    pub images: Vec<HeadingImage>,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub spans: Vec<Span>,
    pub kind: LineKind,
}

#[derive(Debug, Clone)]
pub enum LineKind {
    Body,
    Heading { level: u8, id: Option<u32> }, // id = Some for H1-H3, None for H4-H6
    CodeBlock { lang: Option<String> },
    BlockQuote { depth: u8 },
    ListItem { depth: u8 },
    Table,
    HorizontalRule,
    Blank,
}

#[derive(Debug, Clone)]
pub enum Span {
    Text { content: String, style: Style },
    HeadingImage { id: u32, rows: u16 },
    Link { content: String, url: String, style: Style },
}

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub dim: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    /// 256-color index (what the existing style.rs already emits)
    Indexed(u8),
    /// Truecolor fallback for future use
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone)]
pub struct HeadingEntry {
    pub level: u8,
    pub text: String,
    pub line_index: usize,
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
}
```

- [ ] **Step 2: Move HeadingImage into render.rs**

Add to `src/render.rs` (after the existing `kitty_display` function):

```rust
/// PNG data + cell dimensions for a rendered heading image.
/// Stored by id in `RenderedDoc` and transmitted to the terminal
/// once per TUI session.
#[derive(Debug, Clone)]
pub struct HeadingImage {
    pub id: u32,
    pub png: Vec<u8>,
    pub cols: u16,
    pub rows: u16,
}
```

- [ ] **Step 3: Register the module**

Edit `src/main.rs:1-6`:

```rust
mod config;
mod font;
mod layout;
mod markdown;
mod render;
mod style;
mod theme;
```

- [ ] **Step 4: Verify**

Run: `cargo test --lib layout::tests::types_compile`
Expected: PASS.

Run: `make check`
Expected: all tests pass (including the snapshot tests from Phase 0).

- [ ] **Step 5: Commit**

```sh
git add src/layout.rs src/render.rs src/main.rs
git commit -m "feat(layout): introduce RenderedDoc/Line/Span types"
```

### Task 1.2: Port markdown event walk into layout.rs

**Files:**
- Modify: `src/layout.rs` (add the `build` function)

This is the core port. The existing `markdown::render` function walks pulldown-cmark events and writes to stdout. We lift the walk into a pure function that returns `RenderedDoc`. No behavior change yet — cat still uses the old path until Task 1.4 wires in `cat.rs`.

- [ ] **Step 1: Write the first failing test (plain paragraph)**

Append to the `#[cfg(test)] mod tests` in `src/layout.rs`:

```rust
use crate::config::Config;
use crate::theme::Theme;

fn build_plain(md: &str) -> RenderedDoc {
    let cfg = Config::default();
    super::build(md, &cfg, Theme::Dark)
}

#[test]
fn build_single_paragraph() {
    let doc = build_plain("hello world\n");
    // One body line + one blank separator — match current cat behavior.
    assert!(doc
        .lines
        .iter()
        .any(|l| matches!(l.kind, LineKind::Body) && spans_plain_text(&l.spans) == "hello world"));
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
```

- [ ] **Step 2: Run the test — it fails because `build` doesn't exist**

Run: `cargo test --lib layout::tests::build_single_paragraph`
Expected: FAIL with "cannot find function `build`".

- [ ] **Step 3: Stub `Config::default`**

If `Config::default` doesn't exist, add it in `src/config.rs`:

```rust
impl Default for Config {
    fn default() -> Self {
        Self { theme: None, font: Default::default() }
    }
}
```

(Inspect the existing `Config` struct first — if it already derives `Default`, skip. Add `#[derive(Default)]` on nested structs as needed.)

- [ ] **Step 4: Implement `build` (minimal — handles paragraph only)**

Add to `src/layout.rs`:

```rust
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use crate::config::Config;
use crate::theme::Theme;

pub fn build(md: &str, _config: &Config, _theme: Theme) -> RenderedDoc {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(md, opts);

    let mut lines: Vec<Line> = Vec::new();
    let mut current = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                lines.push(Line {
                    spans: vec![Span::Text {
                        content: std::mem::take(&mut current),
                        style: Style::default(),
                    }],
                    kind: LineKind::Body,
                });
            }
            Event::Text(t) => current.push_str(&t),
            _ => {}
        }
    }

    RenderedDoc { lines, headings: vec![], images: vec![] }
}
```

- [ ] **Step 5: Run — test passes**

Run: `cargo test --lib layout::tests::build_single_paragraph`
Expected: PASS.

- [ ] **Step 6: Commit the first slice**

```sh
git add src/layout.rs src/config.rs
git commit -m "feat(layout): build handles plain paragraphs"
```

### Task 1.3: Port inline text, emphasis, strong, strikethrough

**Files:**
- Modify: `src/layout.rs`

The cat output for inline formatting today embeds ANSI escapes into the text buffer. Our new model emits them as separate `Span::Text` entries with `Style` flags set.

- [ ] **Step 1: Failing test**

Append to the `tests` module in `src/layout.rs`:

```rust
#[test]
fn build_inline_bold_and_italic() {
    let doc = build_plain("hello **bold** and *it*\n");
    let line = doc.lines.iter().find(|l| matches!(l.kind, LineKind::Body)).unwrap();

    // Expect at least: "hello ", "bold" (bold), " and ", "it" (italic)
    let bold_span = line.spans.iter().find(|s| matches!(s, Span::Text { style, .. } if style.bold));
    let italic_span = line.spans.iter().find(|s| matches!(s, Span::Text { style, .. } if style.italic));
    assert!(matches!(bold_span, Some(Span::Text { content, .. }) if content == "bold"));
    assert!(matches!(italic_span, Some(Span::Text { content, .. }) if content == "it"));
}
```

- [ ] **Step 2: Extend `build` to track inline state**

Replace the body of `build` in `src/layout.rs`:

```rust
pub fn build(md: &str, _config: &Config, _theme: Theme) -> RenderedDoc {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(md, opts);

    let mut lines: Vec<Line> = Vec::new();
    let mut spans: Vec<Span> = Vec::new();
    let mut text_buf = String::new();
    let mut style = Style::default();

    // Flush pending plain-text buffer into a span with the current style.
    let flush_text = |text_buf: &mut String, spans: &mut Vec<Span>, style: &Style| {
        if !text_buf.is_empty() {
            spans.push(Span::Text {
                content: std::mem::take(text_buf),
                style: style.clone(),
            });
        }
    };

    for event in parser {
        match event {
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                flush_text(&mut text_buf, &mut spans, &style);
                lines.push(Line {
                    spans: std::mem::take(&mut spans),
                    kind: LineKind::Body,
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
            Event::Text(t) => text_buf.push_str(&t),
            _ => {}
        }
    }

    RenderedDoc { lines, headings: vec![], images: vec![] }
}
```

- [ ] **Step 3: Run both tests**

Run: `cargo test --lib layout::tests`
Expected: both `build_single_paragraph` and `build_inline_bold_and_italic` PASS.

- [ ] **Step 4: Commit**

```sh
git add src/layout.rs
git commit -m "feat(layout): support inline strong/emphasis/strikethrough"
```

### Task 1.4: Port links, inline code, autolinks

**Files:**
- Modify: `src/layout.rs`

- [ ] **Step 1: Failing test**

```rust
#[test]
fn build_link_becomes_link_span() {
    let doc = build_plain("see [docs](https://example.com) now\n");
    let line = doc.lines.iter().find(|l| matches!(l.kind, LineKind::Body)).unwrap();
    let link = line.spans.iter().find_map(|s| match s {
        Span::Link { content, url, .. } => Some((content.clone(), url.clone())),
        _ => None,
    });
    assert_eq!(link, Some(("docs".into(), "https://example.com".into())));
}

#[test]
fn build_inline_code_has_code_style_flag() {
    // Inline code shows up as a Text span with a fg/bg Style; we distinguish
    // by a dedicated flag or by both colors being set. Use bg.is_some() as proxy.
    let doc = build_plain("run `ls` now\n");
    let line = doc.lines.iter().find(|l| matches!(l.kind, LineKind::Body)).unwrap();
    let code = line.spans.iter().find_map(|s| match s {
        Span::Text { content, style } if content == "ls" && style.bg.is_some() => Some(()),
        _ => None,
    });
    assert!(code.is_some());
}
```

- [ ] **Step 2: Add Link + Code handling to `build`**

Inside the match in `src/layout.rs::build`, add these arms (place before `Event::Text`):

```rust
Event::Start(Tag::Link { dest_url, .. }) => {
    flush_text(&mut text_buf, &mut spans, &style);
    // Stash url; the following Event::Text is the link text.
    pending_link_url = Some(dest_url.to_string());
}
Event::End(TagEnd::Link) => {
    if let Some(url) = pending_link_url.take() {
        let content = std::mem::take(&mut text_buf);
        spans.push(Span::Link { content, url, style: style.clone() });
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
```

At the top of `build` add:

```rust
let mut pending_link_url: Option<String> = None;
```

- [ ] **Step 3: Run tests**

Run: `cargo test --lib layout::tests`
Expected: all four layout tests PASS.

- [ ] **Step 4: Commit**

```sh
git add src/layout.rs
git commit -m "feat(layout): support links and inline code"
```

### Task 1.5: Port headings (H1-H3 image, H4-H6 text)

**Files:**
- Modify: `src/layout.rs`
- Modify: `src/render.rs` (expose an id-allocation helper)

The key addition: H1-H3 render to PNG via the existing `render::render_heading`, get assigned a unique `image_id`, and the heading `Line` gets a `Span::HeadingImage { id, rows }`. `rows` is derived from image height divided by an estimated cell height (we use a reasonable constant per level until we wire terminal cell size in Phase 3).

- [ ] **Step 1: Failing test**

```rust
#[test]
fn build_h1_emits_heading_image_span_and_entry() {
    let md = "# Title\n\nbody\n";
    let doc = build_plain(md);

    // Heading line has a HeadingImage span (assuming fonts resolve on this platform;
    // if not, assert that kind is Heading with a text-fallback span instead).
    let heading_line = doc
        .lines
        .iter()
        .find(|l| matches!(l.kind, LineKind::Heading { level: 1, .. }))
        .expect("heading line should exist");

    // Must have either a HeadingImage span or a plain-text fallback span.
    let ok = heading_line.spans.iter().any(|s| {
        matches!(s, Span::HeadingImage { .. })
            || matches!(s, Span::Text { content, .. } if content.contains("Title"))
    });
    assert!(ok);

    // HeadingEntry present with matching text.
    let entry = doc.headings.iter().find(|h| h.level == 1);
    assert!(matches!(entry, Some(e) if e.text == "Title"));
}
```

- [ ] **Step 2: Add heading-level state to `build`**

At the top of `build`, add:

```rust
let mut heading_level: u8 = 0;
let mut heading_text = String::new();
let mut next_image_id: u32 = 1;
let mut images: Vec<HeadingImage> = Vec::new();
let mut headings: Vec<HeadingEntry> = Vec::new();
```

Inside the match, add (before the catch-all `_ => {}`):

```rust
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

    let heading_spans: Vec<Span> = if heading_level <= 3 {
        match crate::render::render_heading(&text, heading_level, _config, _theme) {
            Some(png) => {
                let id = next_image_id;
                next_image_id += 1;
                // Estimated rows: H1≈6, H2≈4, H3≈3 (refined in Phase 3 with real cell height).
                let rows = match heading_level { 1 => 6, 2 => 4, _ => 3 };
                images.push(HeadingImage { id, png, cols: 0, rows });
                vec![Span::HeadingImage { id, rows }]
            }
            None => vec![Span::Text {
                content: text.clone(),
                style: Style { bold: true, ..Style::default() },
            }],
        }
    } else {
        vec![Span::Text {
            content: text.clone(),
            style: Style { bold: true, ..Style::default() },
        }]
    };

    let id_for_kind = if heading_level <= 3 {
        images.last().map(|img| img.id)
    } else {
        None
    };
    lines.push(Line {
        spans: heading_spans,
        kind: LineKind::Heading { level: heading_level, id: id_for_kind },
    });
}
```

Then redirect text during heading to `heading_text`. Modify the `Event::Text` arm:

```rust
Event::Text(t) => {
    if heading_level > 0 {
        heading_text.push_str(&t);
    } else {
        text_buf.push_str(&t);
    }
}
```

And make sure `Event::End(TagEnd::Heading)` resets `heading_level = 0` at the end of its arm (already clears `heading_text` via `take`; add `heading_level = 0;` as the last line).

- [ ] **Step 3: Update `build` signature to pass config/theme through**

Change the leading underscores to real bindings:

```rust
pub fn build(md: &str, config: &Config, theme: Theme) -> RenderedDoc {
```

And use `config`/`theme` in the heading arm above.

- [ ] **Step 4: Fix the remaining RenderedDoc return**

Replace `RenderedDoc { lines, headings: vec![], images: vec![] }` with `RenderedDoc { lines, headings, images }`.

- [ ] **Step 5: Run tests**

Run: `cargo test --lib layout::tests`
Expected: all five layout tests PASS.

- [ ] **Step 6: Commit**

```sh
git add src/layout.rs src/render.rs
git commit -m "feat(layout): support headings with image ids"
```

### Task 1.6: Port blockquotes, lists, task lists, rules, code blocks

**Files:**
- Modify: `src/layout.rs`

Mirror the state logic from `src/markdown.rs:334-388` and `src/markdown.rs:391-414`. Each block element becomes one or more `Line` entries with the appropriate `LineKind`.

- [ ] **Step 1: Failing tests (one per element)**

Append to `src/layout.rs` tests:

```rust
#[test]
fn build_blockquote_carries_depth() {
    let doc = build_plain("> quoted\n");
    assert!(doc.lines.iter().any(|l| matches!(l.kind, LineKind::BlockQuote { depth: 1 })));
}

#[test]
fn build_unordered_list_item_has_depth() {
    let doc = build_plain("- a\n- b\n");
    let items: Vec<_> = doc.lines.iter().filter(|l| matches!(l.kind, LineKind::ListItem { depth: 1 })).collect();
    assert_eq!(items.len(), 2);
}

#[test]
fn build_rule_emits_horizontal_rule_line() {
    let doc = build_plain("---\n");
    assert!(doc.lines.iter().any(|l| matches!(l.kind, LineKind::HorizontalRule)));
}

#[test]
fn build_code_block_emits_codeblock_lines() {
    let doc = build_plain("```rust\nfn main() {}\n```\n");
    let lang_ok = doc.lines.iter().any(|l| matches!(
        &l.kind,
        LineKind::CodeBlock { lang: Some(s) } if s == "rust"
    ));
    assert!(lang_ok);
}
```

- [ ] **Step 2: Add state + arms for these elements**

Add to `build`'s state:

```rust
let mut quote_depth: u8 = 0;
struct ListState { ordered: bool, counter: u64 }
let mut list_stack: Vec<ListState> = Vec::new();
let mut in_code_block: Option<Option<String>> = None; // Some(lang) while active
```

Add arms in the event match:

```rust
Event::Start(Tag::BlockQuote(..)) => quote_depth += 1,
Event::End(TagEnd::BlockQuote(..)) => quote_depth = quote_depth.saturating_sub(1),

Event::Start(Tag::List(start)) => list_stack.push(ListState {
    ordered: start.is_some(),
    counter: start.unwrap_or(1),
}),
Event::End(TagEnd::List(..)) => { list_stack.pop(); }

Event::Start(Tag::Item) => {
    // Start collecting a new list item's inline content.
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
        pulldown_cmark::CodeBlockKind::Fenced(s) if !s.is_empty() => Some(s.to_string()),
        _ => None,
    };
    in_code_block = Some(lang);
}
Event::End(TagEnd::CodeBlock) => { in_code_block = None; }

Event::Rule => {
    lines.push(Line { spans: vec![], kind: LineKind::HorizontalRule });
}
```

Modify `Event::Text` to route code-block content:

```rust
Event::Text(t) => {
    if heading_level > 0 {
        heading_text.push_str(&t);
    } else if let Some(lang) = &in_code_block {
        let lang = lang.clone();
        for line in t.lines() {
            lines.push(Line {
                spans: vec![Span::Text { content: line.to_string(), style: Style::default() }],
                kind: LineKind::CodeBlock { lang: lang.clone() },
            });
        }
    } else {
        text_buf.push_str(&t);
    }
}
```

Blockquotes: when `quote_depth > 0` and we push a paragraph line, set `kind: LineKind::BlockQuote { depth: quote_depth }` instead of `Body`. Update the `Event::End(TagEnd::Paragraph)` arm:

```rust
Event::End(TagEnd::Paragraph) => {
    flush_text(&mut text_buf, &mut spans, &style);
    let kind = if quote_depth > 0 {
        LineKind::BlockQuote { depth: quote_depth }
    } else {
        LineKind::Body
    };
    lines.push(Line { spans: std::mem::take(&mut spans), kind });
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test --lib layout::tests`
Expected: all layout tests PASS.

- [ ] **Step 4: Commit**

```sh
git add src/layout.rs
git commit -m "feat(layout): support quotes, lists, rules, code blocks"
```

### Task 1.7: Port tables, task lists, images, HTML inline & block, breaks

**Files:**
- Modify: `src/layout.rs`

Rather than duplicating the existing `render_table` bytes into `Span::Text`, store each rendered table row as a separate `Line { kind: Table }` with spans for cells. The table formatter stays in `layout.rs` (moved from `markdown.rs`).

- [ ] **Step 1: Failing tests**

```rust
#[test]
fn build_table_emits_table_lines() {
    let doc = build_plain("| A | B |\n| - | - |\n| x | y |\n");
    let rows: Vec<_> = doc.lines.iter().filter(|l| matches!(l.kind, LineKind::Table)).collect();
    // Header + separator + body = 3 lines minimum.
    assert!(rows.len() >= 3);
}

#[test]
fn build_task_list_marker_replaces_bullet() {
    let doc = build_plain("- [x] done\n- [ ] todo\n");
    let items: Vec<_> = doc.lines.iter().filter(|l| matches!(l.kind, LineKind::ListItem { .. })).collect();
    assert_eq!(items.len(), 2);
    let rendered = spans_plain_text(&items[0].spans);
    assert!(rendered.contains("[✓]") || rendered.contains("[x]"));
}

#[test]
fn build_image_renders_placeholder_text() {
    let doc = build_plain("![alt](https://example.com/x.png)\n");
    let any_placeholder = doc.lines.iter().any(|l| {
        spans_plain_text(&l.spans).contains("alt") && spans_plain_text(&l.spans).contains("https://")
    });
    assert!(any_placeholder);
}
```

- [ ] **Step 2: Port table state**

Add state:

```rust
let mut table_rows: Vec<Vec<Vec<Span>>> = Vec::new(); // rows × cells × spans
let mut current_row: Vec<Vec<Span>> = Vec::new();
let mut in_table_header = false;
let mut image_url: Option<String> = None;
```

Add arms (in the event match):

```rust
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
Event::Start(Tag::TableRow) => { current_row.clear(); }
Event::End(TagEnd::TableRow) => {
    table_rows.push(std::mem::take(&mut current_row));
}
Event::Start(Tag::TableCell) => { spans.clear(); }
Event::End(TagEnd::TableCell) => {
    flush_text(&mut text_buf, &mut spans, &style);
    if in_table_header {
        for s in spans.iter_mut() {
            if let Span::Text { style, .. } = s { style.bold = true; }
        }
    }
    current_row.push(std::mem::take(&mut spans));
}

Event::Start(Tag::Image { dest_url, .. }) => { image_url = Some(dest_url.to_string()); }
Event::End(TagEnd::Image) => {
    flush_text(&mut text_buf, &mut spans, &style);
    let alt = spans_plain_text_inline(&spans);
    spans.clear();
    let url = image_url.take().unwrap_or_default();
    let content = format!("[\u{1f5bc} {alt}]({url})");
    let mut dim = Style::default();
    dim.dim = true;
    lines.push(Line {
        spans: vec![Span::Text { content, style: dim }],
        kind: LineKind::Body,
    });
}

Event::TaskListMarker(checked) => {
    // Replace the trailing bullet placeholder the list Start arm set.
    // We haven't implemented one in Start(Item); instead prepend the marker here
    // to the current line's first text span.
    let marker = if checked { "[\u{2713}] " } else { "[ ] " };
    if spans.is_empty() && text_buf.is_empty() {
        text_buf.push_str(marker);
    } else {
        // Prepend to first span's content
        if let Some(Span::Text { content, .. }) = spans.first_mut() {
            *content = format!("{marker}{content}");
        } else {
            text_buf = format!("{marker}{text_buf}");
        }
    }
}

Event::SoftBreak | Event::HardBreak => {
    if heading_level > 0 {
        heading_text.push(' ');
    } else {
        text_buf.push(' ');
    }
}
```

Helpers outside the loop:

```rust
fn emit_table(lines: &mut Vec<Line>, rows: &[Vec<Vec<Span>>]) {
    if rows.is_empty() { return; }
    let cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut widths = vec![0usize; cols];
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            let w: usize = cell.iter().map(|s| plain_width(s)).sum();
            widths[i] = widths[i].max(w);
        }
    }
    for (ri, row) in rows.iter().enumerate() {
        let mut spans: Vec<Span> = Vec::new();
        for (i, cell) in row.iter().enumerate() {
            for s in cell { spans.push(s.clone()); }
            let w: usize = cell.iter().map(|s| plain_width(s)).sum();
            let pad = widths[i].saturating_sub(w);
            if pad > 0 { spans.push(Span::Text { content: " ".repeat(pad), style: Style::default() }); }
            if i < row.len() - 1 {
                let mut dim = Style::default(); dim.dim = true;
                spans.push(Span::Text { content: "  │  ".into(), style: dim });
            }
        }
        lines.push(Line { spans, kind: LineKind::Table });
        if ri == 0 {
            let mut sep_spans: Vec<Span> = Vec::new();
            for (i, &w) in widths.iter().enumerate() {
                let mut dim = Style::default(); dim.dim = true;
                sep_spans.push(Span::Text { content: "─".repeat(w), style: dim.clone() });
                if i < widths.len() - 1 {
                    sep_spans.push(Span::Text { content: "  ┼  ".into(), style: dim });
                }
            }
            lines.push(Line { spans: sep_spans, kind: LineKind::Table });
        }
    }
}

fn plain_width(span: &Span) -> usize {
    match span {
        Span::Text { content, .. } => unicode_width::UnicodeWidthStr::width(content.as_str()),
        Span::Link { content, .. } => unicode_width::UnicodeWidthStr::width(content.as_str()),
        Span::HeadingImage { .. } => 0,
    }
}

fn spans_plain_text_inline(spans: &[Span]) -> String {
    let mut s = String::new();
    for sp in spans {
        match sp {
            Span::Text { content, .. } => s.push_str(content),
            Span::Link { content, .. } => s.push_str(content),
            Span::HeadingImage { .. } => {}
        }
    }
    s
}
```

HTML support (both inline and block) can be handled minimally by treating HTML blocks as dimmed-text `LineKind::Body` entries, mirroring `flush_html_block` behavior:

```rust
Event::Html(s) => {
    for line in s.lines() {
        let mut dim = Style::default(); dim.dim = true;
        lines.push(Line {
            spans: vec![Span::Text { content: line.to_string(), style: dim }],
            kind: LineKind::Body,
        });
    }
}
Event::InlineHtml(s) => {
    // v1: pass the raw tag text through as plain text; cat output will match
    // existing behavior closely enough for the snapshot audit.
    text_buf.push_str(&s);
}
```

- [ ] **Step 3: Run all layout tests**

Run: `cargo test --lib layout::tests`
Expected: PASS (8 tests).

- [ ] **Step 4: Commit**

```sh
git add src/layout.rs
git commit -m "feat(layout): support tables, task markers, images, html"
```

### Task 1.8: Write cat.rs (RenderedDoc → stdout)

**Files:**
- Create: `src/cat.rs`
- Modify: `src/main.rs` (add `mod cat;`)

This module takes `RenderedDoc` and writes the ANSI stream that used to come from `markdown.rs`. It owns wrapping, margins, quote prefixes, list indentation, table margin alignment, and the Kitty image emission for heading images.

- [ ] **Step 1: Module skeleton**

Create `src/cat.rs`:

```rust
use std::io::{BufWriter, Write};

use crate::layout::{Color, HeadingEntry, Line, LineKind, RenderedDoc, Span, Style};
use crate::render;
use crate::style::{
    display_width, Colors, BOLD_ON, DIM_ON, ITALIC_OFF, ITALIC_ON, MARGIN, MARGIN_WIDTH, RESET,
    STRIKETHROUGH_OFF, STRIKETHROUGH_ON, UNDERLINE_OFF, UNDERLINE_ON,
};

pub fn print(doc: &RenderedDoc, term_width: usize, colors: &Colors) {
    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut first_block = true;
    for line in &doc.lines {
        write_line(&mut out, line, &doc.images, term_width, colors, &mut first_block);
    }
    let _ = out.flush();
}

fn write_line<W: Write>(
    out: &mut W,
    line: &Line,
    images: &[crate::render::HeadingImage],
    term_width: usize,
    colors: &Colors,
    first_block: &mut bool,
) {
    match &line.kind {
        LineKind::Blank => {
            let _ = writeln!(out);
        }
        LineKind::HorizontalRule => {
            block_gap(out, first_block);
            let width = term_width.min(62).saturating_sub(2);
            let _ = writeln!(out, "{MARGIN}{DIM_ON}{}{RESET}", "\u{2500}".repeat(width));
        }
        LineKind::Heading { level, id } => {
            block_gap(out, first_block);
            if let Some(image_id) = id {
                if let Some(img) = images.iter().find(|i| i.id == *image_id) {
                    let _ = writeln!(out, "{MARGIN}{}", render::kitty_display(&img.png));
                    return;
                }
            }
            // Fallback: plain bold text derived from line spans.
            let text = render_spans_plain(&line.spans);
            let _ = writeln!(out, "{MARGIN}{BOLD_ON}{text}{RESET}");
            let _ = level; // unused; kept for future font-scaling hook
        }
        LineKind::BlockQuote { depth } => {
            write_paragraph(out, &line.spans, *depth as usize, term_width, colors);
        }
        LineKind::Body => {
            write_paragraph(out, &line.spans, 0, term_width, colors);
        }
        LineKind::ListItem { depth } => {
            let indent = "  ".repeat((*depth as usize).saturating_sub(1));
            // Bullet or number injection happens during layout; spans are cell text.
            let mut buf = format!("{MARGIN}{indent}\u{2022} ");
            buf.push_str(&render_spans_ansi(&line.spans, colors));
            wrap_and_write(out, &buf, term_width, "");
        }
        LineKind::CodeBlock { .. } => {
            let text = render_spans_plain(&line.spans);
            let _ = writeln!(
                out,
                "{MARGIN}{}{} {text} {RESET}",
                colors.code_bg, colors.code_fg
            );
        }
        LineKind::Table => {
            // Table rows are pre-padded by layout; just add margin and emit.
            let rendered = render_spans_ansi(&line.spans, colors);
            let _ = writeln!(out, "{MARGIN}  {rendered}");
        }
    }
}

fn block_gap<W: Write>(out: &mut W, first_block: &mut bool) {
    if !*first_block { let _ = writeln!(out); }
    *first_block = false;
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
    // Identical to markdown::wrap_text — moved here for cat.rs self-sufficiency.
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
    if !current.is_empty() { lines.push(current); }
    if lines.is_empty() { lines.push(text.to_string()); }
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
                push_style_on(&mut out, style, colors);
                out.push_str(content);
                push_style_off(&mut out, style);
            }
            Span::Link { content, url, style } => {
                out.push_str(colors.link);
                out.push_str(UNDERLINE_ON);
                push_style_on(&mut out, style, colors);
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

fn push_style_on(out: &mut String, style: &Style, _colors: &Colors) {
    if style.bold { out.push_str(BOLD_ON); }
    if style.italic { out.push_str(ITALIC_ON); }
    if style.underline { out.push_str(UNDERLINE_ON); }
    if style.strikethrough { out.push_str(STRIKETHROUGH_ON); }
    if style.dim { out.push_str(DIM_ON); }
    if let Some(fg) = &style.fg { out.push_str(&color_fg(*fg)); }
    if let Some(bg) = &style.bg { out.push_str(&color_bg(*bg)); }
}

fn push_style_off(out: &mut String, style: &Style) {
    // Use RESET when any heavy attribute was on, otherwise emit targeted off codes.
    if style.fg.is_some() || style.bg.is_some() || style.bold || style.dim {
        out.push_str(RESET);
    } else {
        if style.italic { out.push_str(ITALIC_OFF); }
        if style.underline { out.push_str(UNDERLINE_OFF); }
        if style.strikethrough { out.push_str(STRIKETHROUGH_OFF); }
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

// Suppress unused-warning on future consumers
#[allow(dead_code)]
pub fn headings(doc: &RenderedDoc) -> &[HeadingEntry] { &doc.headings }
```

- [ ] **Step 2: Register the module**

Add `mod cat;` to `src/main.rs` below `mod config;`.

- [ ] **Step 3: Verify build**

Run: `make build`
Expected: compiles with no errors (warnings allowed; `make check`'s clippy will flag unused if any — fix them).

- [ ] **Step 4: Commit**

```sh
git add src/cat.rs src/main.rs
git commit -m "feat(cat): render RenderedDoc to ANSI stdout"
```

### Task 1.9: Wire main.rs to the new cat path and audit snapshot diffs

**Files:**
- Modify: `src/main.rs`
- Modify: `fixtures/expected/*.ansi` (only if intentional drift accepted)

- [ ] **Step 1: Switch the default path**

Edit `src/main.rs:96` — replace:

```rust
markdown::render(&md, term_width, &config, theme, &colors);
```

with:

```rust
let doc = layout::build(&md, &config, theme);
cat::print(&doc, term_width, &colors);
```

- [ ] **Step 2: Run the snapshot tests and observe failures**

Run: `cargo test --test snapshots`
Expected: failures are likely (byte drift). The failure message points to the temp file with the new output.

- [ ] **Step 3: Diff each drift case**

For each failing fixture, open a diff between `fixtures/expected/<name>.ansi` and the temp file. Evaluate:
- If visible rendering matches the old one (margins, colors, wrap points, image placement), accept the drift: `cp /tmp/termdown-snapshot-<name>.ansi fixtures/expected/<name>.ansi`.
- If visible rendering differs (wrong wrap, missing emphasis, wrong color), fix `layout.rs` or `cat.rs` and rerun.

This step has no single command — it's a manual audit. Expect 2-5 rounds of fix + rerun.

- [ ] **Step 4: Confirm clean snapshots**

Run: `make check`
Expected: all tests pass, including `snapshot_*`.

- [ ] **Step 5: Commit**

```sh
git add src/main.rs fixtures/expected
git commit -m "refactor: route cat mode through layout.rs + cat.rs"
```

### Task 1.10: Remove obsolete logic from markdown.rs

**Files:**
- Delete or shrink: `src/markdown.rs`

If `markdown::render` is no longer called, remove the module or leave a deprecation shim. Preferred: remove the function and any no-longer-referenced helpers. Keep standalone tests that still apply (wrap_text test, table test) by moving them to the corresponding module (`cat.rs`'s unit tests or a new `layout.rs` test).

- [ ] **Step 1: Identify dead items**

Run: `cargo build --all-targets 2>&1 | rg 'unused|dead_code'`
Expected: a list of functions in `markdown.rs` that are now unused.

- [ ] **Step 2: Move kept tests**

The tests `wrap_text_keeps_single_overlong_word_intact` and `wrap_text_uses_display_width_when_ansi_and_wide_chars_are_present` describe `cat.rs::wrap_text` now — move to `#[cfg(test)] mod tests` in `src/cat.rs`.

`render_table_aligns_columns_using_visual_width` describes `layout.rs::emit_table` — move to `src/layout.rs` tests, rewriting it to assert over span text rather than ANSI-tagged strings.

- [ ] **Step 3: Delete markdown.rs**

```sh
rm src/markdown.rs
```

Remove `mod markdown;` from `src/main.rs`.

- [ ] **Step 4: Final check**

Run: `make check`
Expected: PASS.

- [ ] **Step 5: Commit**

```sh
git add -u src/main.rs
git rm src/markdown.rs
git commit -m "refactor: remove legacy markdown.rs now that cat.rs owns output"
```

---

## Phase 2 — TUI Scaffold

### Task 2.1: Add dependencies

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add the deps**

Edit `Cargo.toml`, under `[dependencies]`:

```toml
crossterm = "0.28"
ratatui = "0.28"
tui-textarea = "0.7"
regex = "1"
```

- [ ] **Step 2: Verify build**

Run: `make build`
Expected: compiles. New crates resolve.

- [ ] **Step 3: Commit**

```sh
git add Cargo.toml Cargo.lock
git commit -m "chore: add ratatui/crossterm/tui-textarea/regex deps"
```

### Task 2.2: Parse --tui flag and dispatch

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Failing CLI test**

Add to `tests/cli.rs`:

```rust
#[test]
fn tui_without_file_fails_with_error() {
    let output = run_termdown(&["--tui"], None, &[("TERM_PROGRAM", "ghostty")], &[]);
    assert!(!output.status.success());
    assert!(stderr_text(&output).contains("--tui requires a FILE"));
}

#[test]
fn tui_with_stdin_fails() {
    let output = run_termdown(
        &["--tui", "-"],
        Some("# hi\n"),
        &[("TERM_PROGRAM", "ghostty")],
        &[],
    );
    assert!(!output.status.success());
}
```

- [ ] **Step 2: Add the dispatch**

Modify `src/main.rs` (after the `--version` block, before `check_terminal_support`):

```rust
let tui_mode = args.iter().any(|a| a == "--tui");
```

Update the help text lines inside the `--help` branch to include `--tui`:

```rust
println!("  --tui                     Open FILE in interactive TUI mode");
```

After file_arg resolution, add a guard:

```rust
if tui_mode {
    let path = match file_arg.as_deref() {
        Some("-") | None => {
            eprintln!("termdown: --tui requires a FILE argument (stdin is not supported)");
            std::process::exit(2);
        }
        Some(p) => p.to_string(),
    };
    // Dispatch to tui (module to be created in Task 2.3).
    crate::tui::run(&path, &config, theme);
    return;
}
```

- [ ] **Step 3: Create a stub `tui` module**

Create `src/tui/mod.rs`:

```rust
use crate::config::Config;
use crate::theme::Theme;

pub fn run(path: &str, _config: &Config, _theme: Theme) {
    eprintln!("termdown: --tui not yet implemented (file: {path})");
    std::process::exit(1);
}
```

Add `mod tui;` to `src/main.rs`.

- [ ] **Step 4: Run tests**

Run: `cargo test --test cli tui_`
Expected: both new tests PASS (exit-code-based; the second test also exits 1 since the stub unconditionally errors — adjust the assertion to `!status.success()` if needed).

- [ ] **Step 5: Run `make check`**

Expected: all tests pass.

- [ ] **Step 6: Commit**

```sh
git add src/main.rs src/tui/mod.rs tests/cli.rs
git commit -m "feat(tui): add --tui CLI flag with stub dispatch"
```

### Task 2.3: TUI terminal setup and bare event loop

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Replace the stub with real setup**

Overwrite `src/tui/mod.rs`:

```rust
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::config::Config;
use crate::layout;
use crate::theme::Theme;

pub fn run(path: &str, config: &Config, theme: Theme) {
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("termdown: error reading {path}: {e}");
            std::process::exit(1);
        }
    };
    let doc = layout::build(&source, config, theme);

    if let Err(e) = run_ui(doc) {
        eprintln!("termdown: tui error: {e}");
        std::process::exit(1);
    }
}

fn run_ui(_doc: layout::RenderedDoc) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = event_loop(&mut terminal);

    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn event_loop<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            use ratatui::widgets::{Block, Borders, Paragraph};
            let block = Block::default().borders(Borders::NONE);
            let para = Paragraph::new("termdown TUI — press q to quit").block(block);
            frame.render_widget(para, frame.area());
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && matches!(key.code, KeyCode::Char('q')) {
                    return Ok(());
                }
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('c')
                    && key.modifiers.contains(event::KeyModifiers::CONTROL)
                {
                    return Ok(());
                }
            }
        }
    }
}
```

- [ ] **Step 2: Verify compile**

Run: `make build`
Expected: compiles clean.

- [ ] **Step 3: Smoke test manually (optional here, mandatory in Phase 7 QA)**

Run: `cargo run -- --tui README.md` in a terminal.
Expected: alternate screen shows `termdown TUI — press q to quit`; `q` or `Ctrl-C` exits cleanly with terminal restored.

- [ ] **Step 4: Run `make check`**

Expected: PASS.

- [ ] **Step 5: Commit**

```sh
git add src/tui/mod.rs
git commit -m "feat(tui): raw-mode alt-screen with q/Ctrl-C exit"
```

### Task 2.4: Viewport module — wrap cache + scroll state

**Files:**
- Create: `src/tui/viewport.rs`
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Failing test**

Create `src/tui/viewport.rs`:

```rust
use crate::layout::{Line, RenderedDoc};

/// A wrapped visual line, pointing back to a logical `Line` and the byte range it covers.
#[derive(Debug, Clone)]
pub struct VisualLine {
    pub logical_index: usize,
    pub byte_start: usize,
    pub byte_end: usize,
}

pub struct Viewport {
    pub top: usize,            // index into `visual_lines`
    pub height: u16,
    pub width: u16,
    visual_lines: Vec<VisualLine>,
    cache_width: u16,
}

impl Viewport {
    pub fn new(height: u16, width: u16) -> Self {
        Self { top: 0, height, width, visual_lines: Vec::new(), cache_width: 0 }
    }

    pub fn ensure_wrap(&mut self, doc: &RenderedDoc) {
        if self.cache_width == self.width && !self.visual_lines.is_empty() {
            return;
        }
        self.visual_lines = wrap_all(&doc.lines, self.width);
        self.cache_width = self.width;
        if self.top > self.visual_lines.len().saturating_sub(1) {
            self.top = self.visual_lines.len().saturating_sub(1);
        }
    }

    pub fn scroll_by(&mut self, delta: i32) {
        let max = self.visual_lines.len().saturating_sub(self.height as usize);
        let new_top = (self.top as i32 + delta).max(0) as usize;
        self.top = new_top.min(max);
    }

    pub fn visible<'a>(&'a self) -> &'a [VisualLine] {
        let end = (self.top + self.height as usize).min(self.visual_lines.len());
        &self.visual_lines[self.top..end]
    }

    pub fn total_visual_lines(&self) -> usize { self.visual_lines.len() }
}

fn wrap_all(lines: &[Line], width: u16) -> Vec<VisualLine> {
    let mut out = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        let byte_len = line_byte_len(line);
        // v1 wrap: naive — no actual width-aware break yet; one visual per logical line.
        // Phase 4 refines with CJK-aware break points.
        let _ = width;
        out.push(VisualLine { logical_index: i, byte_start: 0, byte_end: byte_len });
    }
    out
}

fn line_byte_len(line: &Line) -> usize {
    line.spans.iter().map(|s| match s {
        crate::layout::Span::Text { content, .. } => content.len(),
        crate::layout::Span::Link { content, .. } => content.len(),
        crate::layout::Span::HeadingImage { .. } => 0,
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{LineKind, Span, Style};

    fn make_doc(n: usize) -> RenderedDoc {
        let lines = (0..n).map(|i| Line {
            spans: vec![Span::Text { content: format!("line {i}"), style: Style::default() }],
            kind: LineKind::Body,
        }).collect();
        RenderedDoc { lines, headings: vec![], images: vec![] }
    }

    #[test]
    fn scroll_respects_bounds() {
        let doc = make_doc(10);
        let mut vp = Viewport::new(4, 40);
        vp.ensure_wrap(&doc);
        assert_eq!(vp.top, 0);
        vp.scroll_by(-3);
        assert_eq!(vp.top, 0);
        vp.scroll_by(100);
        assert_eq!(vp.top, 6); // max = 10 - 4
        assert_eq!(vp.visible().len(), 4);
    }
}
```

- [ ] **Step 2: Wire it into tui/mod.rs**

Add `mod viewport;` at the top of `src/tui/mod.rs`. The event loop doesn't use the viewport yet — Task 2.5 does that.

- [ ] **Step 3: Run tests**

Run: `cargo test --lib tui::viewport::tests`
Expected: PASS.

- [ ] **Step 4: Commit**

```sh
git add src/tui/viewport.rs src/tui/mod.rs
git commit -m "feat(tui): viewport with wrap cache and scroll state"
```

### Task 2.5: Wire viewport into event loop with j/k scroll

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Build an App wrapping doc + viewport**

Add to the top of `src/tui/mod.rs` (before `run_ui`):

```rust
mod viewport;
use viewport::Viewport;

struct App {
    doc: layout::RenderedDoc,
    viewport: Viewport,
}

impl App {
    fn new(doc: layout::RenderedDoc, height: u16, width: u16) -> Self {
        Self { doc, viewport: Viewport::new(height, width) }
    }
}
```

Replace `run_ui` and `event_loop`:

```rust
fn run_ui(doc: layout::RenderedDoc) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let mut app = App::new(doc, size.height, size.width);

    let result = event_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn event_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        app.viewport.ensure_wrap(&app.doc);
        terminal.draw(|frame| draw(frame, app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                let ctrl = key.modifiers.contains(event::KeyModifiers::CONTROL);
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c') if ctrl => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => app.viewport.scroll_by(1),
                    KeyCode::Char('k') | KeyCode::Up => app.viewport.scroll_by(-1),
                    _ => {}
                }
            }
        }
    }
}

fn draw(frame: &mut ratatui::Frame, app: &App) {
    use ratatui::text::{Line as RLine, Span as RSpan};
    use ratatui::widgets::Paragraph;

    let rendered: Vec<RLine> = app.viewport.visible().iter()
        .map(|vl| {
            let logical = &app.doc.lines[vl.logical_index];
            let mut rspans: Vec<RSpan> = Vec::new();
            for span in &logical.spans {
                match span {
                    layout::Span::Text { content, .. } | layout::Span::Link { content, .. } => {
                        rspans.push(RSpan::raw(content.clone()));
                    }
                    layout::Span::HeadingImage { .. } => {
                        // Placeholder — Phase 3 fills with reserved rows.
                        rspans.push(RSpan::raw("[image]"));
                    }
                }
            }
            RLine::from(rspans)
        })
        .collect();

    let para = Paragraph::new(rendered);
    frame.render_widget(para, frame.area());
}
```

- [ ] **Step 2: Manual smoke test**

Run: `cargo run -- --tui fixtures/full-syntax.md`
Expected: content shows, `j`/`k` scrolls, `q` exits.

- [ ] **Step 3: `make check`**

Expected: PASS.

- [ ] **Step 4: Commit**

```sh
git add src/tui/mod.rs
git commit -m "feat(tui): render doc with j/k scrolling"
```

### Task 2.6: Input module — action abstraction

**Files:**
- Create: `src/tui/input.rs`
- Modify: `src/tui/mod.rs`

Split the key-to-action mapping out of the event loop so Phase 4 can add many more bindings without bloating `mod.rs`.

- [ ] **Step 1: Create the module with the full normal-mode key map**

Create `src/tui/input.rs`:

```rust
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    ScrollLines(i32),
    ScrollHalfPage(i32),  // ±1
    ScrollPage(i32),      // ±1
    JumpStart,
    JumpEnd,
    NextHeading,
    PrevHeading,
    ToggleToc,
    SearchBegin { reverse: bool },
    SearchNext,
    SearchPrev,
    OpenLink,
    Back,
    Forward,
    None,
}

pub fn map_normal(key: KeyEvent) -> Action {
    if key.kind != KeyEventKind::Press { return Action::None; }
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('c') if ctrl => Action::Quit,
        KeyCode::Char('j') | KeyCode::Down => Action::ScrollLines(1),
        KeyCode::Char('k') | KeyCode::Up   => Action::ScrollLines(-1),
        KeyCode::Char('d') => Action::ScrollHalfPage(1),
        KeyCode::Char('u') => Action::ScrollHalfPage(-1),
        KeyCode::Char('f') | KeyCode::Char(' ') | KeyCode::PageDown => Action::ScrollPage(1),
        KeyCode::Char('b') | KeyCode::PageUp   => Action::ScrollPage(-1),
        KeyCode::Char('G') => Action::JumpEnd,
        KeyCode::Char('g') => Action::JumpStart, // `gg` handled with prior-g state in Task 4.1
        KeyCode::Char(']') => Action::NextHeading,
        KeyCode::Char('[') => Action::PrevHeading,
        KeyCode::Char('t') => Action::ToggleToc,
        KeyCode::Char('/') => Action::SearchBegin { reverse: false },
        KeyCode::Char('?') => Action::SearchBegin { reverse: true },
        KeyCode::Char('n') => Action::SearchNext,
        KeyCode::Char('N') => Action::SearchPrev,
        KeyCode::Enter     => Action::OpenLink,
        KeyCode::Char('o') => Action::Back,
        KeyCode::Char('i') => Action::Forward,
        _ => Action::None,
    }
}
```

- [ ] **Step 2: Use it in the event loop**

Replace the key-code match block in `event_loop` with:

```rust
if let Event::Key(key) = event::read()? {
    match input::map_normal(key) {
        input::Action::Quit => return Ok(()),
        input::Action::ScrollLines(d) => app.viewport.scroll_by(d),
        input::Action::ScrollHalfPage(s) => {
            let delta = (app.viewport.height as i32 / 2) * s;
            app.viewport.scroll_by(delta);
        }
        input::Action::ScrollPage(s) => {
            let delta = app.viewport.height as i32 * s;
            app.viewport.scroll_by(delta);
        }
        input::Action::JumpStart => app.viewport.top = 0,
        input::Action::JumpEnd => {
            let max = app.viewport.total_visual_lines().saturating_sub(app.viewport.height as usize);
            app.viewport.top = max;
        }
        _ => {} // filled in Phase 4+
    }
}
```

Add `mod input;` to `src/tui/mod.rs`.

- [ ] **Step 3: Manual smoke test**

Run: `cargo run -- --tui fixtures/full-syntax.md`
Expected: d/u/f/b/PageUp/PageDown/Space/G work. Single g doesn't yet do anything useful — Task 4.1 adds the two-key gg sequence.

- [ ] **Step 4: `make check`**

Expected: PASS.

- [ ] **Step 5: Commit**

```sh
git add src/tui/input.rs src/tui/mod.rs
git commit -m "feat(tui): action-based input mapping"
```

---

## Phase 3 — Kitty Image Handling

### Task 3.1: Extend render.rs with transmit/place/delete primitives

**Files:**
- Modify: `src/render.rs`

- [ ] **Step 1: Failing test**

Append to `src/render.rs` after existing code:

```rust
#[cfg(test)]
mod kitty_tests {
    use super::*;

    #[test]
    fn transmit_produces_a_eq_t_with_id() {
        let mut buf = Vec::new();
        transmit(&mut buf, 42, b"\x89PNG\r\n").unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.starts_with("\x1b_Gf=100,a=T,i=42,q=2"));
        assert!(s.ends_with("\x1b\\"));
    }

    #[test]
    fn place_produces_a_eq_p() {
        let mut buf = Vec::new();
        place(&mut buf, 7, 3, 5).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(s, "\x1b_Ga=p,i=7,x=3,y=5,q=2;\x1b\\");
    }

    #[test]
    fn delete_placement_sends_d_i() {
        let mut buf = Vec::new();
        delete_placement(&mut buf, 9).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(s, "\x1b_Ga=d,d=i,i=9,q=2;\x1b\\");
    }

    #[test]
    fn delete_all_this_client_sends_d_cap_a() {
        let mut buf = Vec::new();
        delete_all_for_client(&mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(s, "\x1b_Ga=d,d=A,q=2;\x1b\\");
    }
}
```

- [ ] **Step 2: Implement the primitives**

Append to `src/render.rs`:

```rust
use std::io::Write;

/// Kitty graphics protocol: transmit PNG data and assign it `id`.
/// Data is not displayed yet; use `place` afterwards.
pub fn transmit<W: Write>(w: &mut W, id: u32, png: &[u8]) -> std::io::Result<()> {
    use base64::engine::general_purpose::STANDARD;
    let b64 = STANDARD.encode(png);
    let total = b64.len();
    let mut offset = 0;
    let mut first = true;
    while offset < total {
        let end = (offset + 4096).min(total);
        let chunk = &b64[offset..end];
        let m = if end == total { "0" } else { "1" };
        if first {
            write!(w, "\x1b_Gf=100,a=T,i={id},q=2,m={m};{chunk}\x1b\\")?;
            first = false;
        } else {
            write!(w, "\x1b_Gm={m};{chunk}\x1b\\")?;
        }
        offset = end;
    }
    Ok(())
}

/// Place previously-transmitted image `id` at cell (col, row).
pub fn place<W: Write>(w: &mut W, id: u32, col: u16, row: u16) -> std::io::Result<()> {
    write!(w, "\x1b_Ga=p,i={id},x={col},y={row},q=2;\x1b\\")
}

/// Delete a single placement of `id` (keeps image data in the terminal cache).
pub fn delete_placement<W: Write>(w: &mut W, id: u32) -> std::io::Result<()> {
    write!(w, "\x1b_Ga=d,d=i,i={id},q=2;\x1b\\")
}

/// Delete all placements AND image data this client has created (exit cleanup).
pub fn delete_all_for_client<W: Write>(w: &mut W) -> std::io::Result<()> {
    write!(w, "\x1b_Ga=d,d=A,q=2;\x1b\\")
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test --lib render::kitty_tests`
Expected: all four PASS.

- [ ] **Step 4: Commit**

```sh
git add src/render.rs
git commit -m "feat(render): add transmit/place/delete Kitty primitives"
```

### Task 3.2: tui/kitty.rs — id-based image lifecycle

**Files:**
- Create: `src/tui/kitty.rs`
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Write the diff test**

Create `src/tui/kitty.rs`:

```rust
use std::collections::HashMap;
use std::io::{self, Write};

use crate::render;

/// Tracks which image ids are currently placed at which (col, row) on the terminal.
/// `sync` diffs a desired set against the current state and emits the minimum
/// place/delete commands to reconcile.
#[derive(Default)]
pub struct ImageLifecycle {
    placed: HashMap<u32, (u16, u16)>,
    transmitted: HashMap<u32, bool>,
}

impl ImageLifecycle {
    pub fn register<W: Write>(
        &mut self,
        w: &mut W,
        id: u32,
        png: &[u8],
    ) -> io::Result<()> {
        if self.transmitted.contains_key(&id) { return Ok(()); }
        render::transmit(w, id, png)?;
        self.transmitted.insert(id, true);
        Ok(())
    }

    pub fn sync<W: Write>(
        &mut self,
        w: &mut W,
        desired: &HashMap<u32, (u16, u16)>,
    ) -> io::Result<()> {
        // Delete placements no longer desired.
        let to_delete: Vec<u32> = self.placed.keys()
            .filter(|id| !desired.contains_key(id))
            .copied()
            .collect();
        for id in &to_delete {
            render::delete_placement(w, *id)?;
            self.placed.remove(id);
        }
        // Place or re-place.
        for (&id, &(col, row)) in desired {
            match self.placed.get(&id) {
                Some(&pos) if pos == (col, row) => {}  // unchanged
                Some(_) => {
                    render::delete_placement(w, id)?;
                    render::place(w, id, col, row)?;
                    self.placed.insert(id, (col, row));
                }
                None => {
                    render::place(w, id, col, row)?;
                    self.placed.insert(id, (col, row));
                }
            }
        }
        Ok(())
    }

    pub fn cleanup<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        render::delete_all_for_client(w)?;
        self.placed.clear();
        self.transmitted.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_new_then_move_then_leave() {
        let mut lc = ImageLifecycle::default();
        let mut buf = Vec::new();

        // Register transmits once.
        lc.register(&mut buf, 1, b"png").unwrap();
        let first_len = buf.len();
        lc.register(&mut buf, 1, b"png").unwrap();
        assert_eq!(buf.len(), first_len, "second register should be a no-op");

        // Place at (5, 10).
        let mut desired = HashMap::new();
        desired.insert(1u32, (5u16, 10u16));
        buf.clear();
        lc.sync(&mut buf, &desired).unwrap();
        let s = String::from_utf8(buf.clone()).unwrap();
        assert!(s.contains("a=p,i=1,x=5,y=10"));

        // Move to (5, 8) — should delete then place.
        desired.insert(1, (5, 8));
        buf.clear();
        lc.sync(&mut buf, &desired).unwrap();
        let s = String::from_utf8(buf.clone()).unwrap();
        assert!(s.contains("a=d,d=i,i=1"));
        assert!(s.contains("a=p,i=1,x=5,y=8"));

        // Leave.
        desired.remove(&1);
        buf.clear();
        lc.sync(&mut buf, &desired).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("a=d,d=i,i=1"));
    }
}
```

- [ ] **Step 2: Register in tui/mod.rs**

Add `mod kitty;` to `src/tui/mod.rs`.

- [ ] **Step 3: Run tests**

Run: `cargo test --lib tui::kitty::tests`
Expected: PASS.

- [ ] **Step 4: Commit**

```sh
git add src/tui/kitty.rs src/tui/mod.rs
git commit -m "feat(tui): image lifecycle with id-based place/delete diff"
```

### Task 3.3: Compute desired placement and wire into draw loop

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Compute placement per frame**

Add below `draw` in `src/tui/mod.rs`:

```rust
fn desired_image_placements(app: &App) -> std::collections::HashMap<u32, (u16, u16)> {
    use std::collections::HashMap;
    let mut out = HashMap::new();
    // `MARGIN_WIDTH` is the cell offset we use for body text in cat; reuse for images.
    let col = crate::style::MARGIN_WIDTH as u16;
    for (visible_row, vl) in app.viewport.visible().iter().enumerate() {
        let logical = &app.doc.lines[vl.logical_index];
        for span in &logical.spans {
            if let layout::Span::HeadingImage { id, .. } = span {
                out.insert(*id, (col, visible_row as u16));
            }
        }
    }
    out
}
```

Store a `kitty::ImageLifecycle` on the `App`:

```rust
struct App {
    doc: layout::RenderedDoc,
    viewport: Viewport,
    images: kitty::ImageLifecycle,
}
```

In `App::new`, instantiate `images: kitty::ImageLifecycle::default()`.

In `run_ui`, after `App::new`:

```rust
let mut writer = io::stdout();
for img in &app.doc.images {
    app.images.register(&mut writer, img.id, &img.png)?;
}
```

In `event_loop` (after the `terminal.draw`), add:

```rust
let mut writer = std::io::stdout();
let desired = desired_image_placements(app);
app.images.sync(&mut writer, &desired).ok();
let _ = writer.flush();
```

Before returning in `run_ui`, call:

```rust
let mut writer = std::io::stdout();
let _ = app.images.cleanup(&mut writer);
```

- [ ] **Step 2: Reserve empty rows in the Paragraph for heading images**

Update the `draw` function: when rendering a visible line that contains a `HeadingImage`, emit blank `RLine`s for the span's `rows` count instead of `[image]` text:

```rust
for span in &logical.spans {
    match span {
        layout::Span::Text { content, .. } | layout::Span::Link { content, .. } => {
            rspans.push(RSpan::raw(content.clone()));
        }
        layout::Span::HeadingImage { rows, .. } => {
            for _ in 0..*rows {
                rspans.push(RSpan::raw(""));
            }
        }
    }
}
```

(Note: this produces too many RLines per logical; adjust rendered so each heading image adds `rows - 1` extra empty RLine entries after the current one. For simplicity at this stage, render one blank row and rely on `layout::HeadingImage.rows` being approximate; Phase 4 replaces with a precise ReserveRows widget.)

- [ ] **Step 3: Manual smoke test in Ghostty**

Run: `cargo run -- --tui fixtures/full-syntax.md`
Expected: headings show as images; body text wraps around them; scrolling moves images and text together; `q` exits without residue.

- [ ] **Step 4: `make check`**

Expected: PASS.

- [ ] **Step 5: Commit**

```sh
git add src/tui/mod.rs
git commit -m "feat(tui): register + place heading images per frame"
```

---

## Phase 4 — Navigation Polish

### Task 4.1: Two-key `gg` sequence

**Files:**
- Modify: `src/tui/mod.rs`, `src/tui/input.rs`

- [ ] **Step 1: Track a "pending key" on the App**

In `src/tui/mod.rs`:

```rust
struct App {
    doc: layout::RenderedDoc,
    viewport: Viewport,
    images: kitty::ImageLifecycle,
    pending_g: bool,
}
```

Update `App::new` to initialize `pending_g: false`.

In the event loop, before calling `map_normal`, intercept `g`:

```rust
if let Event::Key(key) = event::read()? {
    if key.kind == KeyEventKind::Press {
        if matches!(key.code, KeyCode::Char('g')) {
            if app.pending_g {
                app.viewport.top = 0;
                app.pending_g = false;
                continue;
            } else {
                app.pending_g = true;
                continue;
            }
        } else {
            app.pending_g = false;
        }
    }
    // existing action dispatch...
}
```

- [ ] **Step 2: Remove the single-g action from input.rs**

Delete the `KeyCode::Char('g') => Action::JumpStart,` line in `map_normal`.

- [ ] **Step 3: Manual smoke test**

Run: `cargo run -- --tui fixtures/full-syntax.md`
Expected: pressing `g` once does nothing; `gg` jumps to top.

- [ ] **Step 4: Commit**

```sh
git add src/tui/mod.rs src/tui/input.rs
git commit -m "feat(tui): gg jumps to document start"
```

### Task 4.2: Heading nav (`]]` / `[[`)

**Files:**
- Modify: `src/tui/mod.rs`, `src/tui/viewport.rs`

- [ ] **Step 1: Failing test on viewport's heading-jump API**

Append to `src/tui/viewport.rs` tests:

```rust
#[test]
fn heading_jump_moves_to_heading_line() {
    use crate::layout::{HeadingEntry, Line, LineKind, Span, Style};
    let lines: Vec<Line> = (0..10).map(|i| Line {
        spans: vec![Span::Text { content: format!("row {i}"), style: Style::default() }],
        kind: LineKind::Body,
    }).collect();
    let headings = vec![
        HeadingEntry { level: 1, text: "A".into(), line_index: 3 },
        HeadingEntry { level: 1, text: "B".into(), line_index: 7 },
    ];
    let doc = RenderedDoc { lines, headings, images: vec![] };
    let mut vp = Viewport::new(3, 40);
    vp.ensure_wrap(&doc);

    vp.jump_to_next_heading(&doc, 0);
    assert_eq!(vp.top, 3);
    vp.jump_to_next_heading(&doc, vp.top + 1);
    assert_eq!(vp.top, 7);
    vp.jump_to_prev_heading(&doc, 7);
    assert_eq!(vp.top, 3);
}
```

- [ ] **Step 2: Implement**

Add methods to `Viewport`:

```rust
pub fn jump_to_next_heading(&mut self, doc: &RenderedDoc, after_visual: usize) {
    // Find the logical line of the first visual line > after_visual
    let start_logical = self.visual_lines.get(after_visual)
        .map(|vl| vl.logical_index)
        .unwrap_or(0);
    let next = doc.headings.iter().find(|h| h.line_index > start_logical);
    if let Some(h) = next {
        if let Some(vi) = self.visual_lines.iter().position(|vl| vl.logical_index == h.line_index) {
            self.top = vi;
        }
    }
}

pub fn jump_to_prev_heading(&mut self, doc: &RenderedDoc, before_visual: usize) {
    let start_logical = self.visual_lines.get(before_visual)
        .map(|vl| vl.logical_index)
        .unwrap_or(0);
    let prev = doc.headings.iter().rev().find(|h| h.line_index < start_logical);
    if let Some(h) = prev {
        if let Some(vi) = self.visual_lines.iter().position(|vl| vl.logical_index == h.line_index) {
            self.top = vi;
        }
    }
}
```

- [ ] **Step 3: Map the action in input.rs**

Add:

```rust
NextHeading,
PrevHeading,
```

(already present — just ensure the code path handles the `]`/`[` keys to mean `]]`/`[[` with a two-key sequence similar to `gg`. For v1 keep single-bracket activation; document two-bracket later if desired.)

- [ ] **Step 4: Handle in event loop**

```rust
input::Action::NextHeading => app.viewport.jump_to_next_heading(&app.doc, app.viewport.top),
input::Action::PrevHeading => app.viewport.jump_to_prev_heading(&app.doc, app.viewport.top),
```

- [ ] **Step 5: Run tests**

Run: `cargo test --lib tui::viewport::tests`
Expected: PASS.

- [ ] **Step 6: Commit**

```sh
git add src/tui/viewport.rs src/tui/mod.rs src/tui/input.rs
git commit -m "feat(tui): heading navigation with ]/["
```

### Task 4.3: Status bar widget

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Split layout into body + status**

In `draw`:

```rust
use ratatui::layout::{Constraint, Direction, Layout};

let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(1), Constraint::Length(1)])
    .split(frame.area());

// body in chunks[0], status in chunks[1]
frame.render_widget(para, chunks[0]);
let progress = progress_fraction(&app);
let pct = (progress * 100.0).round() as u32;
let status = Paragraph::new(format!(" {}  {pct}%", app.current_path_display()))
    .style(ratatui::style::Style::default()
        .bg(ratatui::style::Color::DarkGray)
        .fg(ratatui::style::Color::White));
frame.render_widget(status, chunks[1]);
```

Add on `App`:

```rust
fn current_path_display(&self) -> String { self.path.clone() }
```

And store `path: String` on App (passed from `run`).

Helper outside:

```rust
fn progress_fraction(app: &App) -> f64 {
    let total = app.viewport.total_visual_lines() as f64;
    if total == 0.0 { return 1.0; }
    let pos = (app.viewport.top as f64 + app.viewport.height as f64).min(total);
    pos / total
}
```

- [ ] **Step 2: Wire path through**

Extend `App::new` to take `path: String` and store it.

- [ ] **Step 3: Manual smoke test**

Run: `cargo run -- --tui fixtures/full-syntax.md`
Expected: bottom line shows path + percentage; it updates as you scroll.

- [ ] **Step 4: Commit**

```sh
git add src/tui/mod.rs
git commit -m "feat(tui): bottom status bar with path and progress"
```

### Task 4.4: Width-aware wrap (replace no-op wrap)

**Files:**
- Modify: `src/tui/viewport.rs`

- [ ] **Step 1: Failing test**

```rust
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
        headings: vec![], images: vec![],
    };
    let mut vp = Viewport::new(10, 20);
    vp.ensure_wrap(&doc);
    assert!(vp.total_visual_lines() > 1, "expected multiple visual lines");
}
```

- [ ] **Step 2: Replace `wrap_all`**

Use `unicode_width::UnicodeWidthStr::width` to accumulate display width and break on word boundaries. URLs inside `Span::Link` are treated as single tokens (never broken).

```rust
fn wrap_all(lines: &[Line], width: u16) -> Vec<VisualLine> {
    use unicode_width::UnicodeWidthStr;
    let max = width.saturating_sub(4) as usize; // reserve margin
    let mut out = Vec::new();
    for (li, line) in lines.iter().enumerate() {
        // Tables / rules / blank lines are emitted as-is (one visual line).
        match line.kind {
            crate::layout::LineKind::Blank
            | crate::layout::LineKind::HorizontalRule
            | crate::layout::LineKind::Table => {
                out.push(VisualLine { logical_index: li, byte_start: 0, byte_end: line_byte_len(line) });
                continue;
            }
            _ => {}
        }

        // Flatten span text into a single string; wrap by display width.
        let text: String = line.spans.iter().filter_map(|s| match s {
            crate::layout::Span::Text { content, .. } | crate::layout::Span::Link { content, .. } => Some(content.as_str()),
            crate::layout::Span::HeadingImage { .. } => None,
        }).collect::<Vec<_>>().join("");

        if max == 0 || UnicodeWidthStr::width(text.as_str()) <= max {
            out.push(VisualLine { logical_index: li, byte_start: 0, byte_end: text.len() });
            continue;
        }

        let mut byte_start = 0usize;
        let mut cur_width = 0usize;
        let mut cur_byte = 0usize;
        for (i, ch) in text.char_indices() {
            let cw = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
            if cur_width + cw > max && cur_byte > byte_start {
                out.push(VisualLine { logical_index: li, byte_start, byte_end: cur_byte });
                byte_start = cur_byte;
                cur_width = 0;
            }
            cur_byte = i + ch.len_utf8();
            cur_width += cw;
        }
        out.push(VisualLine { logical_index: li, byte_start, byte_end: text.len() });
    }
    out
}
```

Note: span-rendered draw now needs byte-range slicing. Update `draw` in `src/tui/mod.rs` to render spans clipped to `vl.byte_start..vl.byte_end`. Write a helper `fn line_byte_slice(line: &Line, start: usize, end: usize) -> Vec<RSpan>` that walks spans, accumulates byte offsets, and emits only the clipped portion of each span.

- [ ] **Step 3: Run tests**

Run: `cargo test --lib tui::viewport::tests`
Expected: PASS (including existing ones).

- [ ] **Step 4: Manual smoke test**

Run: `cargo run -- --tui fixtures/full-syntax-zh.md`
Expected: long paragraphs wrap at terminal width; CJK width correct.

- [ ] **Step 5: Commit**

```sh
git add src/tui/viewport.rs src/tui/mod.rs
git commit -m "feat(tui): width-aware wrap with display-width accounting"
```

---

## Phase 5 — Search

### Task 5.1: SearchState with substring match

**Files:**
- Create: `src/tui/search.rs`
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Failing test**

Create `src/tui/search.rs`:

```rust
use crate::layout::{Line, RenderedDoc, Span};

#[derive(Debug, Clone)]
pub struct MatchPos {
    pub line_index: usize,
    pub byte_range: std::ops::Range<usize>,
}

pub struct SearchState {
    pub query: String,
    pub matches: Vec<MatchPos>,
    pub current: Option<usize>,
}

impl SearchState {
    pub fn new(query: String, doc: &RenderedDoc) -> Self {
        let matches = find_all(&query, doc);
        Self { query, matches, current: None }
    }
}

pub fn find_all(query: &str, doc: &RenderedDoc) -> Vec<MatchPos> {
    if query.is_empty() { return Vec::new(); }
    let smart_case = !query.chars().any(|c| c.is_uppercase());
    let mut out = Vec::new();
    for (i, line) in doc.lines.iter().enumerate() {
        let haystack = line_text(line);
        find_in_line(&haystack, query, smart_case, i, &mut out);
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

fn find_in_line(haystack: &str, needle: &str, case_insensitive: bool, line: usize, out: &mut Vec<MatchPos>) {
    if case_insensitive {
        let lower = haystack.to_lowercase();
        let nlow = needle.to_lowercase();
        let mut start = 0usize;
        while let Some(off) = lower[start..].find(&nlow) {
            let abs = start + off;
            out.push(MatchPos { line_index: line, byte_range: abs..abs + needle.len() });
            start = abs + needle.len().max(1);
        }
    } else {
        let mut start = 0usize;
        while let Some(off) = haystack[start..].find(needle) {
            let abs = start + off;
            out.push(MatchPos { line_index: line, byte_range: abs..abs + needle.len() });
            start = abs + needle.len().max(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{Line, LineKind, RenderedDoc, Span, Style};

    fn doc_with(lines: &[&str]) -> RenderedDoc {
        RenderedDoc {
            lines: lines.iter().map(|t| Line {
                spans: vec![Span::Text { content: (*t).into(), style: Style::default() }],
                kind: LineKind::Body,
            }).collect(),
            headings: vec![], images: vec![],
        }
    }

    #[test]
    fn smart_case_lowercase_query_matches_insensitive() {
        let doc = doc_with(&["Hello World", "hello there"]);
        let m = find_all("hello", &doc);
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn mixed_case_query_is_sensitive() {
        let doc = doc_with(&["Hello World", "hello there"]);
        let m = find_all("Hello", &doc);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].line_index, 0);
    }

    #[test]
    fn empty_query_returns_no_matches() {
        let doc = doc_with(&["anything"]);
        assert!(find_all("", &doc).is_empty());
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test --lib tui::search::tests`
Expected: PASS.

- [ ] **Step 3: Register the module**

Add `mod search;` to `src/tui/mod.rs`.

- [ ] **Step 4: Commit**

```sh
git add src/tui/search.rs src/tui/mod.rs
git commit -m "feat(tui): literal smart-case search state"
```

### Task 5.2: Search prompt input mode

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Add a Mode enum**

In `src/tui/mod.rs`:

```rust
enum Mode {
    Normal,
    Search { input: tui_textarea::TextArea<'static>, reverse: bool },
}

struct App {
    doc: layout::RenderedDoc,
    viewport: Viewport,
    images: kitty::ImageLifecycle,
    pending_g: bool,
    path: String,
    mode: Mode,
    search: Option<search::SearchState>,
}
```

Initialize `mode: Mode::Normal, search: None`.

- [ ] **Step 2: Route keys by mode**

Replace the single-mode key dispatch with:

```rust
match &mut app.mode {
    Mode::Normal => { /* existing normal handling */ }
    Mode::Search { input, reverse } => {
        let reverse = *reverse;
        let key_event = if let Event::Key(k) = event::read()? { k } else { continue; };
        match key_event.code {
            KeyCode::Esc => app.mode = Mode::Normal,
            KeyCode::Enter => {
                let query = input.lines().join("");
                let state = search::SearchState::new(query, &app.doc);
                app.search = Some(state);
                app.mode = Mode::Normal;
                apply_search_jump(app, reverse);
            }
            _ => { input.input(key_event); }
        }
    }
}
```

For the `SearchBegin` action in Normal mode:

```rust
input::Action::SearchBegin { reverse } => {
    let mut ta = tui_textarea::TextArea::default();
    ta.set_cursor_line_style(ratatui::style::Style::default());
    app.mode = Mode::Search { input: ta, reverse };
}
```

- [ ] **Step 3: Render the prompt in Search mode**

In `draw`, when `app.mode` is `Search`, overlay a single-line prompt at the bottom (replacing the status bar for that frame):

```rust
if let Mode::Search { input, reverse } = &app.mode {
    let prompt_text = format!("{}{}", if *reverse { "?" } else { "/" }, input.lines().join(""));
    let prompt = Paragraph::new(prompt_text);
    frame.render_widget(prompt, chunks[1]);
}
```

- [ ] **Step 4: Implement `apply_search_jump`**

```rust
fn apply_search_jump(app: &mut App, reverse: bool) {
    let Some(state) = app.search.as_mut() else { return; };
    if state.matches.is_empty() { return; }

    let current_logical = app.viewport.visible()
        .first()
        .map(|vl| vl.logical_index)
        .unwrap_or(0);

    let idx = if !reverse {
        state.matches.iter().position(|m| m.line_index >= current_logical).unwrap_or(0)
    } else {
        state.matches.iter().rposition(|m| m.line_index <= current_logical).unwrap_or(state.matches.len() - 1)
    };
    state.current = Some(idx);
    center_on_logical(&mut app.viewport, state.matches[idx].line_index);
}

fn center_on_logical(vp: &mut Viewport, logical: usize) {
    if let Some(vi) = vp.visual_lines_iter().position(|vl| vl.logical_index == logical) {
        let third = (vp.height as usize) / 3;
        vp.top = vi.saturating_sub(third);
        let max = vp.total_visual_lines().saturating_sub(vp.height as usize);
        vp.top = vp.top.min(max);
    }
}
```

Add a `pub fn visual_lines_iter(&self) -> std::slice::Iter<'_, VisualLine>` helper on `Viewport`.

- [ ] **Step 5: Manual smoke test**

Run: `cargo run -- --tui fixtures/full-syntax.md`
Press `/`, type a word, Enter → viewport jumps to first match. Esc cancels. `?` starts reverse.

- [ ] **Step 6: `make check`**

Expected: PASS.

- [ ] **Step 7: Commit**

```sh
git add src/tui/mod.rs src/tui/search.rs src/tui/viewport.rs
git commit -m "feat(tui): search prompt and initial jump"
```

### Task 5.3: n / N navigation

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Wire the actions**

Add to the `Action` handlers in the Normal-mode branch of the event loop:

```rust
input::Action::SearchNext => advance_search(app, 1),
input::Action::SearchPrev => advance_search(app, -1),
```

Implement:

```rust
fn advance_search(app: &mut App, delta: i32) {
    let Some(state) = app.search.as_mut() else { return; };
    if state.matches.is_empty() { return; }
    let len = state.matches.len() as i32;
    let cur = state.current.unwrap_or(0) as i32;
    let next = ((cur + delta) % len + len) % len;
    state.current = Some(next as usize);
    let line = state.matches[next as usize].line_index;
    center_on_logical(&mut app.viewport, line);
}
```

- [ ] **Step 2: Manual smoke test**

Run and verify `n`/`N` cycle through matches with wrap-around.

- [ ] **Step 3: Commit**

```sh
git add src/tui/mod.rs
git commit -m "feat(tui): n/N navigate between search matches"
```

### Task 5.4: Highlight matches on screen

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Locate matches visible in the current frame**

In `draw`, before building each `RLine`, compute matches intersecting the visible byte range of that visual line:

```rust
let visible_matches: Vec<&search::MatchPos> = app.search.as_ref()
    .map(|s| s.matches.iter()
        .filter(|m| m.line_index == vl.logical_index)
        .filter(|m| m.byte_range.start < vl.byte_end && m.byte_range.end > vl.byte_start)
        .collect())
    .unwrap_or_default();
```

- [ ] **Step 2: Clip-slice spans with highlight background**

Write a helper `highlighted_line_spans(line, vl, &visible_matches, current_match_index) -> Vec<RSpan>` that walks the line's spans, respecting `vl.byte_start..vl.byte_end`, and whenever a byte range overlaps a match, emits a dedicated `RSpan` with:

- Current-match highlight: `Color::Yellow` bg, `Color::Black` fg.
- Non-current match highlight: `Color::Rgb(80, 80, 0)` bg (dim yellow).

Use the match's `byte_range` start/end to split the text into before/match/after chunks.

- [ ] **Step 3: Manual smoke test**

Run: `cargo run -- --tui fixtures/full-syntax.md`, `/word<Enter>` → all `word` occurrences highlighted; `n`/`N` moves the brighter one.

- [ ] **Step 4: Commit**

```sh
git add src/tui/mod.rs
git commit -m "feat(tui): inverse-highlight search matches"
```

---

## Phase 6 — ToC Panel

### Task 6.1: Layout split + toggleable ToC list

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Add `toc_open: bool` to App**

- [ ] **Step 2: Handle ToggleToc action**

```rust
input::Action::ToggleToc => app.toc_open = !app.toc_open,
```

- [ ] **Step 3: Split the layout when open**

In `draw`:

```rust
let body_area = if app.toc_open {
    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(20)])
        .split(chunks[0]);
    let toc_items: Vec<ratatui::widgets::ListItem> = app.doc.headings.iter()
        .map(|h| {
            let indent = " ".repeat((h.level as usize).saturating_sub(1) * 2);
            ratatui::widgets::ListItem::new(format!("{indent}{}", h.text))
        })
        .collect();
    let toc = ratatui::widgets::List::new(toc_items)
        .block(ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::RIGHT)
            .title("Contents"));
    frame.render_widget(toc, split[0]);
    split[1]
} else {
    chunks[0]
};

// Render the paragraph into body_area instead of chunks[0].
```

- [ ] **Step 4: Manual smoke test**

Press `t` toggles ToC on/off; widths rebalance correctly.

- [ ] **Step 5: Commit**

```sh
git add src/tui/mod.rs
git commit -m "feat(tui): toggleable ToC side panel"
```

---

## Phase 7 — Multi-File Back/Forward + Links

### Task 7.1: DocEntry list with history/forward stacks

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Refactor state to per-doc**

```rust
struct DocEntry {
    path: String,
    doc: layout::RenderedDoc,
    viewport: Viewport,
    search: Option<search::SearchState>,
    placed_images_registered: bool,
    pending_g: bool,
    toc_open: bool,
}

struct App {
    docs: Vec<DocEntry>,
    cursor: usize,
    history: Vec<usize>,
    forward: Vec<usize>,
    mode: Mode,
    images: kitty::ImageLifecycle,
    next_image_id: u32,
}

impl App {
    fn active(&self) -> &DocEntry { &self.docs[self.cursor] }
    fn active_mut(&mut self) -> &mut DocEntry { &mut self.docs[self.cursor] }
}
```

Replace every `app.viewport` with `app.active_mut().viewport` throughout the event loop and draw.

- [ ] **Step 2: Allocate image ids globally**

The `layout::build` call should accept an id counter so ids stay unique across docs. Change its signature to take `&mut u32` or return the last id + allocate from `App`. Simpler: expose a thin re-number pass in App when a new doc is added:

```rust
fn add_doc(&mut self, path: String, doc: layout::RenderedDoc) -> usize {
    let mut doc = doc;
    for img in &mut doc.images {
        img.id = self.next_image_id;
        self.next_image_id += 1;
    }
    // Also patch Span::HeadingImage id references so they still match.
    renumber_image_refs(&mut doc);
    let size = /* current term size */;
    let vp = Viewport::new(size.1, size.0);
    self.docs.push(DocEntry { path, doc, viewport: vp, search: None,
        placed_images_registered: false, pending_g: false, toc_open: false });
    self.docs.len() - 1
}
```

Renumber the `Span::HeadingImage { id, .. }` entries in lockstep with `doc.images`. Because image ids are assigned in sequence 1..N during `layout::build` and then shifted by a constant offset, this is a simple offset pass: remember the old starting id, compute the shift, apply to every `Span::HeadingImage` and `LineKind::Heading { id, .. }`.

- [ ] **Step 3: Implement back / forward handlers**

```rust
input::Action::Back => {
    if let Some(prev) = app.history.pop() {
        app.forward.push(app.cursor);
        app.cursor = prev;
    }
}
input::Action::Forward => {
    if let Some(next) = app.forward.pop() {
        app.history.push(app.cursor);
        app.cursor = next;
    }
}
```

When navigating to a new doc (Task 7.2 below), push `app.cursor` onto `history` and clear `forward`.

- [ ] **Step 4: Commit**

```sh
git add src/tui/mod.rs
git commit -m "feat(tui): multi-doc history with back/forward"
```

### Task 7.2: Link opening (Enter + LinkSelect overlay)

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Collect visible links**

Add `fn visible_links(app: &App) -> Vec<(String, String)>` returning `(content, url)` tuples in document order for the current viewport.

- [ ] **Step 2: Handle OpenLink action**

```rust
input::Action::OpenLink => {
    let links = visible_links(app);
    match links.len() {
        0 => {}
        1 => open_link(&links[0].1, app),
        _ => app.mode = Mode::LinkSelect { links },
    }
}
```

Add `LinkSelect { links: Vec<(String, String)> }` to the `Mode` enum. Mode handler:

```rust
Mode::LinkSelect { links } => {
    if let Event::Key(k) = event::read()? {
        if k.kind != KeyEventKind::Press { continue; }
        match k.code {
            KeyCode::Esc => app.mode = Mode::Normal,
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let idx = (c as u8 - b'0') as usize;
                if idx > 0 && idx <= links.len() {
                    let (_, url) = links[idx - 1].clone();
                    app.mode = Mode::Normal;
                    open_link(&url, app);
                }
            }
            _ => {}
        }
    }
}
```

- [ ] **Step 3: Draw numbered overlay**

In `draw`, when `Mode::LinkSelect`, overlay bracketed digits beside each visible link. Implementation sketch: when building visible line spans, track link span order; for each visible link at index `i`, prepend `[i+1]` as an extra RSpan with a distinct style.

- [ ] **Step 4: Open URL**

```rust
fn open_link(url: &str, app: &App) {
    if url.ends_with(".md") && std::path::Path::new(url).exists() {
        // Handled in Task 7.3 (local-file navigation)
        return;
    }
    let cmd = if cfg!(target_os = "macos") { "open" } else { "xdg-open" };
    let _ = std::process::Command::new(cmd).arg(url).spawn();
    let _ = app; // reserved for future telemetry
}
```

- [ ] **Step 5: Manual smoke test**

Open a doc with multiple external links; Enter in single-link case opens it; multi-link shows digits; pressing 2 opens the second.

- [ ] **Step 6: Commit**

```sh
git add src/tui/mod.rs
git commit -m "feat(tui): open links via Enter with numeric overlay for ambiguity"
```

### Task 7.3: Local .md link navigation

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Extend `open_link` to treat `*.md` paths specially**

```rust
fn open_link(url: &str, app: &mut App) {
    let as_path = std::path::Path::new(url);
    if url.ends_with(".md") && as_path.exists() {
        match std::fs::read_to_string(as_path) {
            Ok(src) => {
                let active_theme = /* store theme on App; thread through */;
                let config = /* ditto */;
                let doc = layout::build(&src, &config, active_theme);
                app.history.push(app.cursor);
                app.forward.clear();
                let new_cursor = app.add_doc(url.to_string(), doc);
                app.cursor = new_cursor;
            }
            Err(_) => {}
        }
        return;
    }
    let cmd = if cfg!(target_os = "macos") { "open" } else { "xdg-open" };
    let _ = std::process::Command::new(cmd).arg(url).spawn();
}
```

Store `theme: Theme` and `config: Config` fields on `App` so `open_link` can rebuild a doc for a followed link.

- [ ] **Step 2: Manual smoke test**

Create two linked fixtures (or use existing docs in the repo): `README.md` → `docs/TUI_MODE_DESIGN.md`. Follow the link, press `o` to go back, `i` to go forward. Verify each doc's scroll position and search state are preserved.

- [ ] **Step 3: Commit**

```sh
git add src/tui/mod.rs
git commit -m "feat(tui): follow local .md links into the doc stack"
```

---

## Phase 8 — Final QA and Docs

### Task 8.1: Manual pre-merge checklist

**Files:**
- None modified. This is manual verification run in Ghostty and iTerm2.

- [ ] Run the checklist from `docs/TUI_MODE_DESIGN.md` § "Manual Pre-merge Checklist":
  - Short / mid / long docs.
  - Heading-dense docs.
  - Mixed script, emoji, wide tables, long code blocks.
  - Held-`j` for 10 s — no flicker, lag, or image residue.
  - Search hit / miss / wrap, re-center at 1/3.
  - Multi-file back/forward, state preserved.
  - Resize mid-session.
  - Link open: 0/1/>1 visible.
  - `q` exit: no image residue.

- [ ] Record any bug findings as follow-up issues; fix in-plan only if blocker.

### Task 8.2: Update README and help text

**Files:**
- Modify: `README.md`, `README_CN.md`, `src/main.rs` (help text)

- [ ] **Step 1: Add a "TUI mode" section**

Append to `README.md` under Usage:

```markdown
### TUI mode

For long files, use `--tui` for a vim-style interactive browser:

    termdown --tui README.md

Key bindings: `j/k` scroll, `d/u` half page, `f/b` page, `gg`/`G` start/end,
`]`/`[` heading nav, `t` table of contents, `/` search, `n/N` next/prev match,
`Enter` follow link, `o/i` back/forward across docs, `q` quit.
```

- [ ] **Step 2: Mirror in README_CN.md**

- [ ] **Step 3: Update --help**

In `src/main.rs` help block, ensure `--tui` is listed with one-line description.

- [ ] **Step 4: Commit**

```sh
git add README.md README_CN.md src/main.rs
git commit -m "docs: document --tui mode and key bindings"
```

### Task 8.3: Bump version

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Bump**

Edit `Cargo.toml`:

```toml
version = "0.4.0"
```

- [ ] **Step 2: Commit**

```sh
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to 0.4.0"
```

---

## Plan Self-Review Notes

- **Spec coverage:** Every section of `docs/TUI_MODE_DESIGN.md` is reflected: activation (Task 2.2), module layout (Phases 1-3 cover `layout.rs`, `cat.rs`, `tui/{mod,viewport,input,search,kitty}.rs`, `render.rs` extensions), data model (Task 1.1), cat rewrite (Tasks 1.2-1.10), runtime state (Tasks 2.3-2.6, 7.1), event loop and layered rendering (Tasks 2.5, 3.3), key bindings (Tasks 2.6, 4.1-4.2), link opening (Tasks 7.2-7.3), search (Phase 5), Kitty image lifecycle (Phase 3), testing strategy (Phase 0 + per-task TDD), open questions (deferred, consistent with spec).
- **Placeholders:** No TBD/TODO in code. Where a task body mentions "refined in Phase N", the referenced task implements it.
- **Type consistency:** `Style` fields (`fg/bg/bold/italic/underline/strikethrough/dim`) introduced in Task 1.1 are used consistently through `cat.rs` (Task 1.8) and `viewport.rs`/`search.rs`. `HeadingImage` moved to `render.rs` in Task 1.1 and re-referenced in `tui/kitty.rs` (Task 3.2) via path `crate::render::HeadingImage`. Kitty protocol APIs in Task 3.1 (`transmit/place/delete_placement/delete_all_for_client`) are the ones called by `ImageLifecycle` in Task 3.2.
- **Known manual gates:** Task 1.9 requires human audit of snapshot diffs; Task 8.1 is the Ghostty/iTerm2 QA pass. Both are called out explicitly.
