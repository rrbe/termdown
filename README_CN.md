# Termdown

在终端中以大字体标题渲染 Markdown，基于 Kitty 图形协议。

## 为什么做这个

本项目受 [glow](https://github.com/charmbracelet/glow) 和 [mdfried](https://github.com/benjajaja/mdfried) 启发。

glow 是优秀的终端 Markdown 渲染器，但标题只能用 ANSI 粗体/颜色区分，无法真正放大显示。mdfried 支持图片化标题，但需要进入 TUI 模式。

termdown 将 H1-H3 标题栅格化为 PNG 图片，通过 Kitty 图形协议直接绘制到终端。同一套渲染器之上提供两种模式：

- **直接输出** —— 像 `cat` 一样轻量、管道友好，把渲染后的 Markdown 直接打到终端。
- **交互式 TUI**（`--tui`）—— 类 vim 的浏览器，支持搜索、目录、链接跳转，适合阅读较长文档。

H4-H6 标题始终以 ANSI 粗体文本渲染。

## 安装

### 下载预编译二进制

从 [Releases 页面](https://github.com/rrbe/termdown/releases/latest) 下载对应平台的归档文件。

支持的平台：`aarch64-apple-darwin`、`x86_64-apple-darwin`、`aarch64-unknown-linux-gnu`、`x86_64-unknown-linux-gnu`、`x86_64-pc-windows-msvc`。

以 macOS arm64 为例：

```sh
TARGET=aarch64-apple-darwin
BASE="https://github.com/rrbe/termdown/releases/latest/download"

curl -LO "${BASE}/termdown-${TARGET}.tar.gz"
curl -LO "${BASE}/SHA256SUMS"
grep "termdown-${TARGET}.tar.gz" SHA256SUMS | shasum -a 256 -c -

tar xzf "termdown-${TARGET}.tar.gz"
sudo mv termdown /usr/local/bin/
```

### 从源码安装

暂未发布到 crates.io —— 克隆仓库后用 cargo 构建：

```sh
git clone https://github.com/rrbe/termdown.git
cd termdown
cargo install --path .    # 安装到 ~/.cargo/bin/
```

如果想装到其他路径：

```sh
cargo build --release
cp target/release/termdown /usr/local/bin/
```

## 卸载

删除二进制文件和配置目录：

```sh
rm $(which termdown)
rm -rf ~/.termdown
```

## 使用

```sh
# 渲染文件
termdown README.md

# 从 stdin 管道输入
cat notes.md | termdown

# 指定主题（不使用自动检测）
termdown --theme light README.md

# 查看帮助
termdown --help
termdown --version
```

### TUI 模式

阅读较长的文档时，可以用 `--tui` 进入类似 vim 的交互浏览器：

```sh
termdown --tui README.md
```

按键绑定：

| 按键 | 动作 |
|---|---|
| `j` / `↓` | 向下滚动一行 |
| `k` / `↑` | 向上滚动一行 |
| `d` / `u` | 半屏向下 / 向上 |
| `f` / `Space` / `PgDn` | 整屏向下 |
| `b` / `PgUp` | 整屏向上 |
| `gg` / `G` | 跳到文档开头 / 末尾 |
| `]` / `[` | 下一个 / 上一个标题 |
| `t` | 切换目录面板 |
| `/` | 正向搜索 |
| `n` / `N` | 下一个 / 上一个匹配 |
| `?` | 切换快捷键帮助弹窗 |
| `Enter` | 打开链接（屏幕中有多个链接时显示序号选择器） |
| `o` / `i` | 在已跳转的 `.md` 文档之间前进 / 后退 |
| `q` / `Ctrl-C` | 退出 |

TUI 模式需要指定文件路径，不支持从 stdin 读取。

## 配置

配置文件位于 `~/.termdown/config.toml`。

```toml
# 主题："auto"（默认）、"dark" 或 "light"
# 自动检测通过 OSC 11 查询终端背景色。
theme = "auto"

[font.heading]
# 英文标题字体（推荐无衬线字体）
latin = "Inter"

# 中文标题字体
cjk = "LXGW WenKai"

# 图片标题里的 emoji / 符号 fallback 字体（可选）
emoji = "Apple Color Emoji"
```

混合语言标题（如 "Hello 世界"）会自动按字符选择对应字体渲染。
H1-H3 标题中的单个 emoji 也会通过 fallback 字体尽量渲染出来。

> **注意：** 正文以 ANSI 纯文本输出，字体由终端模拟器决定，不受 termdown 控制。

未配置时使用平台默认字体，最终回退到内嵌的 SourceSerif4 字体。

### 平台默认标题字体

**Latin**（无衬线）：

| macOS | Linux | Windows |
|-------|-------|---------|
| Avenir | Inter | Segoe UI |
| Avenir Next | Noto Sans | Arial |
| Futura | DejaVu Sans | Verdana |
| Helvetica Neue | Liberation Sans | |

**CJK**：

| macOS | Linux | Windows |
|-------|-------|---------|
| Noto Serif CJK SC | Noto Serif CJK SC | SimSun |
| Source Han Serif SC | Source Han Serif SC | KaiTi |
| Songti SC | Noto Serif | Microsoft YaHei |
| STSong | DejaVu Serif | |

## 终端支持

需要支持 **Kitty 图形协议** 的终端：

- [Ghostty](https://ghostty.org)
- [Kitty](https://sw.kovidgoyal.net/kitty/)
- [WezTerm](https://wezfurlong.org/wezterm/)
- [iTerm2](https://iterm2.com)

不支持的终端会打印警告。H4-H6 标题始终以 ANSI 粗体文本渲染。

## 已知问题

- **换行显示** -- 含 ANSI 转义序列的长行可能无法正确换行
- **终端兼容** -- 目前仅在 Ghostty 和 iTerm2 上测试过，其他支持 Kitty 协议的终端可能表现不一致
- **字体选择与降级** -- 字体粗细匹配依赖平台 API（Core Text / fontconfig），不一定能解析到预期的字重变体
- **主题检测** -- 自动检测依赖终端对 OSC 11 的响应；如终端不支持，请通过 `--theme` 或配置文件手动指定主题
- **复杂 emoji 序列** -- 依赖 ZWJ 的复杂 emoji（例如家庭/群组类组合、部分肤色组合）目前仍可能拆成多个字形，因为标题渲染还没有完整文本 shaping
- **TUI 帮助弹窗与标题图片** -- TUI 模式下 `?` 帮助弹窗绘制在文字层，而标题图片位于 Kitty graphics 层（始终覆盖在文字之上）。与弹窗区域重叠的标题图片会在弹窗打开时被临时移除，关闭后自动恢复 —— 这是 Kitty graphics 协议的限制，不是 bug

## 许可证

[Apache-2.0](LICENSE)
