# termdown — Context & Glossary

Single-context project. This file is the canonical glossary for domain terms
that appear in code, ADRs, and conversations about termdown. Keep entries short.
If you're tempted to add a term that future maintainers can derive from the
code itself (struct names, file layout, etc.), don't — only put **shared
vocabulary** here.

## Glossary

### File Browser
The interactive **filesystem** panel that scans a directory for Markdown files
and lets the user switch between them inside the TUI (inspired by yazi). A left
side panel coexisting with the reader. **Never call this "目录"** — see
[[Contents (ToC)]] for the collision. The user-facing word is **File Browser**;
in Chinese, **文件浏览器**.

Status: proposed, not yet built. Reserve the name now so it can't drift.

### Contents (ToC)
The heading-outline side panel toggled by `t`, titled `"Contents"`, built from
the active document's `RenderedDoc.headings`. In casual Chinese this is often
called "目录", which **collides** with the filesystem sense used by the
[[File Browser]]. In this project: **Contents / ToC = headings of one document**;
**File Browser = files on disk**. They are different panels with different data
sources; do not conflate them in code, docs, or conversation.

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

### `metadata`
The single top-level config knob (in `~/.config/termdown/config.toml`)
controlling whether frontmatter is visible at all. `metadata = true` (the
default, and the behavior when the key is absent) renders the [[metadata
one-line summary]] / expanded box; `metadata = false` hides the metadata block
in **both** cat and TUI. The pulldown-cmark metadata extensions are always
enabled internally regardless — it only gates rendering, never parsing.
See [[adr-0001-metadata-block-handling]].

### Heuristic parser
The line-based key/value extractor used to turn a raw frontmatter block into
the one-line summary. Does **not** depend on a real YAML/TOML parser; splits
each non-blank line on the first `:` (YAML) or `=` (TOML) and trims. If zero
valid key/value pairs are extracted, falls back to a raw single-line join of
the block. Rationale: keeps the dependency surface small for a use case
(quick visual summary) where parse fidelity doesn't matter.
See [[adr-0001-metadata-block-handling]].
