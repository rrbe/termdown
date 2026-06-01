# Termdown

[English](README.md)

在终端中以大字体标题渲染 Markdown，让观感更接近 GUI Markdown 阅读器的体验，基于 Kitty 图形协议。

<table>
<tr>
<td><img src="https://raw.githubusercontent.com/rrbe/termdown/v0.4.0/docs/screenshots/termdown_render_cn_demo.png" width="380" alt="termdown 渲染中文 README" /></td>
<td><img src="https://raw.githubusercontent.com/rrbe/termdown/v0.4.0/docs/screenshots/termdown_render_en_tui_demo.png" width="380" alt="termdown 在 TUI 模式下渲染英文 README" /></td>
</tr>
</table>

## 为什么做这个

本项目受 [glow](https://github.com/charmbracelet/glow) 和 [mdfried](https://github.com/benjajaja/mdfried) 启发。

- **glow** 不支持放大标题字体
- **mdfried** 支持放大 markdown 标题，但个人感觉可以做的更美观一点

termdown 将 H1-H3 标题栅格化为 PNG 图片，通过 Kitty 图形协议直接绘制到终端。提供两种使用模式：

- **交互式 TUI**（默认）—— `termdown README.md`，类 vim/less 的体验，支持常见的翻页、搜索等快捷键，支持查看 TOC、链接跳转，适合阅读较长文档。
- **直接输出**（`--cat`，或当 stdout 被管道/重定向、输入来自 stdin 时自动启用）—— 像 `cat` 一样轻量、管道友好，把渲染后的 Markdown 直接打到终端。

H4-H6 标题始终以 ANSI 粗体文本渲染。不想让文档加入那么多种字重，那样反而损害可读性。

## 安装

### 从 crates.io（推荐，需要 Rust）

```sh
cargo install termdown
```

安装到 `~/.cargo/bin/`。需要 Rust 1.95+。

### 安装脚本（无需 Rust 工具链）

```sh
curl -fsSL https://raw.githubusercontent.com/rrbe/termdown/master/install.sh | bash
```

默认装到 `/usr/local/bin`。用 `TERMDOWN_INSTALL_DIR` 覆盖安装目录。

<details>
<summary>手动下载</summary>

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

### 从源码

```sh
cargo install --git https://github.com/rrbe/termdown
```

## 卸载

```sh
curl -fsSL https://raw.githubusercontent.com/rrbe/termdown/master/uninstall.sh | bash
```

<details>
<summary>手动卸载</summary>

```sh
rm $(which termdown)
rm -rf ~/.config/termdown
```

</details>

## 使用

```sh
# 默认进入交互式 TUI
termdown README.md

# cat 风格纯输出（非交互、管道友好）
termdown --cat README.md
cat notes.md | termdown

# 指定主题；查看帮助
termdown --theme light README.md
termdown --help
```

完整的命令行参数、TUI 快捷键、配置项和已知问题都在 **[使用指南](docs/USAGE_CN.md)**。配置是可选的，位于 `~/.config/termdown/config.toml` —— 全部默认值见 [`config.example.toml`](config.example.toml)。

## 终端支持

需要支持 **Kitty 图形协议** 的终端（目前仅在 Ghostty 和 iTerm2 上测试过）：

- [Ghostty](https://ghostty.org)
- [Kitty](https://sw.kovidgoyal.net/kitty/)
- [WezTerm](https://wezfurlong.org/wezterm/)
- [iTerm2](https://iterm2.com)

不支持的终端会打印警告。H4-H6 标题始终以 ANSI 粗体文本渲染。

## 许可证

[Apache-2.0](LICENSE)
