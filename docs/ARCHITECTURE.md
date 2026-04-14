# Architecture

## Module Overview

```
src/
├── main.rs       CLI entry point, arg parsing, terminal state management
├── config.rs     ~/.termdown/config.toml loading (serde)
├── font.rs       Font resolution, caching, CJK/Latin detection
├── style.rs      HeadingStyle, ANSI constants, display width utilities
├── render.rs     Text measurement, glyph drawing, PNG encoding, Kitty protocol
└── markdown.rs   pulldown-cmark event handling, terminal output, word wrapping
```

## Rendering Pipeline

```
                         ┌─────────────────────┐
                         │   Markdown source    │
                         └──────────┬──────────┘
                                    │
                         ┌──────────▼──────────┐
                         │  pulldown-cmark      │
                         │  event stream        │
                         └──────────┬──────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    │                               │
          ┌─────────▼─────────┐          ┌──────────▼──────────┐
          │    H1 / H2 / H3   │          │   Everything else   │
          │  (image heading)  │          │   (ANSI text out)   │
          └─────────┬─────────┘          └─────────────────────┘
                    │
       ┌────────────▼────────────┐
       │  Per-character routing  │
       │  is_cjk(ch) ?          │
       ├────────┬───────────────┤
       │ Latin  │     CJK       │
       │ font   │     font      │
       └────┬───┴───────┬──────┘
            │           │
       ┌────▼───────────▼──────┐
       │  ab_glyph rasterize   │
       │  → RGBA pixel buffer  │
       └───────────┬───────────┘
                   │
       ┌───────────▼───────────┐
       │  PNG encode (image)   │
       └───────────┬───────────┘
                   │
       ┌───────────▼───────────┐
       │  Kitty graphics       │
       │  base64 → 4KB chunks  │
       │  \x1b_G...;\x1b\\    │
       └───────────┬───────────┘
                   │
       ┌───────────▼───────────┐
       │  stdout               │
       └───────────────────────┘
```

## Font Resolution

For each heading, a `FontPair` (Latin + CJK) is resolved:

```
1. User config  [font.heading] latin = "..."  /  cjk = "..."
       │
       ▼
2. Explicit weight variants (macOS workaround)
   Try "{family} Black", "{family} Heavy" as separate family names
   (Core Text registers bold variants as separate families)
       │
       ▼
3. Standard weight matching
   font-kit select_best_match with Weight::BLACK / EXTRA_BOLD / BOLD
       │
       ▼
4. Platform defaults
   Latin: Avenir, Futura, Helvetica Neue (macOS)
          Inter, Noto Sans, DejaVu Sans (Linux)
          Segoe UI, Arial (Windows)
   CJK:   Noto Serif CJK SC, Source Han Serif SC ... (per platform)
       │
       ▼
5. Embedded fallback
   SourceSerif4-SemiBold.ttf (bundled in binary via include_bytes!)
```

Font data loaded from disk or Core Text is `Box::leak`-ed into `'static` lifetime and cached in a global `HashMap` to avoid repeated allocations.

## CJK / Latin Split

`font::is_cjk(ch)` checks Unicode ranges:

| Range | Block |
|-------|-------|
| U+2E80..U+9FFF | CJK Radicals through Unified Ideographs (includes Hiragana, Katakana, CJK Symbols) |
| U+AC00..U+D7AF | Hangul Syllables |
| U+F900..U+FAFF | CJK Compatibility Ideographs |
| U+FE30..U+FE4F | CJK Compatibility Forms |
| U+FF00..U+FFEF | Halfwidth and Fullwidth Forms |
| U+20000..U+2FA1F | CJK Extensions B-F, Supplement |

All other characters (ASCII, Latin, Cyrillic, etc.) use the Latin font.

## Kitty Graphics Protocol

Headings are transmitted as:

```
\x1b_G f=100,a=T,q=2,m=1 ; <base64 chunk 1> \x1b\\
\x1b_G m=1 ; <base64 chunk 2> \x1b\\
...
\x1b_G m=0 ; <base64 chunk N> \x1b\\
```

- `f=100` — PNG format
- `a=T` — transmit and display
- `q=2` — suppress response (avoids iTerm2 "OK" leak)
- `m=1/0` — more chunks / last chunk
- Chunk size: 4096 bytes base64

## ANSI Text Rendering

Non-heading content goes through `markdown.rs`:

| Element | Rendering |
|---------|-----------|
| H4-H6 | Bold ANSI text |
| Paragraphs | Word-wrapped to terminal width |
| Ordered lists | Numbered with counter per nesting level |
| Unordered lists | Bullet (•) with indent per level |
| Blockquotes | Vertical bar (│) per nesting depth, italic gray |
| Code blocks | Buffered for uniform-width background |
| Inline code | Pink on dark gray |
| Links | Cyan underline + gray URL |
| Tables | Unicode box-drawing, ANSI-aware column width |
| Horizontal rule | ─ repeated to terminal width (max 60) |
| Images | Placeholder with alt text |

## Terminal State (UNIX only)

On UNIX, `main.rs` temporarily disables terminal echo before rendering and restores it after. This prevents Kitty protocol acknowledgment responses (notably from iTerm2) from appearing as visible text. Guarded by `#[cfg(unix)]`.

## Configuration

`~/.termdown/config.toml` is deserialized via serde:

```
Config
 └── font: FontSection
      └── heading: HeadingFontConfig
           ├── latin: Option<String>
           └── cjk: Option<String>
```

Missing config file or missing fields silently fall back to defaults.
