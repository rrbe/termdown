# Termdown

[中文文档](README_CN.md)

Render Markdown with large-font headings in the terminal using the Kitty graphics protocol.

## Motivation

Inspired by [glow](https://github.com/charmbracelet/glow) and [mdfried](https://github.com/benjajaja/mdfried).

glow is a great terminal Markdown renderer, but headings are only distinguished by ANSI bold/color -- they can't actually be displayed at a larger size. mdfried supports image-rendered headings, but requires entering a TUI.

termdown aims to be **a lightweight `cat`-like tool where headings are truly rendered large**. It rasterizes H1-H3 text as PNG images and displays them via the Kitty graphics protocol -- no TUI, pipe-friendly, just direct output.

## Terminal Support

Requires a terminal with **Kitty graphics protocol** support:

- [Ghostty](https://ghostty.org)
- [Kitty](https://sw.kovidgoyal.net/kitty/)
- [WezTerm](https://wezfurlong.org/wezterm/)
- [iTerm2](https://iterm2.com)

On unsupported terminals, termdown will print a warning and heading images may not display correctly. H4-H6 headings always render as plain ANSI bold text.

## Installation

### From source

```sh
cargo install --path .
```

### Build manually

```sh
cargo build --release
cp target/release/termdown /usr/local/bin/
```

## Usage

```sh
# Render a file
termdown README.md

# Pipe from stdin
cat notes.md | termdown

# Flags
termdown --help
termdown --version
```

## Configuration

termdown reads configuration from `~/.termdown/config.toml`.

```toml
[font.heading]
# Font for Latin/English text in H1-H3 headings (sans-serif recommended)
latin = "Inter"

# Font for CJK text in H1-H3 headings
cjk = "LXGW WenKai"

# Optional emoji / symbol fallback font for image-rendered headings
emoji = "Apple Color Emoji"
```

Headings with mixed scripts (e.g. "Hello 世界") will render each character with the appropriate font automatically.
Standalone emoji in H1-H3 headings are also supported through font fallback when the selected emoji font exposes outline or raster glyphs.

> **Note:** Body text is rendered as plain ANSI text -- its font is determined by your terminal emulator settings, not by termdown. To change the body font, configure your terminal directly.

If no config file exists, termdown uses platform-specific defaults and falls back to an embedded SourceSerif4 font.

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

## Uninstall

Remove the binary and delete the configuration directory:

```sh
rm $(which termdown)
rm -rf ~/.termdown
```

## Known Issues

- **Line wrapping** -- long lines may not wrap correctly when mixed with ANSI escape sequences
- **Terminal compatibility** -- only tested on Ghostty and iTerm2; other Kitty-protocol terminals may behave differently
- **Font selection & fallback** -- weight matching relies on platform font APIs (Core Text / fontconfig) which may not always resolve to the expected variant
- **Theme awareness** -- heading colors are hardcoded for dark backgrounds; light terminal themes may have poor contrast
- **Complex emoji sequences** -- ZWJ-heavy emoji sequences (family/grouping variants, some skin-tone combinations) may still render as separate glyphs because heading layout does not perform full text shaping

## License

[Apache-2.0](LICENSE)
