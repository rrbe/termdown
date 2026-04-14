# iTerm2 Kitty 图形协议响应泄漏问题

## 现象

在 iTerm2 中运行 termdown 渲染 Markdown 时，标题图片可以正常显示，但会伴随出现大量乱码：

```
^[_Gi=0,p=0;OK^[\^[_Gi=0;OK^[\^[_Gi=0,p=0;OK^[\^[_Gi=0;OK^[\%
```

在 shell prompt 行也会出现类似内容：

```
README.mdGi=0,p=0;OKREADME.mdGi=0;OKREADME.mdGi=0,p=0;OKREADME.md...
```

在 Ghostty、Kitty、WezTerm 等终端中不会出现此问题。

## 原因分析

### Kitty 图形协议的请求-响应模型

Kitty 图形协议是一个**双向协议**。当应用向终端发送图片命令时：

```
应用 → 终端:  \x1b_Gf=100,a=T,m=0;{base64_png}\x1b\\
终端 → 应用:  \x1b_Gi=0;OK\x1b\\
```

终端处理完图片后，会通过 PTY 的输入端（即应用的 stdin）发回一个确认响应。

### iTerm2 的行为差异

- **Ghostty / Kitty / WezTerm**：正确处理响应，应用不读取也不会造成问题
- **iTerm2**：发送 `OK` 响应到 PTY 输入缓冲区，由于 TTY 驱动的 echo 设置，这些字节被**回显到屏幕上**

关键点：终端默认处于 cooked mode（canonical mode），TTY 驱动会自动 echo 所有写入输入缓冲区的内容。iTerm2 的响应经过 PTY 输入端时被 echo 机制显示，导致出现乱码。

### 数据流路径

```
termdown → stdout → PTY master → 终端模拟器(iTerm2)
                                        |
                                        | 处理图片命令，生成响应
                                        v
termdown ← stdin ← PTY slave ← 终端模拟器(iTerm2)
                       |
                       | TTY 驱动 echo 开启
                       v
                   屏幕上显示响应内容（乱码）
```

## 排查过程

### 尝试 1：Kitty 协议 quiet 标志 `q=2`

Kitty 图形协议规范定义了 `q` 参数来控制响应行为：

- `q=0`（默认）：终端发送所有响应
- `q=1`：仅发送错误响应，抑制 OK
- `q=2`：抑制所有响应

修改了 `kitty_display()` 的首包格式：

```rust
// 添加 q=2 参数
"\x1b_Gf=100,a=T,q=2,m={m};{chunk}\x1b\\"
```

**结果：无效。** iTerm2 不遵守 `q=2` 标志，仍然发送 OK 响应。

### 尝试 2：程序退出前 `tcflush` 清空 stdin

在 `main()` 末尾添加 `tcflush(STDIN_FILENO, TCIFLUSH)` 来丢弃 stdin 缓冲区中积累的响应：

```rust
fn drain_terminal_responses() {
    let _ = io::stdout().flush();
    std::thread::sleep(Duration::from_millis(50));
    unsafe { libc::tcflush(libc::STDIN_FILENO, libc::TCIFLUSH); }
}
```

**结果：部分有效。** 减少了乱码数量，但仍有残留。原因：`tcflush` 只能丢弃调用时刻 stdin 缓冲区中的内容。如果响应在 flush 之后才到达（因为终端处理图片需要时间），则无法被清除。

### 尝试 3：每张图片输出后 flush + drain

将 drain 逻辑移到每张图片输出之后，确保 BufWriter flush 后立即清空响应：

```rust
// markdown.rs 中，每次输出图片后
let _ = out.flush();
render::drain_kitty_responses();
```

**结果：部分有效，但乱码位置改变。** 问题的本质没有解决——`tcflush` 丢弃的是 stdin 缓冲区中的字节，但 TTY echo 机制在字节进入缓冲区的**瞬间**就已经把它们显示到屏幕上了。`tcflush` 能阻止后续 `read()` 读到这些字节，但无法撤回已经显示在屏幕上的内容。

### 尝试 4（最终方案）：禁用 TTY echo

既然问题的根源是 TTY 驱动的 echo 机制，直接在渲染前禁用 echo：

```rust
fn disable_echo() -> libc::termios {
    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut termios);
        let saved = termios;
        termios.c_lflag &= !libc::ECHO;
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &termios);
        saved
    }
}
```

完整流程：

1. 保存当前 termios 状态
2. 清除 `ECHO` 标志位
3. 执行所有 Markdown 渲染（包括 Kitty 图形协议输出）
4. `tcflush` 清空 stdin 缓冲区中积累的响应
5. 恢复原始 termios 状态

**结果：完全解决。** 终端响应仍然被写入 stdin 缓冲区，但 TTY 驱动不再将其回显到屏幕上。渲染结束后 `tcflush` 清除缓冲区，恢复 echo 后 shell 正常工作。

## 最终实现

涉及文件：

- `src/main.rs`：在 `markdown::render()` 前后管理 termios 状态
- `src/render.rs`：`drain_kitty_responses()` 函数 + `q=2` 保留在协议序列中
- `Cargo.toml`：新增 `libc` 依赖

`q=2` 虽然对 iTerm2 无效，但保留它是正确的——对于遵守协议规范的终端（Ghostty、Kitty 等），`q=2` 可以从源头避免响应产生，echo 禁用只是作为兜底方案。

## 兼容性

| 终端 | 是否受此 bug 影响 | `q=2` 是否有效 | echo 禁用是否安全 |
|------|:-:|:-:|:-:|
| Ghostty | 否 | 是 | 是（无副作用） |
| Kitty | 否 | 是 | 是（无副作用） |
| WezTerm | 否 | 是 | 是（无副作用） |
| iTerm2 | **是** | 否 | **是（解决问题）** |

对于不受影响的终端，禁用 echo 是一个无害操作——因为在正常渲染过程中应用本来就不需要读取用户输入。

## 经验总结

1. **终端协议规范 ≠ 终端实际行为。** iTerm2 声称支持 Kitty 图形协议，但不遵守 `q=2` quiet 标志。处理终端兼容性时不能只看规范，要实测每个终端的行为。

2. **理解数据流经过的每一层。** 这个 bug 涉及三层：应用层（Kitty 协议命令）→ PTY/TTY 驱动层（echo 机制）→ 终端模拟器层（响应生成）。只盯着应用层或协议层是找不到解决方案的。

3. **`tcflush` 清空的是缓冲区内容，不是已经显示的内容。** 这是一个容易混淆的点：丢弃 stdin 缓冲区并不能撤回 TTY echo 已经输出到屏幕上的字节。

4. **修复方案要在正确的层级操作。** 协议层的 `q=2` 和缓冲区层的 `tcflush` 都不够，最终需要在 TTY 驱动层禁用 echo 才能彻底解决。
