# Termdown

在终端中以大字体标题渲染 Markdown，基于 Kitty 图形协议。

## 为什么做这个

本项目受 [glow](https://github.com/charmbracelet/glow) 和 [mdfried](https://github.com/benjajaja/mdfried) 启发。

glow 是优秀的终端 Markdown 渲染器，但标题只能用 ANSI 粗体/颜色区分，无法真正放大显示。mdfried 支持图片化标题，但需要进入 TUI 模式。

termdown 的目标很简单：**像 `cat` 一样轻量，但标题能真正大写化显示**。它将 H1-H3 标题栅格化为 PNG 图片，通过 Kitty 图形协议直接输出到终端，无需 TUI，管道友好。

## 终端支持

需要支持 **Kitty 图形协议** 的终端：

- [Ghostty](https://ghostty.org)
- [Kitty](https://sw.kovidgoyal.net/kitty/)
- [WezTerm](https://wezfurlong.org/wezterm/)
- [iTerm2](https://iterm2.com)

不支持的终端会打印警告。H4-H6 标题始终以 ANSI 粗体文本渲染。

## 安装

### 从源码安装

```sh
cargo install --path .
```

### 手动构建

```sh
cargo build --release
cp target/release/termdown /usr/local/bin/
```

## 使用

```sh
# 渲染文件
termdown README.md

# 从 stdin 管道输入
cat notes.md | termdown

# 查看帮助
termdown --help
termdown --version
```

## 配置

配置文件位于 `~/.termdown/config.toml`。

```toml
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

## 卸载

删除二进制文件和配置目录：

```sh
rm $(which termdown)
rm -rf ~/.termdown
```

## 已知问题

- **换行显示** -- 含 ANSI 转义序列的长行可能无法正确换行
- **终端兼容** -- 目前仅在 Ghostty 和 iTerm2 上测试过，其他支持 Kitty 协议的终端可能表现不一致
- **字体选择与降级** -- 字体粗细匹配依赖平台 API（Core Text / fontconfig），不一定能解析到预期的字重变体
- **主题适配** -- 标题配色为深色背景硬编码，浅色终端主题下对比度可能不足
- **复杂 emoji 序列** -- 依赖 ZWJ 的复杂 emoji（例如家庭/群组类组合、部分肤色组合）目前仍可能拆成多个字形，因为标题渲染还没有完整文本 shaping

## 许可证

[Apache-2.0](LICENSE)
