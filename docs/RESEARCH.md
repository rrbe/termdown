# 终端渲染大字号 Markdown 标题 — 技术调研

## 问题

终端中渲染 Markdown 时，标题无法显示为更大字号。现有工具（glow、frogmouth 等）只能通过加粗/颜色区分标题层级，无法改变字体大小。这是终端字符网格（character-cell）架构的根本限制——每个字符占固定大小的格子，字体大小由终端模拟器全局控制，应用程序无权改变单个字符的大小。

## Frogmouth 调研结论（已排除）

https://github.com/Textualize/frogmouth

- 基于 Textual TUI 框架，底层仍是字符网格渲染
- 标题处理方式：加粗 + 颜色 + 上下留白，和 glow 本质相同
- **无法改变字号**，此方案不可行

## 已有项目（竞品调研）

### mdfried — 与本 idea 完全重合

https://github.com/benjajaja/mdfried （300+ stars，Rust）

- 核心做法和我们的 idea 一模一样：用字体光栅化把标题渲染成图片，通过 Sixel/Kitty/iTerm2 协议显示
- H1-H6 按比例缩放字号（H1 约 2x）
- Rust 实现，使用 `ratatui-image` 库处理终端图片协议
- 首次运行时选择系统字体（建议匹配终端字体）
- 在 Kitty 0.40+ 上优先使用 OSC 66 协议（无需光栅化）
- 不支持图片协议的终端 fallback 到 Chafa（ASCII/ANSI art）
- 兼容终端：Kitty, WezTerm, iTerm2, Ghostty, Foot, xterm (vt340)

### 其他相关项目

| 项目 | 方案 | 语言 | 说明 |
|------|------|------|------|
| [mkd](https://github.com/amatsuda/mkd) | 纯 OSC 66 | Ruby | 无图片 fallback，仅 Kitty |
| [presenterm](https://github.com/mfontanini/presenterm) | OSC 66 幻灯片 | Rust | 终端演示工具，标题用 font_size 1-7 |
| [cb-headscale.nvim](https://github.com/CbBelmante/cb-headscale.nvim) | OSC 66 in Neovim | Lua | 早期阶段 |
| [mcat](https://github.com/Skardyy/mcat) | Markdown 内嵌图片 | Rust | 标题仍是 ANSI 文本，不光栅化 |

### 主流工具均不支持大字号标题

glow、mdcat、rich-cli、frogmouth、mdless、terminal_markdown_viewer 等均只用 ANSI 加粗/颜色渲染标题。

## Kitty Text Sizing Protocol (OSC 66) — 新兴方案

Kitty 0.40（2025-03）引入的终端协议，可**不用渲染图片**直接显示大字号文本：

```bash
printf "\e]66;s=2;双倍大小的文字\a\n\n"   # 2x
printf "\e]66;s=3;三倍大小的文字\a\n\n\n"  # 3x
```

规范：https://sw.kovidgoyal.net/kitty/text-sizing-protocol/

- `s` 参数（scale, 1-7）：终端将每个字符渲染在 s×s 的 cell 块中
- 终端原生缩放字体，无需应用层光栅化
- **终端支持状态**：
  - Kitty 0.40+：完整支持（scale + width）
  - Foot：部分支持（仅 width）
  - **Ghostty：开发中（PR 进行中，优先支持 width）**
  - 其他终端：暂不支持

**结论：OSC 66 是更优雅的未来方向。** 等 Ghostty 支持后，标题放大可以零开销实现。但当前阶段仍需图片协议作为 fallback。

## 可行方案对比

| 方案 | 能否真正放大标题 | 复杂度 | 效果 |
|------|:---:|:---:|------|
| TUI 框架（Frogmouth/Textual/glow） | 否 | 中 | 加粗+颜色，本质相同 |
| FIGlet/ASCII Art | 视觉上是 | 低 | 用多行字符拼出大字，粗糙但兼容所有终端 |
| **Sixel/Kitty 图片协议** | **是** | 高 | 真正的大字号，依赖终端图片协议支持 |
| **OSC 66 Text Sizing** | **是** | 低 | 终端原生缩放，最优方案，但终端支持有限 |

## 选定方案：Kitty 图片协议

### 原理

1. 解析 Markdown，识别标题（`#` / `##` / `###`）
2. 用字体光栅化库将标题文字渲染为 PNG 图片（透明背景，不同层级对应不同字号）
3. 将 PNG base64 编码后，通过 Kitty 图片协议转义序列（`\033_Gf=100,a=T,...;base64data\033\\`）输出到终端
4. 非标题内容原样输出为终端文本

### 终端兼容性

支持 Kitty 图片协议的终端：
- **Ghostty**（当前使用的终端）
- Kitty
- WezTerm
- iTerm2（也支持自有的 inline image 协议）

Sixel 协议可作为 fallback，兼容更多终端。

### Python MVP 已验证

`mdrender.py` 是一个 Python 概念验证，使用 Pillow 做字体渲染 + Kitty 协议输出。已验证方案可行。

配置参数：
- 字体：PingFang（支持中英文）
- 字号：h1=48px, h2=36px, h3=28px
- 颜色：h1 亮白, h2 柔蓝, h3 柔绿
- 透明背景（继承终端背景色）

## 正式实现：推荐 Rust

目标是保持单二进制分发、体积尽量小。

### 语言选型对比

| | Rust | Go | Node.js |
|--|------|-----|---------|
| 单二进制 | 是 | 是 | 需 bun compile/pkg 打包 |
| 二进制大小 | ~2-3MB | ~6-8MB | ~30MB+ |
| 字体渲染 | `ab_glyph` + `image`（纯 Rust，零系统依赖） | `golang.org/x/image/font`（纯 Go） | `node-canvas`（依赖 Cairo） |
| 交叉编译 | 方便 | 方便 | 麻烦 |

**结论：选 Rust。** 二进制最小，纯 Rust 字体光栅化无系统依赖，最适合"尽量小"的目标。

### Rust 关键 crate

- `ab_glyph` 或 `fontdue` — 字体光栅化（纯 Rust）
- `image` — PNG 编码
- `base64` — base64 编码
- Markdown 解析可用 `pulldown-cmark`

## 下一步决策

### 选项 A：直接使用 mdfried
- 已经是成熟的 Rust 实现，和我们的 idea 完全重合
- 可以 fork 后定制，或直接使用

### 选项 B：自己实现（差异化方向）
如果要做，需要找到和 mdfried 的差异点，可能的方向：
- 更轻量（mdfried 基于 ratatui 全功能 TUI，我们可以做纯 pipeline 工具）
- OSC 66 优先 + 图片协议 fallback 的策略
- 更好的配置化（字体/颜色/字号）
- 作为 library 提供，而非独立应用

### 选项 C：关注 OSC 66 生态
- 等 Ghostty 支持 OSC 66 后，用最简方案实现
- 不需要图片渲染，代码量和二进制体积都极小

## 其他改进方向（如果自己实现）

- 颜色/字号可配置化
- inline 样式支持（`**bold**`、`*italic*` 等）
- 检测终端宽度，自动换行长标题
- Sixel 协议 fallback（兼容更多终端）
- 检测终端是否支持图片协议，不支持时退化为纯文本样式
