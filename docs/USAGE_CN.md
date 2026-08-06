# termdown — 使用指南

termdown 的完整使用参考。安装与快速上手见 [README_CN](../README_CN.md)。English
version: [USAGE.md](USAGE.md)。

## 命令行

```
termdown [选项] [文件]
```

| 选项 | 说明 |
|---|---|
| `文件` | 要渲染的 Markdown 文件。用 `-` 或省略则从 stdin 读取（始终是 cat 模式）。 |
| `--cat` | 强制使用非交互的 cat 风格输出（管道友好）。 |
| `--theme <auto\|dark\|light>` | 配色主题。默认 `auto`，通过 OSC 11 检测终端背景色。 |
| `--no-bell` | 关闭到顶/到底的终端提示铃（也可在配置中设 `bell = false`）。 |
| `-w`, `--watch` | 监听文件变动并在保存时实时刷新预览（也可在配置中设 `watch = true`）。仅 TUI 模式生效。 |
| `-h`, `--help` | 显示帮助。 |
| `-V`, `--version` | 显示版本。 |

默认情况下，传入文件会进入交互式 TUI。stdout 被管道/重定向、输入来自 stdin、或加
`--cat` 时都会回退到 cat 模式。通过管道或重定向输出时，标题使用文本而不是 Kitty
图片，确保下游程序保留标题内容。

### 示例

```sh
# 默认进入交互式 TUI
termdown README.md

# 强制使用 cat 风格的纯输出（非交互、管道友好）
termdown --cat README.md

# 从 stdin 管道输入（始终是 cat 模式 —— TUI 需要真实文件）
cat notes.md | termdown

# stdout 被管道/重定向时也会自动回退到 cat
termdown README.md | less

# 指定主题（不使用自动检测）
termdown --theme light README.md

# 关闭到顶/到底时的提示铃声
termdown --no-bell README.md

# 实时预览：在编辑器里改，保存即重新渲染
termdown --watch notes.md
```

## TUI 模式

当传入文件且 stdout 为真实终端时自动进入 TUI。TUI 模式需要指定文件路径，不支持从
stdin 读取。

### 实时刷新（`--watch`）

`termdown --watch 文件` 会在文件发生变动时重新渲染预览 —— 非常适合左右分屏的工作流：
一侧用编辑器（如 vim）编辑 Markdown，另一侧开着 `termdown --watch` 实时预览。重新加载
会保留滚动位置、目录面板的开合状态、元数据折叠状态以及正在进行的搜索；状态栏会显示
`[watch]` 标记。编辑器的原子保存（先写临时文件再重命名覆盖）也能正确处理。标题图片会被
缓存，因此只改正文的保存几乎瞬间刷新，只有标题文字真正变化时才会重新栅格化。

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
| `o` / `i` | 在已跳转的 `.md` 文档之间后退 / 前进 |
| `q` / `Ctrl-C` | 退出 |

在 TUI 中随时按 `?` 即可查看此列表。

## 配置

配置文件位于 `~/.config/termdown/config.toml`（若设置了 `XDG_CONFIG_HOME`，则为
`$XDG_CONFIG_HOME/termdown/config.toml`）。所有配置项均为可选；仓库根目录的
[`config.example.toml`](../config.example.toml) 提供了一份包含全部默认值、可直接
复制的示例。

```toml
# 主题："auto"（默认）、"dark" 或 "light"
# 自动检测通过 OSC 11 查询终端背景色。
theme = "auto"

# 文档到顶/到底时向终端发一次 BEL。具体表现（响铃、标题栏 🔔、
# dock 弹跳等）由终端模拟器决定。默认 true，命令行可用 `--no-bell` 关闭。
bell = true

# 是否渲染 YAML（`---`）/ TOML（`+++`）frontmatter 元数据块。默认 true：
# --cat 和 TUI 会显示一行 dim 摘要，TUI 按 `m` 可展开。设为 false 则完全
# 隐藏元数据（仍会解析，因此不会泄漏进正文）。
metadata = true

# 监听文件变动并在保存时实时刷新预览。默认 false，仅 TUI 模式生效。
# 命令行可用 `--watch` / `-w` 开启。
watch = false

[font.heading]
# 英文标题字体（推荐无衬线字体）
latin = "Inter"

# 中文标题字体
cjk = "LXGW WenKai"

# 图片标题里的 emoji / 符号 fallback 字体（可选）
emoji = "Apple Color Emoji"
```

混合语言标题（如 "Hello 世界"）会自动按字符选择对应字体渲染。H1-H3 标题中的单个
emoji 也会通过 fallback 字体尽量渲染出来。

> **注意：** 正文以 ANSI 纯文本输出，字体由终端模拟器决定，不受 termdown 控制。
> 想改正文字体请直接配置终端。

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

## 已知问题

- **换行显示** —— 含 ANSI 转义序列的长行可能无法正确换行。
- **终端兼容性** —— 目前仅在 Ghostty 和 iTerm2 上测试过，其它 Kitty 协议终端表现可能不同。
- **字体选择与降级** —— 字体粗细匹配依赖平台 API（Core Text / fontconfig），不一定能解析到预期的字重变体。
- **主题检测** —— 自动检测依赖终端对 OSC 11 的响应；如终端不支持，请通过 `--theme` 或配置文件手动指定主题。
- **复杂 emoji 序列** —— 依赖 ZWJ 的复杂 emoji（家庭/群组类组合、部分肤色组合）可能拆成多个字形，因为标题渲染还没有完整文本 shaping。
- **TUI 帮助弹窗与标题图片** —— `?` 帮助弹窗绘制在文字层，而标题图片位于 Kitty graphics 层（始终覆盖在文字之上）。与弹窗区域重叠的标题图片会在弹窗打开时被临时移除，关闭后自动恢复 —— 这是 Kitty graphics 协议的限制，不是 bug。
