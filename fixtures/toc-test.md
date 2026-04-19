# TOC Test Document

A document with a deep, varied heading hierarchy for exercising termdown's
TUI table-of-contents panel (toggle with the TOC key, then use ↑/↓/Enter to
jump between sections).

## 1. Introduction

Short intro paragraph to establish some body content under the first H2 so
scrolling the document moves the active TOC highlight.

### 1.1 Goals

- Exercise heading levels H1 through H6
- Verify TOC scroll-sync with the document
- Verify Enter jumps to the right section

### 1.2 Non-goals

Things deliberately **not** tested by this file:

1. Image rendering
2. Emoji fallback
3. Code block syntax highlighting

## 2. Shallow Section

Only one subsection here, to contrast with the deeper sections below.

### 2.1 Only child

Filler paragraph. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.

## 3. Deep Section

This section goes all the way down to H6 so the TOC has to render indent
levels correctly.

### 3.1 Level three

Paragraph at level three.

#### 3.1.1 Level four

Paragraph at level four.

##### 3.1.1.1 Level five

Paragraph at level five.

###### 3.1.1.1.1 Level six

Paragraph at level six — the deepest heading level Markdown supports.

###### 3.1.1.1.2 Another level six

A second H6 sibling, to make sure consecutive same-level headings both
appear in the TOC.

##### 3.1.1.2 Another level five

Back up to level five.

#### 3.1.2 Another level four

Back up to level four.

### 3.2 Sibling of 3.1

A sibling at level three, to verify the TOC collapses back to the correct
indent after a deep descent.

## 4. Section with Repeated Titles

Two subsections share the same title to check slug/id disambiguation in the
TOC (if implemented) and that both entries are independently navigable.

### 4.1 Duplicate

First occurrence.

### 4.1 Duplicate

Second occurrence.

## 5. Section with Long Title That Might Wrap in a Narrow TOC Panel Across Multiple Lines

The TOC panel has a fixed width; this heading is deliberately long enough
to test truncation or wrapping behavior.

### 5.1 Short child

Kept short so the contrast with the parent is obvious.

## 6. Section with Mixed Content

Headings alone are boring — this section includes a table, list, code
block, and blockquote between subheadings so navigation lands on headings
regardless of what surrounds them.

### 6.1 Table

| Col A | Col B | Col C |
| ----- | ----- | ----- |
| 1     | 2     | 3     |
| 4     | 5     | 6     |

### 6.2 Code

```rust
fn main() {
    println!("hello, toc");
}
```

### 6.3 Quote

> A heading preceded by a blockquote should still land correctly when
> selected from the TOC.

### 6.4 List

- alpha
- beta
  - beta.1
  - beta.2
- gamma

## 7. Tail Section

Final top-level section. Useful for checking that "jump to last heading"
lands at the bottom of the document rather than mid-scroll.

### 7.1 Last child

End of file.
