# Terminal Protocols: ANSI 与 Kitty Graphics

termdown 写到 stdout 的字节流里同时包含两层协议——通用的 **ANSI 转义码** 和 Kitty 特有的 **graphics protocol**。理解二者的分层关系有助于阅读测试 fixture、调试渲染问题，以及看懂相关测试代码。

## 一、ANSI 转义码（ECMA-48 / ISO 6429）

通用的终端控制约定。正式名是 **ECMA-48 / ISO 6429**，"ANSI" 是历史叫法（最早是 1976 年的 ANSI X3.64）。它定义了一族以 `ESC`（`\x1b`）开头的转义序列，用来在字符流里夹带控制指令：

| 序列              | 含义                  |
| ----------------- | --------------------- |
| `\x1b[31m`        | 前景色设为红色        |
| `\x1b[1m`         | 粗体开                |
| `\x1b[0m`         | 重置所有属性          |
| `\x1b[2J`         | 清屏                  |
| `\x1b[H`          | 光标移到左上角        |
| `\x1b[38;5;213m`  | 256 色调色板前景色 213 |

几乎所有终端（xterm、Ghostty、Kitty、iTerm、Windows Terminal、tmux 内的伪终端…）都认这些。`fixtures/expected/*.ansi` 文件里大量出现的 `^[[1m...^[[0m` 就是 ANSI 着色码（粗体开/关）。

## 二、Kitty Graphics Protocol

Kitty 终端发明的**图像传输协议**，用来在文本流里夹带 PNG/RGBA 等图像数据，让终端把它当贴图渲染。它**借用了 ANSI/ECMA-48 里一种叫 APC（Application Program Command）的转义信封**：

```
\x1b_G  <header键值对>  ;  <base64 payload>  \x1b\
```

- `\x1b_` 起始、`\x1b\` 结束这两个**信封**是 ECMA-48 标准里为"应用层私有协议"专门留的口子，所有终端都认信封边界
- 但**信封里的语义**完全是 Kitty 自定义的：

| 键        | 含义                                                       |
| --------- | ---------------------------------------------------------- |
| `f=100`   | payload 是 PNG 数据                                        |
| `a=T`     | transmit + display（传输并立即显示，"display 形"）         |
| `a=t`     | transmit only（仅传输，等后续 `a=p` 放置，"lifecycle 形"） |
| `a=p`     | place（放置一张已传输过的图像）                            |
| `a=d`     | delete（删除已传输的图像）                                 |
| `i=N`     | image ID                                                   |
| `m=0`/`1` | 1 = 还有后续 chunk，0 = 这是最后一帧                       |

只有 Kitty、Ghostty、WezTerm、iTerm2（部分）认这些键。其它终端看到 APC 信封会**静默丢弃**——所以非 Kitty 系终端不会显示图像，但也不会把信封内容当文本回显（前提是它正确遵守 ECMA-48；iTerm2 在响应路径上有个 echo 问题，见 `ITERM2_KITTY_RESPONSE_LEAK.md`）。

## 三、分层关系一图速查

| 层级                                          | 是什么                       | 谁的            |
| --------------------------------------------- | ---------------------------- | --------------- |
| `\x1b[31m` 这类着色 / 光标控制                | ANSI/ECMA-48 标准转义码      | 所有终端        |
| `\x1b_G ... \x1b\` APC 信封                   | ECMA-48 标准里的"应用私有"口子 | 所有终端认信封  |
| 信封里 `f=100,a=T;<base64 PNG>` 的语义        | Kitty graphics protocol      | 只有 Kitty 系   |

## 四、在 termdown 里的体现

### Fixture 与 snapshot

`fixtures/expected/*.ansi` 捕获的是 termdown 写到 stdout 的整段字节流——里面**两层协议都有**：

- ANSI 着色码（粗体、颜色、高亮等）
- Kitty APC 帧（标题被光栅化成 PNG 后通过 APC 信封传输）

文件扩展名叫 `.ansi`，是因为 ANSI 着色码是更普遍的内容形态——叫 `.terminal` 或 `.kitty` 都不太贴。

### `tests/snapshots.rs::strip_kitty_images`

字节流里的 PNG payload 是字体/OS 相关的（同一段文字，macOS 和 Linux 渲染出来的像素不一样），跨平台逐字节对比会爆。所以 snapshot 测试在比对前用 `strip_kitty_images` **把每一段连续的 APC 帧替换成一个 `<IMG>` 占位符**：

```
\x1b_Gf=100,a=T,m=1;<base64>\x1b\\x1b_G;<base64>\x1b\\x1b_G;<base64>\x1b\
                              ↓ strip_kitty_images
                              <IMG>
```

这样 snapshot 比的是：
- ANSI 着色码序列完全一致
- 图片在哪些位置出现完全一致
- 但不比图片的 PNG 像素内容

### `tests/headings.rs::extract_kitty_pngs`

反方向——专门**只看图片**：扫 stdout 里所有 APC 帧，按 `m=1`/`m=0` 把分片拼回完整 base64，解码出原始 PNG 字节。然后用 `image` crate 解码 PNG，断言宽高、像素非空白、H1>H2>H3 缩放正确。

这种"按帧解析 APC + 拼接 chunk + base64 解码"的代码在调试新光栅化逻辑时也很有用——可以脱离终端直接拿到 termdown 想画什么。

## 五、调试技巧

- **看原始字节**：`cargo run -- --cat <file.md> | cat -v` 把不可见的 ESC 字符显示成 `^[`，肉眼可读
- **只看控制流不看图片**：管道接 `sed 's/\x1b_G[^\\]*\x1b\\\\/<IMG>/g'`（粗糙版的 strip_kitty_images）
- **验证终端是否支持 Kitty 图形**：`printf '\x1b_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1b\\'`——支持的终端会回 `\x1b_Gi=31;OK\x1b\`，不支持的什么都不回
- **强制启用 / 禁用 Kitty 输出**：termdown 通过 `TERM_PROGRAM` 环境变量识别终端类型，测试里固定 `TERM_PROGRAM=ghostty` 即可强制走 Kitty 路径

## 参考资料

- ECMA-48 / ISO 6429: https://ecma-international.org/publications-and-standards/standards/ecma-48/
- Kitty graphics protocol: https://sw.kovidgoyal.net/kitty/graphics-protocol/
- 同目录 `ITERM2_KITTY_RESPONSE_LEAK.md`：iTerm2 上 Kitty 响应被 echo 的具体案例
