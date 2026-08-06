# Termdown

[中文文档](README_CN.md)

termdown uses the Kitty graphics protocol to render Markdown with large-font headings in the terminal, providing a reading experience closer to a GUI Markdown reader.

<table>
<tr>
<td><img src="https://raw.githubusercontent.com/rrbe/termdown/master/docs/screenshots/termdown_render_cn_demo.png" width="380" alt="termdown rendering the Chinese README" /></td>
<td><img src="https://raw.githubusercontent.com/rrbe/termdown/master/docs/screenshots/termdown_render_en_tui_demo.png" width="380" alt="termdown rendering the English README in TUI mode" /></td>
</tr>
</table>

## Features

termdown rasterizes H1-H3 headings as PNG images and draws them directly in the terminal through the Kitty graphics protocol. It provides two modes:

- **Interactive TUI** (default) -- `termdown README.md` provides a vim/less-like experience with paging, search, a table of contents, and link navigation for longer documents.
- **Direct output** -- `termdown --cat README.md` prints rendered Markdown like `cat`, making it suitable for short documents or piping to other programs.

H4-H6 headings always use ANSI bold text instead of simulating more font sizes and weights that could reduce readability in a terminal.

## Installation

### Cargo

```sh
cargo install termdown
```

Installs into `~/.cargo/bin/`. Requires Rust 1.95+.

### Install script

```sh
curl -fsSL https://raw.githubusercontent.com/rrbe/termdown/master/install.sh | bash
```

Defaults to `/usr/local/bin`. Override the target directory with `TERMDOWN_INSTALL_DIR`.

<details>
<summary>Manual download</summary>

```sh
TARGET=aarch64-apple-darwin
BASE="https://github.com/rrbe/termdown/releases/latest/download"

curl -LO "${BASE}/termdown-${TARGET}.tar.gz"
curl -LO "${BASE}/SHA256SUMS"
grep "termdown-${TARGET}.tar.gz" SHA256SUMS | shasum -a 256 -c -

tar xzf "termdown-${TARGET}.tar.gz"
sudo mv termdown /usr/local/bin/
```

</details>

### Install from source

```sh
cargo install --git https://github.com/rrbe/termdown
```

## Uninstall

```sh
curl -fsSL https://raw.githubusercontent.com/rrbe/termdown/master/uninstall.sh | bash
```

<details>
<summary>Manual uninstall</summary>

```sh
rm $(which termdown)
rm -rf ~/.config/termdown
```

</details>

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

## Documentation

- [Usage guide](docs/USAGE.md)
- [Project overview](docs/OVERVIEW.md)
- Configuration and defaults: [`config.example.toml`](config.example.toml)
- Configuration file: `~/.config/termdown/config.toml`

## Terminal Support

Requires a terminal with **Kitty graphics protocol** support, such as:

- [Kitty](https://sw.kovidgoyal.net/kitty/)
- [iTerm2](https://iterm2.com)
- [Ghostty](https://ghostty.org)

## License

[Apache-2.0](LICENSE)
