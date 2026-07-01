# Termdown

[中文文档](README_CN.md)

Render Markdown with large-font headings in the terminal using the Kitty graphics protocol.

<table>
<tr>
<td><img src="https://raw.githubusercontent.com/rrbe/termdown/master/docs/screenshots/termdown_render_cn_demo.png" width="380" alt="termdown rendering the Chinese README" /></td>
<td><img src="https://raw.githubusercontent.com/rrbe/termdown/master/docs/screenshots/termdown_render_en_tui_demo.png" width="380" alt="termdown rendering the English README in TUI mode" /></td>
</tr>
</table>

## Motivation

Inspired by [glow](https://github.com/charmbracelet/glow) and [mdfried](https://github.com/benjajaja/mdfried).

glow is a great terminal Markdown renderer, but headings are only distinguished by ANSI bold/color -- they can't actually be displayed at a larger size. mdfried supports image-rendered headings, but requires entering a TUI.

termdown rasterizes H1-H3 headings as PNG and paints them via the Kitty graphics protocol. Two modes share the same renderer:

- **Interactive TUI** (default when a file is given) -- vim-style browser with search, Table of Contents, and link-follow navigation for longer documents.
- **Direct output** (`--cat`, or automatic when stdout is piped / input comes from stdin) -- dump rendered Markdown straight into your terminal.

H4-H6 headings always fall back to ANSI bold text.

## Installation

### From crates.io (recommended, requires Rust)

```sh
cargo install termdown
```

Installs into `~/.cargo/bin/`. Requires Rust 1.95+.

> **Linux:** no `-dev` packages or `pkg-config` are required to build — only a
> C toolchain (freetype is compiled from source when the system one isn't
> found), and fontconfig is loaded lazily at run time. For system font
> discovery (including CJK headings), install `fontconfig` plus the fonts you
> want (e.g. `apt install fontconfig fonts-noto-cjk`). Without it, termdown
> falls back to its bundled font.

### Prebuilt binary (no Rust toolchain needed)

**macOS / Linux:**

```sh
curl -fsSL https://raw.githubusercontent.com/rrbe/termdown/master/install.sh | bash
```

**Windows (PowerShell):**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/rrbe/termdown/releases/latest/download/termdown-installer.ps1 | iex"
```

The macOS/Linux command is a compatibility wrapper: it understands both the new
cargo-dist archive layout and the legacy archive layout. The Windows installer
is available on releases built with cargo-dist. **To update, just re-run the
install command.**

Or, with [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) (fetches the prebuilt binary, no compile):

```sh
cargo binstall termdown
```

<details>
<summary>Manual download (no script)</summary>

```sh
TARGET=aarch64-apple-darwin
BASE="https://github.com/rrbe/termdown/releases/latest/download"

curl -LO "${BASE}/termdown-${TARGET}.tar.xz"
curl -LO "${BASE}/termdown-${TARGET}.tar.xz.sha256"
shasum -a 256 -c "termdown-${TARGET}.tar.xz.sha256"

tar xf "termdown-${TARGET}.tar.xz"
sudo mv "termdown-${TARGET}/termdown" /usr/local/bin/
```

</details>

### From git (latest development snapshot)

```sh
cargo install --git https://github.com/rrbe/termdown
```

## Uninstall

If you installed from crates.io:

```sh
cargo uninstall termdown
```

If you installed from the shell installer:

```sh
curl -fsSL https://raw.githubusercontent.com/rrbe/termdown/master/uninstall.sh | bash
```

Manual cleanup:

```sh
rm $(which termdown)
rm -rf ~/.config/termdown
```

## Usage

```sh
# Open a file in the interactive TUI (default)
termdown README.md

# Plain cat-style output (non-interactive, pipe-friendly)
termdown --cat README.md
cat notes.md | termdown

# Pick a theme; show help
termdown --theme light README.md
termdown --help

# Live preview: re-render on every save (edit in your editor, watch here)
termdown --watch notes.md
```

The full CLI reference, TUI key bindings, configuration, and known issues live in the **[Usage Guide](docs/USAGE.md)**. Configuration is optional and lives at `~/.config/termdown/config.toml` -- see [`config.example.toml`](config.example.toml) for every default.

## Terminal Support

Requires a terminal with **Kitty graphics protocol** support:

- [Ghostty](https://ghostty.org)
- [Kitty](https://sw.kovidgoyal.net/kitty/)
- [WezTerm](https://wezfurlong.org/wezterm/)
- [iTerm2](https://iterm2.com)

On unsupported terminals, termdown prints a warning and heading images may not display correctly. H4-H6 headings always render as plain ANSI bold text.

## License

[Apache-2.0](LICENSE)
