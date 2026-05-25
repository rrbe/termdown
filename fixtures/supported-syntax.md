---
title: Supported Syntax Showcase
author: termdown
tags: [markdown, fixture]
description: One comprehensive fixture for every Markdown feature termdown supports today or is on the roadmap to support.
---

# Supported Syntax Showcase

This fixture is the **single source of truth** for syntax termdown currently supports
or has explicitly committed to supporting (see `TODO.md`). When you add or change a
renderer feature, you should expect to update the snapshot of *this* file.

The YAML frontmatter above is part of the "markdown metadata" roadmap feature. It
currently leaks into the rendered output and should one day be hidden.

## 1. Headings

This section exercises all six heading levels, depth nesting, repeated titles, and
long-title wrapping — covering both the heading rasterizer and the TUI table-of-contents
panel.

### 1.1 Levels

# H1 标题 / Heading One

## H2 标题 / Heading Two

### H3 标题 / Heading Three

#### H4 — 四级标题

##### H5 — 五级标题

###### H6 — 六级标题 (the deepest level Markdown supports)

### 1.2 Headings with emoji & mixed scripts

# 🚀 Launch builders, not just code

## ✨ 标题里的 emoji 和中文混排

### 🧪 Body Text Baseline · 正文基线

### 1.3 Repeated titles (slug disambiguation)

#### Duplicate

#### Duplicate

### 1.4 Very long heading that should wrap inside a narrow TOC panel across multiple lines

Filler paragraph so the long heading above is followed by some body content.

## 2. Emphasis

_Italic via underscores_, *italic via asterisks*.

**Bold via double asterisks**, __bold via double underscores__.

_You **can** combine them_, and **bold _with_ nested italic** is fine too.

~~Strikethrough~~ via GFM tilde syntax.

_斜体_ · **粗体** · _**粗斜体组合**_ · ~~中文删除线~~

## 3. Lists

### 3.1 Unordered, nested

- Item 1
- Item 2
  - Item 2a
  - Item 2b
    - Item 2b.i
    - Item 2b.ii
- Item 3

### 3.2 Ordered, nested

1. Step one
2. Step two
   1. Sub-step 2.1
   2. Sub-step 2.2
3. Step three

### 3.3 中英混排列表

- 第一项 (item 1)
- 第二项 (item 2)
  - 嵌套 a / nested a
  - 嵌套 b / nested b

## 4. Task lists

### 4.1 Project TODO

- [x] Setup project structure
- [x] Add markdown parser
- [ ] Implement task list rendering
- [ ] Add configuration options
- [ ] Write documentation

### 4.2 Nested tasks

- [x] Phase 1
  - [x] Design architecture
  - [x] Write prototype
  - [ ] Code review
- [ ] Phase 2
  - [ ] Performance optimization
  - [ ] Integration testing

### 4.3 Mixed with regular list items & inline formatting

1. Ordered item one
2. Ordered item two

- [x] Completed task
- [ ] Pending task
- Regular list item (no checkbox)
- [x] Another done item
- [x] ~~Completed and struck through~~
- [ ] Task with **bold** and _italic_ text
- [ ] Task with `inline code`

## 5. Links

Inline link: [Markdown Live Preview](https://markdownlivepreview.com/).

Link with title: [termdown source](https://github.com/rrbe/termdown "Source on GitHub").

Reference-style link: [reference example][ref-1].

[ref-1]: https://example.com/ref "Reference target"

Bare URL (GFM autolink): https://example.com/docs/readme.html — bare email:
support@example.com — these are part of the autolink roadmap.

## 6. Images

> Roadmap feature — `TODO.md` 中"图片支持"。当前 termdown 渲染图片的能力还不完整，本节
> 既是覆盖目标也是回归 fixture。

Local image (does not exist on disk — exercises the broken-path branch):

![local image alt](./fixtures/nonexistent.png "An image title")

Local image with relative path that *does* exist (fixtures/links/sub/b.md as a
placeholder target — termdown should warn or skip gracefully):

![relative target](./links/sub/b.md "Not actually an image")

Remote image (no network in tests — exercises the remote-skip branch):

![remote image](https://example.com/banner.png "Banner")

Reference-style image:

![ref image][banner]

[banner]: https://example.com/banner.png "Banner title"

## 7. Blockquotes

> Markdown is a lightweight markup language with plain-text-formatting syntax,
> created in 2004 by John Gruber with Aaron Swartz.
>
> > Nested quote: Markdown is often used to format readme files and to create rich
> > text using a plain text editor.

> 中文引用块：Markdown 是一种轻量级标记语言，使用纯文本格式语法。
>
> > 嵌套引用：常用于编写 readme 文件与在线论坛中的消息格式化。

## 8. Tables

### 8.1 Three alignment modes

| Left | Center | Right |
| :--- | :----: | ----: |
| a    |   b    |     c |
| long left content | center | 1 |
| x    | **bold** in cell | `code` |

### 8.2 Mixed scripts & emoji in cells

| Case               | Example              |
| ------------------ | -------------------- |
| Single emoji       | 😀                   |
| Mixed text         | 修正版 ✨ version 2   |
| Symbol-like        | ✅ ⚠ ❌               |
| Variation selector | ☀️ ❤️ ⭐️              |

## 9. Code blocks

Fenced block with language tag:

```rust
fn main() {
    let greeting = "Hello, termdown!";
    println!("{}", greeting);
}
```

Fenced block, no language tag:

```
let message = 'Hello world';
alert(message);
```

Indented code block:

    fn indented() {
        // four-space indent
    }

Inline code: this web site is using `markedjs/marked`, and shell command
`cargo run -- fixtures/supported-syntax.md` should render this file.

## 10. HTML tags

> Roadmap feature — `TODO.md` 中"测试 html 标签支持"。覆盖块级 HTML、行内 HTML、HTML 注释。

A raw block HTML element:

<div style="padding: 8px; border: 1px solid #ccc;">
  <strong>Hello</strong> from inline HTML.
</div>

Inline HTML in a paragraph: this word is <u>underlined via HTML</u>, this one is
<b>bold via HTML</b>, and this one is <span style="color: red">red via HTML</span>.
An inline <br/> break and an <abbr title="HyperText Markup Language">HTML</abbr>
abbreviation.

An HTML comment: <!-- this comment should not appear --> end of line.

Self-closing void element: line one<br/>line two on the same paragraph.

## 11. Long-text wrap & indent

> Roadmap feature — `TODO.md` 中"长文本换行时缩进的处理"。本节专门用于验证：长行换行后
> 续行的缩进是否与首行对齐（特别是在列表、引用、标题里）。

### 11.1 Long paragraph

This is a deliberately long English paragraph that should wrap multiple times in a narrow terminal so we can verify that continuation lines start flush with the left margin and do not get an accidental indent. 这是一段足够长的中文段落，用来验证终端窗口较窄时，软换行产生的续行应该与首行左对齐而不应该出现额外缩进，特别是混排了 emoji 🚀 与英文 ASCII 之后宽度计算仍然正确。

### 11.2 Long line in a nested list

- 顶级列表项 / Top-level list item
  - 第二层项目，这一项里写一段非常长的中英混排正文，用来验证在嵌套列表内部软换行后续行是否能够正确缩进到列表项内容的起始列，而不是退到列表标记本身的位置 — this nested item should wrap with its continuation lines aligned to the start of the item text, not to the bullet marker.
    - 再嵌一层 / Deeper nest with another long line that should preserve the deeper indent across wrapping. This continuation should be indented even further to the right than the previous item's continuation lines.

### 11.3 Long line in a blockquote

> 引用块里的长行也需要正确换行：这是一段被包在 `>` 引用标记里的中英混排长文，软换行
> 后的续行需要与首行同一列对齐，而不应该跑到引用块外面去。And the same expectation in
> English — wrapped continuation lines must remain inside the blockquote prefix
> rather than escape to the outer margin.

### 11.4 Long heading (continuation indent test)

#### 一个超长四级标题，包含中文、English、还有 emoji 🚀，用来验证标题在窄终端中换行后续行的缩进表现

After the long heading above, this short paragraph anchors the test so the heading is followed by body content.

## 12. Emoji & 中英混排

### 12.1 Inline emoji

- 单个 emoji: 😀 😎 ✨ 🚀
- 中英混排: Hello 世界 🌍
- 符号混排: ✅ Done · ⚠ Warning · ❌ Failed
- 常见 emoji 变体: ☀️ ❤️ ⭐️

### 12.2 Complex ZWJ sequences

观察复杂 ZWJ emoji 的边界表现：👨‍👩‍👧‍👦 👩🏽‍💻 🧑‍🚀

### 12.3 Bidirectional & mixed-script paragraph

正文仍然由终端自身字体渲染，所以这里主要用来对比标题图片化与正文文本化的表现差异。
Body text mixes 中文、English、ASCII punctuation, and emoji 🌍 within the same line.

> 引用块里也放几个字符：💡 🛠 📦

## End

如果以上每一节都能正确（或按当前能力部分正确）渲染，termdown 的核心语法覆盖就达标了。
The snapshot baseline freezes what's "current" — when a roadmap feature lands, the
snapshot will diff and the new baseline records the improvement.
