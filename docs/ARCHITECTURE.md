# Architecture

## Overview

termdown has **two output paths that share one rendering core**. `layout::build`
parses Markdown into a `RenderedDoc` — a structured, terminal-agnostic line/span
model with heading images already rasterized. The **cat** path streams that doc
to stdout once and exits; the **TUI** path drives it as an interactive,
scrollable, vim-style pager. Both consume the same `RenderedDoc`, so wrapping,
styling, and heading rendering never fork.

```
                        ┌──────────────────────┐
                        │    Markdown source    │
                        └───────────┬──────────┘
                                    │
                        ┌───────────▼───────────┐
                        │     layout::build      │  pulldown-cmark events →
                        │     → RenderedDoc       │  lines + spans + heading
                        │   (rayon-parallel       │  PNGs + frontmatter
                        │    heading PNGs)         │
                        └───────────┬───────────┘
                                    │
                      ┌─────────────┴─────────────┐
                      │                            │
             ┌────────▼────────┐        ┌──────────▼─────────┐
             │   cat::print     │        │      tui::run      │
             │  ANSI → stdout   │        │  ratatui pager +   │
             │   (one shot)     │        │  Kitty image       │
             │                  │        │  lifecycle         │
             └─────────────────┘        └────────────────────┘
```

### Mode dispatch (`main.rs`)

TUI is the **default**. `main` selects the path:

- **TUI** when a real file path is given *and* stdout is a TTY *and* `--cat` is
  absent.
- **cat** otherwise — `--cat`, piped/redirected stdout (`termdown foo.md | less`),
  or stdin input (`-` or no argument).

`main.rs` also parses `--theme` / `--no-bell` / `--help` / `--version`, resolves
the theme (CLI flag > config file > OSC 11 auto-detect), warns on terminals
unlikely to support Kitty graphics, and manages UNIX terminal echo state for cat
mode.

## Module Overview

```
src/
├── main.rs        CLI entry: arg parsing, mode dispatch, theme resolution,
│                  terminal-support warning, UNIX termios echo handling
├── config.rs      XDG config load (~/.config/termdown/config.toml);
│                  theme/bell/metadata/font options; legacy-path migration warning
├── theme.rs       Theme {Dark, Light} + OSC 11 background auto-detection
├── style.rs       HeadingStyle, ANSI constants, theme-aware Colors palette,
│                  strip_ansi / display_width helpers
├── font.rs        Latin/CJK/emoji font resolution + per-level cache, is_cjk()
├── frontmatter.rs YAML (---) / TOML (+++) metadata-block heuristic parser + summary
├── render.rs      Glyph rasterization, PNG encoding, Kitty protocol primitives
│                  (transmit / place / delete), HeadingImage
├── layout.rs      ★ Shared core: pulldown-cmark → RenderedDoc (Line / Span / Style)
├── cat.rs         RenderedDoc → stdout ANSI stream (cat path)
└── tui/           Interactive pager (default mode)
    ├── mod.rs       App state, doc stack, event loop, frame rendering
    ├── input.rs     KeyEvent → Action mapping
    ├── viewport.rs  Scroll offset + width-aware wrap cache
    ├── search.rs    Smart-case literal substring search over RenderedDoc
    └── kitty.rs     Transmit-once + per-frame placement-diff image lifecycle
```

## The RenderedDoc model (`layout.rs`)

`layout::build(md, config, theme)` is the single Markdown→structure step. It runs
pulldown-cmark with GFM strikethrough, tables, and YAML/TOML metadata-block
extensions enabled, and produces:

```
RenderedDoc
├── lines:    Vec<Line>            // each Line = Vec<Span> + LineKind
├── headings: Vec<HeadingEntry>    // ToC / heading-jump targets (level, text, line_index)
├── images:   Vec<HeadingImage>    // rasterized H1–H3 PNGs, referenced by id from spans
└── metadata: Option<MetadataInfo> // parsed frontmatter (never leaks into `lines`)
```

- **`LineKind`** classifies each line: `Body`, `Heading{level, id}`,
  `CodeBlock{lang}`, `BlockQuote{depth}`, `ListItem{depth}`, `Table`,
  `HorizontalRule`, `Blank`. `id` is `Some` for H1–H3 (image) and `None` for
  H4–H6 (ANSI bold text).
- **`Span`** is `Text{content, style}`, `Link{content, url, style}`, or
  `HeadingImage{id, rows}`. Styling is structural — `Style{fg, bg, bold, italic,
  underline, strikethrough, dim}` over `Color::Indexed | Rgb` — so the same doc
  can be emitted as ANSI (cat) or painted as ratatui spans (TUI) without
  re-parsing.
- **Heading images** are rasterized during `build` in parallel via rayon
  (`par_iter` over heading text → `render::render_heading`); rasterization is the
  dominant cost in a document.

## Rendering Pipeline (heading image)

H1–H3 headings become PNGs through this sub-pipeline; everything else stays ANSI
text.

```
          ┌────────────────────┐
          │  Heading H1/H2/H3  │
          └─────────┬──────────┘
                    │ per-character routing
          ┌─────────▼──────────────────────┐
          │ is_emoji_like(ch) → emoji font  │
          │ is_cjk(ch)        → CJK font    │
          │ else              → Latin font  │
          └─────────┬──────────────────────┘
                    │
          ┌─────────▼─────────┐
          │  ab_glyph         │
          │  rasterize → RGBA │
          └─────────┬─────────┘
                    │
          ┌─────────▼─────────┐
          │  PNG encode       │
          └─────────┬─────────┘
                    │
          ┌─────────▼─────────┐
          │  Kitty graphics   │
          └───────────────────┘
```

## Font Resolution

For each heading level a `FontSet` (Latin + CJK + optional emoji) is resolved and
cached for the process lifetime — resolution is ~30–40 ms per font on macOS, so
it is memoized per level:

```
1. User config  [font.heading] latin / cjk / emoji
       │
       ▼
2. Explicit weight-variant family names (macOS workaround)
   Core Text registers bold variants as separate families, so try
   "{family} Black" / "{family} Heavy" before standard matching
       │
       ▼
3. Standard weight matching
   font-kit select_best_match with Weight::BLACK / EXTRA_BOLD / BOLD
       │
       ▼
4. Platform defaults
   Latin: Avenir, Avenir Next, Futura, Helvetica Neue    (macOS)
          Inter, Noto Sans, DejaVu Sans, Liberation Sans (Linux)
          Segoe UI, Arial, Verdana                       (Windows)
   CJK:   Noto Serif CJK SC, Source Han Serif SC, …       (per platform)
   Emoji: Apple Color Emoji (macOS) / Noto Color Emoji (Linux) /
          Segoe UI Emoji (Windows)
       │
       ▼
5. Embedded fallback
   fonts/SourceSerif4-SemiBold.ttf (bundled in binary via include_bytes!)
```

Font data loaded from disk or Core Text is `Box::leak`-ed into `'static`
lifetime and cached in a global map to avoid repeated allocation.

## CJK / Latin / Emoji split

`font::is_cjk(ch)` routes characters to the CJK font by Unicode block;
`font::is_emoji_like(ch)` routes emoji and symbol glyphs to the emoji font
(rasterized as color bitmaps). Everything else (ASCII, Latin, Cyrillic, …) uses
the Latin font.

| Range | Block |
|-------|-------|
| U+2E80..U+9FFF | CJK Radicals through Unified Ideographs (includes Hiragana, Katakana, CJK Symbols) |
| U+AC00..U+D7AF | Hangul Syllables |
| U+F900..U+FAFF | CJK Compatibility Ideographs |
| U+FE30..U+FE4F | CJK Compatibility Forms |
| U+FF00..U+FFEF | Halfwidth and Fullwidth Forms |
| U+20000..U+2FA1F | CJK Extensions B–F, Supplement |

## Kitty Graphics Protocol

termdown emits heading PNGs two ways depending on the path.

**cat path — transmit-and-display inline.** A single `a=T` run transmits and
immediately displays at the cursor:

```
\x1b_G f=100,a=T,q=2,m=1 ; <base64 chunk 1> \x1b\
\x1b_G m=1 ; <base64 chunk 2> \x1b\
...
\x1b_G m=0 ; <base64 chunk N> \x1b\
```

- `f=100` — PNG format
- `a=T` — transmit and display
- `q=2` — suppress response (avoids the iTerm2 "OK" leak)
- `m=1/0` — more chunks / last chunk
- Chunk size: 4096 bytes base64

**TUI path — transmit once, place/delete per frame.** `render::transmit` (`a=t`)
uploads each PNG to the terminal exactly once, keyed by id. On each frame
`tui::kitty::ImageLifecycle` diffs the desired placement map against what is
currently placed and emits the minimum `place` (`a=p`, with `C=1` so the cursor
does not advance and scroll the screen) / `delete_placement` commands;
`delete_all_for_client` cleans up at exit. This avoids the per-frame PNG
re-transmission that makes similar tools feel sluggish.

## ANSI Text Rendering (cat path)

`cat::print` streams the `RenderedDoc` to stdout, wrapping to terminal width and
emitting Kitty heading images inline. Rendering is driven by each line's
`LineKind` / `Span`:

| Element | Rendering |
|---------|-----------|
| H1–H3 | PNG via Kitty graphics |
| H4–H6 | Bold ANSI text |
| Paragraphs | Word-wrapped to terminal width |
| Ordered lists | Numbered with counter per nesting level |
| Unordered lists | Bullet (•) with indent per level |
| Blockquotes | Vertical bar (│) per nesting depth, italic gray |
| Code blocks | Buffered and padded to uniform width for a clean background rectangle |
| Inline code | Pink on dark gray |
| Links | Colored + underline, with the URL shown |
| Tables | Unicode box-drawing, ANSI-aware column width |
| Horizontal rule | ─ repeated to terminal width |
| Images | Placeholder with alt text |
| Frontmatter | Dim one-line summary `[metadata · k=v, …]` (when `metadata` enabled) |

## TUI Mode (`tui/`)

Interactive pager built on ratatui + crossterm. The body is painted as a ratatui
text layer; heading images float above it via the Kitty placement lifecycle.

- **Modes:** `Normal`, `Search{…}`, `LinkSelect{…}`, `Help`. `input::map_normal`
  turns key events into intent-level `Action`s; `mod.rs` dispatches them to state
  mutations.
- **Navigation:** vim-style paging, `gg` / `G`, heading jumps, `/` search with
  `n` / `N`. Search is smart-case literal substring matching (`search.rs`); regex
  is deferred to a future version.
- **Document stack:** following a local `.md` link pushes a new `DocEntry`;
  back/forward keys pop/replay the stack, each doc preserving its own scroll
  position and search state.
- **Viewport** (`viewport.rs`): scroll offset plus a width-aware wrap cache
  (`VisualLine`s), including synthetic rows for the foldable metadata box.
- **Table of contents:** a side panel built from `RenderedDoc.headings`.
- **Edge bell:** a terminal BEL on blocked scroll past the top/bottom (vim-style),
  disabled via `--no-bell` or `bell = false`. The visible effect (beep, title-bar
  🔔, dock bounce) is the emulator's own response to BEL, not something termdown
  paints.
- **Metadata box:** the frontmatter summary folds/expands inline (`m`).

## Terminal State (UNIX, cat mode)

iTerm2 ignores Kitty's `q=2` response-suppression flag and emits `OK` ACKs
anyway. So on UNIX, **only under iTerm2** (`TERM_PROGRAM == iTerm.app`),
`main.rs` disables `ECHO` before rendering and restores it after, then
`render::drain_iterm2_acks` waits briefly and discards the leaked bytes. Other
terminals (Ghostty, Kitty, WezTerm) respect `q=2` and are left untouched —
notably so termdown does not trip Ghostty's Secure Keyboard Entry heuristic,
which treats `~ECHO` as a password prompt. Guarded by `#[cfg(unix)]`.

## Configuration

Loaded once at startup from `~/.config/termdown/config.toml` (XDG: an absolute
`$XDG_CONFIG_HOME` is honored, otherwise `~/.config`). A config still sitting at
the legacy `~/.termdown/config.toml` triggers a one-line migration warning.
Unknown keys and invalid values are hard errors surfaced as warnings, not silent
fallbacks.

```
Config
├── theme:    Option<ThemeChoice>  // auto (default) | dark | light;  CLI --theme overrides
├── bell:     Option<bool>         // edge-scroll BEL, default on;     CLI --no-bell overrides
├── metadata: Option<bool>         // render frontmatter, default on
└── font: FontSection
     └── heading: HeadingFontConfig
          ├── latin: Option<String>
          ├── cjk:   Option<String>
          └── emoji: Option<String>
```

Missing file or fields fall back to defaults. `config.example.toml` documents the
effective defaults and is guarded by a test (`config.rs`) so the two never drift.
