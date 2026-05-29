# termdown — Context & Glossary

Single-context project. This file is the canonical glossary for domain terms
that appear in code, ADRs, and conversations about termdown. Keep entries short.
If you're tempted to add a term that future maintainers can derive from the
code itself (struct names, file layout, etc.), don't — only put **shared
vocabulary** here.

## Glossary

### Frontmatter
A block of metadata written at the **very beginning** of a Markdown file, fenced
by either `---` (YAML syntax) or `+++` (TOML syntax). Used by static site
generators (Jekyll, Hugo, Zola), note apps (Obsidian, Logseq), and agent skill
files (Anthropic, Cursor) to attach structured fields (title, author, tags,
name, description, …) to a document. Not part of CommonMark or GFM. Termdown
supports both YAML and TOML fences.

Synonym: **metadata block**. The two terms are interchangeable in this
project — `frontmatter` is the user-facing word, `MetadataBlock` is the
pulldown-cmark event name.

### Metadata one-line summary
The single dim line termdown renders in place of a parsed frontmatter block.
Format: `[metadata · key=value, key=value, …]` — wrapped in square brackets,
truncated to terminal width with the closing `]` preserved after the ellipsis.
Identical in both `--cat` and TUI **folded** state. Followed by one blank row
for visual separation from the body.

### Folded / Expanded (TUI metadata)
The two display states for a metadata block in TUI mode:
- **Folded** (default): one dim line — the [[metadata one-line summary]].
- **Expanded**: an inline box listing each key/value on its own row, pushing
  body content down. Triggered by the `m` key. Second `m` collapses back.

Cat mode has no "expanded" state — only the one-line summary or nothing.

### `[metadata] show`
The single config knob (in `~/.config/termdown/config.toml`) controlling whether
frontmatter is visible at all. `show = true` (default) renders the [[metadata
one-line summary]] / expanded box; `show = false` hides the metadata block in
**both** cat and TUI. The pulldown-cmark metadata extensions are always
enabled internally regardless — `show` only gates rendering, never parsing.
See [[adr-0001-metadata-block-handling]].

### Heuristic parser
The line-based key/value extractor used to turn a raw frontmatter block into
the one-line summary. Does **not** depend on a real YAML/TOML parser; splits
each non-blank line on the first `:` (YAML) or `=` (TOML) and trims. If zero
valid key/value pairs are extracted, falls back to a raw single-line join of
the block. Rationale: keeps the dependency surface small for a use case
(quick visual summary) where parse fidelity doesn't matter.
See [[adr-0001-metadata-block-handling]].
