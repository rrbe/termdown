# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Markdown frontmatter support.** YAML (`---`) and TOML (`+++`) metadata
  blocks at the top of a document are now parsed and never leak into body
  content. `--cat` renders a dim one-line summary
  (`[metadata · key=value, …]`); TUI shows the same line and can expand it
  to an inline key/value box with the new `m` key. A blank row follows the
  summary for visual separation. Opt out entirely via `metadata = false`
  in `~/.config/termdown/config.toml`. See `docs/adr/0001-metadata-block-handling.md`.

### Changed
- **Config location moved to the XDG path.** termdown now reads
  `~/.config/termdown/config.toml` (or `$XDG_CONFIG_HOME/termdown/config.toml`)
  instead of `~/.termdown/config.toml`. If a legacy `~/.termdown/config.toml`
  is found while the new path is empty, termdown prints a one-line warning so
  the old settings aren't dropped silently — move the file to the new location
  to clear it. A documented `config.example.toml` with every default ships in
  the repo root.
- **Config parsing is now strict.** An unknown key (e.g. a typo like `bel` for
  `bell`) or an invalid `theme` value is reported as a one-line warning and the
  config falls back to defaults, instead of being silently ignored. `--theme`
  likewise warns on an unrecognized value rather than quietly auto-detecting.

## [0.5.1] - 2026-05-26

### Added
- Vim-style edge bell: scrolling past the top or bottom now emits a
  terminal BEL. The visible effect (audible beep, title-bar 🔔, dock
  bounce) is up to the terminal emulator — e.g. Ghostty surfaces a 🔔
  in the window title via its default `bell-features`. Triggers on
  `j`/`k`, `d`/`u`, `f`/`b`/`Space`/`PgUp`/`PgDn`; explicit jumps
  (`gg`, `G`, `]`, `[`) stay silent. Disable with `--no-bell` or
  `bell = false` in `~/.termdown/config.toml`.

### Changed
- Removed the 4-column outer margin that cat mode and TUI body rows
  shared, plus the additional 2-column inset on cat-mode table rows.
  Content now starts at column 0. The gutter originally mirrored `glow`;
  with TUI as the default it cost 4 cols for no remaining visual gain.

## [0.5.0] - 2026-05-21

### Changed
- **BREAKING:** Passing a Markdown file now opens the interactive TUI by
  default instead of printing cat-style output. Use `--cat` to force the
  previous non-interactive renderer. Pipes still work transparently:
  `termdown FILE.md | less` and `cat FILE.md | termdown` automatically
  fall back to cat-style output when stdout is not a terminal or input
  comes from stdin.

### Added
- `--cat` flag to force non-interactive cat-style output regardless of
  whether stdout is a terminal.

### Removed
- **BREAKING:** The `--tui` flag is gone. With TUI as the default it was
  either a no-op (TTY case) or a footgun (forcing TUI when stdout is not
  a terminal, which ratatui can't drive). Drop it from scripts.

## [0.4.0] - 2026-04-22

First release published to [crates.io](https://crates.io/crates/termdown).

### Added
- Interactive TUI mode (`--tui`) with vim-style navigation, forward search
  (`/`, `n`, `N`), Table of Contents panel (`t`), keyboard-shortcut help
  overlay (`?`), link-follow navigation across Markdown files (`Enter`, `o`,
  `i`), and Kitty-protocol heading images inside the TUI viewport.
- Side-by-side README screenshots showing direct-render and TUI modes.
- `repository`, `homepage`, `rust-version`, and `exclude` metadata in
  `Cargo.toml` for the crates.io release.

### Changed
- Build, lint, format, and test now route through a shared `Makefile` that
  CI and local workflows both invoke — no drift between environments.
- README installation section leads with `cargo install termdown`.

### Fixed
- Markdown renderer: collapsed nested `if` into `match` guards, improving
  readability and branch coverage.

## [0.3.0] - prior to crates.io

### Added
- HTML block and inline HTML rendering in the Markdown pipeline.
- Markdown feature coverage audit documenting supported/unsupported syntax.
- GitHub Actions CI and auto-release workflows; cross-compilation support
  for `aarch64-unknown-linux-gnu` via `cross`.
- Install and uninstall shell scripts for prebuilt binary releases.

### Fixed
- Install `fontconfig` in the cross container so aarch64 builds succeed.
- Rustfmt and system-deps issues on CI.

## [0.2.0] - prior to crates.io

### Added
- Theme auto-detection (OSC 11) with explicit `--theme light|dark|auto`.
- Task list checkbox rendering.
- CLI integration tests and markdown edge-case tests.
- Emoji fallback for image-rendered headings.
- Architecture documentation.

### Fixed
- Nested list text loss in the Markdown renderer.

## [0.1.0] - prior to crates.io

Initial release: direct-output Markdown renderer with H1–H3 headings
rasterized to PNG and painted via the Kitty graphics protocol.

[0.5.0]: https://github.com/rrbe/termdown/releases/tag/v0.5.0
[0.4.0]: https://github.com/rrbe/termdown/releases/tag/v0.4.0
[0.3.0]: https://github.com/rrbe/termdown/releases/tag/v0.3.0
[0.2.0]: https://github.com/rrbe/termdown/releases/tag/v0.2.0
[0.1.0]: https://github.com/rrbe/termdown/releases/tag/v0.1.0
