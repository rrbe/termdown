# Project Overview

termdown renders Markdown in two modes backed by the same parsed document:

- Interactive TUI for file paths on a terminal, with scrolling, search, a table
  of contents, local Markdown links, metadata folding, and live reload.
- Non-interactive output for `--cat`, stdin, pipes, and redirects.

H1-H3 headings are rasterized as PNG and displayed through the Kitty graphics
protocol. Body text, H4-H6 headings, lists, tables, code, quotes, links, and
frontmatter summaries use terminal text and ANSI styling. Theme, bell,
frontmatter, live reload, and heading fonts are configurable.

## Code map

- `src/layout.rs` parses Markdown into the shared `RenderedDoc` model.
- `src/cat.rs` writes that model once; `src/tui/` renders it interactively.
- `src/render.rs` and `src/font.rs` rasterize headings and emit Kitty commands.
- `src/config.rs` and `config.example.toml` define the configuration surface.
- `fixtures/` and `tests/` cover terminal output, CLI behavior, and heading PNGs.

## Maintenance pitfalls

- Kitty placements are identified by both image and placement IDs. Reposition
  an existing placement without deleting it first, and use targeted deletion;
  deleting image data forces retransmission and can make headings disappear.
- iTerm2 may return Kitty acknowledgements even with `q=2`. Cat mode suppresses
  TTY echo only on iTerm2; doing this globally triggers Ghostty's secure-input
  heuristic. TUI acknowledgement filtering must remain bounded by timeouts so
  malformed responses cannot swallow later keyboard input.
- Heading PNG pixels vary by OS and installed fonts. Snapshot tests replace PNG
  payloads with `<IMG>` and separate tests validate decoded image dimensions and
  non-empty pixels.
- Linux uses fontconfig through dynamic loading so builds do not require its
  development package. Keep the embedded Source Serif fallback usable.
- Build, format, lint, and test commands go through the `Makefile`; `make check`
  is the required local gate.
