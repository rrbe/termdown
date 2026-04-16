# Markdown Feature Coverage

Audit of `src/markdown.rs` against pulldown-cmark 0.13 and common Markdown extensions.

## Supported (CommonMark core)

| Feature | Status | Notes |
|---|---|---|
| Heading H1–H6 | ✓ | H1–H3 rendered as PNG via Kitty graphics; H4–H6 fall back to ANSI bold |
| Paragraph / SoftBreak / HardBreak | ✓ | |
| Bold / Italic | ✓ | |
| Inline code | ✓ | |
| Code block (fenced & indented) | ✓ | No syntax highlighting, no language label |
| Blockquote | ✓ | Nested supported |
| Unordered / Ordered list | ✓ | Nested supported |
| Link | ✓ | URL printed next to the link text |
| Image | ⚠ | Placeholder text `[🖼 alt](url)` only — not rendered via Kitty |
| Horizontal rule | ✓ | |

## Enabled GFM extensions

- Strikethrough `~~x~~`
- Tables
- Task lists `[ ]` / `[x]`

## Not supported

### Mainstream Markdown gaps

- **HTML blocks / inline HTML** — silently dropped (`_ => {}` arm); content mixed with HTML becomes invisible
- **GFM autolinks** (bare URLs) — `ENABLE_GFM` not set
- **GFM alerts / admonitions** (`> [!NOTE]`, `[!WARNING]`, …) — rendered as plain blockquote
- **Footnotes** `[^1]` — `ENABLE_FOOTNOTES` not set

### Common extensions

- **Math** `$...$` / `$$...$$` — `ENABLE_MATH` not set
- **Definition list** — not enabled
- **YAML / TOML frontmatter** — not enabled; currently leaks into body
- **Smart punctuation** — not enabled
- **Wikilinks / Subscript / Superscript** — not enabled

### Graphical / rich-content extensions (outside pulldown-cmark — need custom handling)

- **Mermaid diagrams** — intercept ```` ```mermaid ```` fenced blocks, pipe to an external renderer (e.g. `mmdc`), output PNG via Kitty
- **Code block syntax highlighting** — could integrate `syntect`
- **Real image rendering** — local/remote `![](img.png)` could use the existing Kitty pipeline instead of the placeholder
- **PlantUML / Graphviz / other diagrams** — not supported

## Suggested priorities

1. **High value (hit in everyday Markdown)**: skip frontmatter, footnotes, alert/admonition styling, GFM autolinks, at least a graceful fallback for HTML
2. **Differentiating (plays to termdown's Kitty-graphics strength)**: Mermaid rendering, real image rendering, code-block syntax highlighting
3. **Nice to have**: math (KaTeX → image), smart punctuation, definition list
