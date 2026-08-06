# Termdown

[English](README.md)

termdown 基于 Kitty 图形协议，在终端中以大字体标题渲染 Markdown，提供更接近 GUI Markdown 阅读器的阅读体验。

<table>
<tr>
<td><img src="https://raw.githubusercontent.com/rrbe/termdown/master/docs/screenshots/termdown_render_cn_demo.png" width="380" alt="termdown 渲染中文 README" /></td>
<td><img src="https://raw.githubusercontent.com/rrbe/termdown/master/docs/screenshots/termdown_render_en_tui_demo.png" width="380" alt="termdown 在 TUI 模式下渲染英文 README" /></td>
</tr>
</table>

## 功能

termdown 将 H1-H3 标题栅格化为 PNG 图片，通过 Kitty 图形协议直接绘制到终端。提供两种使用模式：

- **交互式 TUI**（默认）—— `termdown README.md`，提供类似 vim/less 的体验，支持翻页、搜索、查看目录和链接跳转，适合阅读较长文档。
- **直接输出** —— `termdown --cat README.md`，像 `cat` 一样直接输出渲染后的 Markdown，适合查看短文档或通过管道交给其他程序处理。

H4-H6 标题始终以 ANSI 粗体文本渲染，不再模拟更多字号和字重，以免损害终端中的可读性。

## 安装

### Cargo

```sh
cargo install termdown
```

安装到 `~/.cargo/bin/`。需要 Rust 1.95+。

### 脚本安装

```sh
curl -fsSL https://raw.githubusercontent.com/rrbe/termdown/master/install.sh | bash
```

默认装到 `/usr/local/bin`。可以用 `TERMDOWN_INSTALL_DIR` 覆盖安装目录。

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

### 源码安装

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

# 实时预览：保存即重新渲染（在编辑器里改，在这里看）
termdown --watch notes.md
```

## 文档

- [使用指南](docs/USAGE_CN.md)
- [项目概览](docs/OVERVIEW.md)
- 配置和默认值：[`config.example.toml`](config.example.toml)
- 配置文件：`~/.config/termdown/config.toml`

## 终端支持

需要支持 **Kitty 图形协议** 的终端，比如：

- [Kitty](https://sw.kovidgoyal.net/kitty/)
- [iTerm2](https://iterm2.com)
- [Ghostty](https://ghostty.org)

## 许可证

[Apache-2.0](LICENSE)
