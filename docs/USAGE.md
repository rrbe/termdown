# termdown — Usage Guide

Full reference for using termdown. For installation and a quick start, see the
[README](../README.md). 中文版见 [USAGE_CN.md](USAGE_CN.md)。

## Command line

```
termdown [OPTIONS] [FILE]
```

| Option | Description |
|---|---|
| `FILE` | Markdown file to render. Use `-` or omit to read from stdin (always cat mode). |
| `--cat` | Force non-interactive cat-style output (pipe-friendly). |
| `--theme <auto\|dark\|light>` | Color theme. Default `auto` detects the terminal background via OSC 11. |
| `--no-bell` | Disable the edge-scroll terminal bell (also `bell = false` in config). |
| `-w`, `--watch` | Watch the file and live-reload the TUI on save (also `watch = true` in config). TUI only. |
| `-h`, `--help` | Show help. |
| `-V`, `--version` | Show version. |

By default, passing a `FILE` opens it in the interactive TUI. Piped/redirected
stdout, stdin input, or `--cat` all fall back to cat mode.

### Examples

```sh
# Open a file in the interactive TUI (default)
termdown README.md

# Force plain cat-style output (non-interactive, pipe-friendly)
termdown --cat README.md

# Pipe from stdin (always cat-style — TUI needs a real file)
cat notes.md | termdown

# Piped or redirected stdout also falls back to cat
termdown README.md | less

# Use a specific theme instead of auto-detect
termdown --theme light README.md

# Disable the edge-scroll bell
termdown --no-bell README.md

# Live preview: edit in your editor, watch it re-render on save
termdown --watch notes.md
```

## TUI mode

The TUI launches automatically whenever you pass a file and stdout is a real
terminal. It requires a file path; stdin input is not supported.

### Live reload (`--watch`)

`termdown --watch FILE` re-renders the preview whenever the file changes on
disk — ideal for a two-pane workflow: edit the Markdown in your editor (e.g.
vim) on one side, keep `termdown --watch` open on the other. Scroll position,
the open/closed Table of Contents, the metadata fold state, and any active
search are preserved across reloads. A `[watch]` marker appears in the status
bar. Editor atomic saves (write-temp-then-rename) are handled. Rasterized
headings are cached, so a save that only touches body text re-renders almost
instantly; only headings whose text actually changed are re-rasterized.

| Key | Action |
|---|---|
| `j` / `↓` | Scroll down one line |
| `k` / `↑` | Scroll up one line |
| `d` / `u` | Half page down / up |
| `f` / `Space` / `PgDn` | Full page down |
| `b` / `PgUp` | Full page up |
| `gg` / `G` | Jump to start / end |
| `]` / `[` | Next / previous heading |
| `t` | Toggle Table of Contents panel |
| `/` | Search forward |
| `n` / `N` | Next / previous match |
| `?` | Toggle keyboard-shortcut help overlay |
| `Enter` | Follow link (overlay picker if multiple visible) |
| `o` / `i` | Back / forward across followed `.md` links |
| `q` / `Ctrl-C` | Quit |

Press `?` in the TUI to see this list at any time.

## Configuration

termdown reads configuration from `~/.config/termdown/config.toml` (or
`$XDG_CONFIG_HOME/termdown/config.toml` if `XDG_CONFIG_HOME` is set). All
settings are optional; see [`config.example.toml`](../config.example.toml) for a
copy-pasteable file with every default.

```toml
# Theme: "auto" (default), "dark", or "light"
# Auto-detection queries the terminal background color via OSC 11.
theme = "auto"

# Vim-style edge bell: emit a terminal BEL when you scroll past the
# top/bottom of the document. The terminal emulator decides the visible
# effect (audible beep, title-bar 🔔, dock bounce, …). Default true.
# CLI: `--no-bell`.
bell = true

# Render YAML (`---`) / TOML (`+++`) frontmatter metadata blocks. Default
# true: --cat and the TUI show a dim one-line summary, and the TUI `m` key
# expands it. When false, metadata is hidden (still parsed, never leaks
# into body content).
metadata = true

# Watch the file and live-reload the preview on save. Default false. TUI only.
# CLI: `--watch` / `-w`.
watch = false

[font.heading]
# English heading font (sans-serif recommended)
latin = "Inter"

# CJK heading font
cjk = "LXGW WenKai"

# Emoji / symbol fallback font for image-rendered headings (optional)
emoji = "Apple Color Emoji"
```

Headings with mixed scripts (e.g. "Hello 世界") render each character with the
appropriate font automatically. Standalone emoji in H1–H3 headings are also
rendered via font fallback where possible.

> **Note:** Body text is rendered as plain ANSI text — its font is determined by
> your terminal emulator settings, not by termdown. To change the body font,
> configure your terminal directly.

If no config file exists, termdown uses platform-specific defaults and falls back
to an embedded SourceSerif4 font.

### Platform default heading fonts

**Latin** (sans-serif):

| macOS | Linux | Windows |
|-------|-------|---------|
| Avenir | Inter | Segoe UI |
| Avenir Next | Noto Sans | Arial |
| Futura | DejaVu Sans | Verdana |
| Helvetica Neue | Liberation Sans | |

**CJK**:

| macOS | Linux | Windows |
|-------|-------|---------|
| Noto Serif CJK SC | Noto Serif CJK SC | SimSun |
| Source Han Serif SC | Source Han Serif SC | KaiTi |
| Songti SC | Noto Serif | Microsoft YaHei |
| STSong | DejaVu Serif | |

## Known issues

- **Line wrapping** — long lines may not wrap correctly when mixed with ANSI escape sequences.
- **Terminal compatibility** — only tested on Ghostty and iTerm2; other Kitty-protocol terminals may behave differently.
- **Font selection & fallback** — weight matching relies on platform font APIs (Core Text / fontconfig), which may not always resolve to the expected variant.
- **Theme detection** — auto-detection relies on OSC 11 terminal responses; if your terminal does not support this, set the theme manually via `--theme` or the config file.
- **Complex emoji sequences** — ZWJ-heavy emoji (family/grouping variants, some skin-tone combinations) may render as separate glyphs because heading layout does not perform full text shaping.
- **TUI help popup vs heading images** — the `?` help overlay is drawn on the text layer, while heading images live on Kitty's graphics layer (always on top of text). A heading image overlapping the popup is temporarily removed while the popup is open and restored when it closes — a Kitty graphics protocol limitation, not a bug.
