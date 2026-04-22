# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.4.0]: https://github.com/rrbe/termdown/releases/tag/v0.4.0
[0.3.0]: https://github.com/rrbe/termdown/releases/tag/v0.3.0
[0.2.0]: https://github.com/rrbe/termdown/releases/tag/v0.2.0
[0.1.0]: https://github.com/rrbe/termdown/releases/tag/v0.1.0
