# ADR 0001 — Metadata block (frontmatter) handling

- **Date**: 2026-05-28
- **Status**: Accepted
- **Owners**: shawn

## Context

Markdown files in the wild — agent skill files (Anthropic/Cursor), static-site
posts (Jekyll/Hugo/Zola), and notes (Obsidian/Logseq) — commonly start with a
**frontmatter** block fenced by `---` (YAML) or `+++` (TOML). Termdown today
does not enable pulldown-cmark's metadata-block extensions, so the opening
fence is parsed as a horizontal rule and the YAML body collapses into a setext
H2 heading — which then gets rasterized into a large PNG via the Kitty
graphics pipeline. The result is loud, ugly, and useless.

Documented prior state: `docs/MARKDOWN_FEATURE_COVERAGE.md:40` notes
"YAML / TOML frontmatter — not enabled; currently leaks into body";
`fixtures/supported-syntax.md:13-15` calls it out as a known roadmap item.

This ADR records how termdown will handle metadata blocks going forward.

## Decision

Termdown will treat frontmatter as a **first-class lightweight element**, not
as body content and not as completely invisible noise:

1. **Parsing**: Enable `ENABLE_YAML_STYLE_METADATA_BLOCKS` **and**
   `ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS` on the pulldown-cmark parser.
   Capture the raw block text via the `MetadataBlock` event pair.
2. **One-line summary**: A heuristic parser splits the raw block into
   `key=value, …` pairs, joined into a single dim line wrapped as
   `[metadata · key=value, …]`, truncated to terminal width — the closing
   `]` is preserved after the truncation ellipsis. No real YAML/TOML parser
   is introduced.
3. **Fallback**: If the heuristic extracts zero valid key/value pairs, fall
   back to a raw single-line join of the block content (so something useful is
   still shown for malformed or exotic input).
4. **Cat mode**: Render exactly the one-line summary at the document's top.
5. **TUI mode**: Render the same one-line summary by default (**folded**). The
   `m` key toggles to an **expanded** inline box listing each key/value on its
   own row. The box is part of the scrolling content (not pinned), not part
   of search, and not part of the Table of Contents. Default state is folded.
6. **Config gate**: A single boolean knob `[metadata] show` (default `true`)
   in `~/.config/termdown/config.toml`. When `false`, frontmatter is **completely
   hidden** in both cat and TUI; `m` becomes a no-op. The pulldown extensions
   remain enabled — `show` gates rendering only, never parsing, so frontmatter
   never leaks back into body regardless of config.

## Alternatives considered

### Visibility (what to show)

- **Hide completely, glow-style**. glow (the most-used CLI Markdown renderer)
  silently strips frontmatter — no chip, no line, gone. Simplest possible
  implementation. *Rejected*: termdown is positioned as a richer
  terminal-reader experience; silently dropping authored metadata sacrifices
  the agent-skill / Hugo-blog reading use case where the title/author/date
  is genuinely informative.
- **Render as a `yaml` code block**. Keeps the original text fully visible.
  *Rejected*: defeats the goal — the whole reason this is annoying today is
  that the raw text dominates the top of the document.
- **One-line summary only, no expanded state**. Always-folded, no `m` key.
  *Rejected*: users opening agent-skill files often want to see all fields at
  once; the expanded state addresses that without burning screen real estate
  by default.

### Parsing strategy

- **Full YAML parser** (e.g. `yaml-rust2`, `saphyr` — `serde_yaml` is
  unmaintained). *Rejected*: adds a non-trivial dependency (~100–200 KB to
  the binary) for a feature whose output is a 1-line summary. Termdown is a
  terminal reader, not a YAML validator. The 5% of real-world files with
  multi-line strings or deep nesting are not the target audience.
- **Treat each block as opaque text, no parsing**. *Rejected*: rejected as a
  primary mode (loses the `key=value` structure that makes the summary
  readable) but **accepted as the fallback** when the heuristic fails.

### Config surface

- **No config knob**. *Rejected*: at least one knob is justified for users who
  consistently don't want any frontmatter UI noise (the glow-style preference).
- **Multiple knobs** (`default_state`, `cat_format`, …). *Rejected*: YAGNI.
  The `m` key already handles per-document fold/expand preference; a single
  master switch covers the only opt-out we can name a real user for.

### Key binding

- `m` is unused in the current TUI map (`src/tui/input.rs`), short, and a
  natural mnemonic. Alternatives considered: `z`-style vim folds (multi-key
  chord, overkill), `F` for fold (less obvious mnemonic).

## Consequences

- **New behavior visible in every fixture that has frontmatter.**
  `fixtures/supported-syntax.md` already carries one; its golden snapshot
  `fixtures/expected/supported-syntax.ansi` will change in the implementation
  PR — reviewers should expect a large diff there and verify the new top
  line reads `· metadata · …`.
- **New keybinding** in TUI: `m`. Must be added to the `?` help screen.
- **New config field** in `config.toml`. Backward compatible — missing field
  defaults to `true`. Schema change in `src/config.rs`.
- **No new dependencies**. Both pulldown-cmark options already ship in the
  pinned 0.13 release.
- **Edge cases handled implicitly by pulldown-cmark**: stray `---` mid-doc
  is still a horizontal rule; frontmatter requires column 1, line 1; missing
  closing fence consumes to EOF (rare; documented behavior, not
  specially handled).
- **What's explicitly out of scope**: parsing frontmatter fields for
  *functional* use (e.g. piping `title` to the TUI title bar, using `tags`
  for filtering). Tracked separately if and when demand emerges.

## Test plan

- Snapshot diff on existing `fixtures/supported-syntax.md` (mutates golden).
- New focused fixtures under `fixtures/specialized/`:
  - `metadata-yaml.md` — standard YAML block, exercises heuristic happy path.
  - `metadata-toml.md` — standard TOML block, proves both syntaxes work.
  - `metadata-malformed.md` — multi-line / nested values that trip the
    heuristic, exercises fallback.
  - `metadata-none.md` — body containing mid-document `---` thematic
    breaks but no frontmatter; regression guard against false positives.
- Unit tests for the heuristic in `src/frontmatter.rs` (or wherever it lands):
  empty block, single field, multiple fields, value containing `:` / `=`,
  zero-valid-pairs → fallback.
- TUI fold/expand toggle: manual verification, not automated. Pure render
  state, no branching logic worth automating.

## Open follow-ups

- Migration of the config dir from `~/.termdown/` to XDG `~/.config/termdown/`
  is handled alongside this change (same branch); see `config.example.toml`
  and the `## Configuration` section of the README.
- "Use `title` field in TUI title bar" — sketched as a future enhancement,
  not committed.
