# Large Fixture — Stress / Performance Baseline

This file is **generated** by `scripts/gen-large-fixture.sh`. Do not hand-edit;
re-run the script and commit the diff. It exists to give us a manual baseline
for "does termdown stay snappy on a 200 KB / 10k-line document?" — it is **not**
wired into the snapshot tests.

## Section 1 — sample heading 1

This is paragraph one of section 1. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 1 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-1).

- 列表项 A of section 1
- 列表项 B of section 1
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 1

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 1a   | 1b   | 1c   |
| 中文 1 | English 1 | 🚀 1 |

```rust
fn section_1() -> usize {
    let n = 1;
    n * 2
}
```

## Section 2 — sample heading 2

This is paragraph one of section 2. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 2 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-2).

- 列表项 A of section 2
- 列表项 B of section 2
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 2

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 2a   | 2b   | 2c   |
| 中文 2 | English 2 | 🚀 2 |

```rust
fn section_2() -> usize {
    let n = 2;
    n * 2
}
```

## Section 3 — sample heading 3

This is paragraph one of section 3. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 3 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-3).

- 列表项 A of section 3
- 列表项 B of section 3
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 3

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 3a   | 3b   | 3c   |
| 中文 3 | English 3 | 🚀 3 |

```rust
fn section_3() -> usize {
    let n = 3;
    n * 2
}
```

## Section 4 — sample heading 4

This is paragraph one of section 4. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 4 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-4).

- 列表项 A of section 4
- 列表项 B of section 4
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 4

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 4a   | 4b   | 4c   |
| 中文 4 | English 4 | 🚀 4 |

```rust
fn section_4() -> usize {
    let n = 4;
    n * 2
}
```

## Section 5 — sample heading 5

This is paragraph one of section 5. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 5 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-5).

- 列表项 A of section 5
- 列表项 B of section 5
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 5

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 5a   | 5b   | 5c   |
| 中文 5 | English 5 | 🚀 5 |

```rust
fn section_5() -> usize {
    let n = 5;
    n * 2
}
```

## Section 6 — sample heading 6

This is paragraph one of section 6. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 6 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-6).

- 列表项 A of section 6
- 列表项 B of section 6
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 6

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 6a   | 6b   | 6c   |
| 中文 6 | English 6 | 🚀 6 |

```rust
fn section_6() -> usize {
    let n = 6;
    n * 2
}
```

## Section 7 — sample heading 7

This is paragraph one of section 7. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 7 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-7).

- 列表项 A of section 7
- 列表项 B of section 7
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 7

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 7a   | 7b   | 7c   |
| 中文 7 | English 7 | 🚀 7 |

```rust
fn section_7() -> usize {
    let n = 7;
    n * 2
}
```

## Section 8 — sample heading 8

This is paragraph one of section 8. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 8 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-8).

- 列表项 A of section 8
- 列表项 B of section 8
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 8

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 8a   | 8b   | 8c   |
| 中文 8 | English 8 | 🚀 8 |

```rust
fn section_8() -> usize {
    let n = 8;
    n * 2
}
```

## Section 9 — sample heading 9

This is paragraph one of section 9. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 9 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-9).

- 列表项 A of section 9
- 列表项 B of section 9
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 9

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 9a   | 9b   | 9c   |
| 中文 9 | English 9 | 🚀 9 |

```rust
fn section_9() -> usize {
    let n = 9;
    n * 2
}
```

## Section 10 — sample heading 10

This is paragraph one of section 10. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 10 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-10).

- 列表项 A of section 10
- 列表项 B of section 10
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 10

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 10a   | 10b   | 10c   |
| 中文 10 | English 10 | 🚀 10 |

```rust
fn section_10() -> usize {
    let n = 10;
    n * 2
}
```

## Section 11 — sample heading 11

This is paragraph one of section 11. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 11 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-11).

- 列表项 A of section 11
- 列表项 B of section 11
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 11

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 11a   | 11b   | 11c   |
| 中文 11 | English 11 | 🚀 11 |

```rust
fn section_11() -> usize {
    let n = 11;
    n * 2
}
```

## Section 12 — sample heading 12

This is paragraph one of section 12. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 12 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-12).

- 列表项 A of section 12
- 列表项 B of section 12
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 12

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 12a   | 12b   | 12c   |
| 中文 12 | English 12 | 🚀 12 |

```rust
fn section_12() -> usize {
    let n = 12;
    n * 2
}
```

## Section 13 — sample heading 13

This is paragraph one of section 13. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 13 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-13).

- 列表项 A of section 13
- 列表项 B of section 13
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 13

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 13a   | 13b   | 13c   |
| 中文 13 | English 13 | 🚀 13 |

```rust
fn section_13() -> usize {
    let n = 13;
    n * 2
}
```

## Section 14 — sample heading 14

This is paragraph one of section 14. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 14 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-14).

- 列表项 A of section 14
- 列表项 B of section 14
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 14

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 14a   | 14b   | 14c   |
| 中文 14 | English 14 | 🚀 14 |

```rust
fn section_14() -> usize {
    let n = 14;
    n * 2
}
```

## Section 15 — sample heading 15

This is paragraph one of section 15. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 15 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-15).

- 列表项 A of section 15
- 列表项 B of section 15
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 15

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 15a   | 15b   | 15c   |
| 中文 15 | English 15 | 🚀 15 |

```rust
fn section_15() -> usize {
    let n = 15;
    n * 2
}
```

## Section 16 — sample heading 16

This is paragraph one of section 16. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 16 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-16).

- 列表项 A of section 16
- 列表项 B of section 16
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 16

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 16a   | 16b   | 16c   |
| 中文 16 | English 16 | 🚀 16 |

```rust
fn section_16() -> usize {
    let n = 16;
    n * 2
}
```

## Section 17 — sample heading 17

This is paragraph one of section 17. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 17 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-17).

- 列表项 A of section 17
- 列表项 B of section 17
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 17

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 17a   | 17b   | 17c   |
| 中文 17 | English 17 | 🚀 17 |

```rust
fn section_17() -> usize {
    let n = 17;
    n * 2
}
```

## Section 18 — sample heading 18

This is paragraph one of section 18. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 18 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-18).

- 列表项 A of section 18
- 列表项 B of section 18
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 18

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 18a   | 18b   | 18c   |
| 中文 18 | English 18 | 🚀 18 |

```rust
fn section_18() -> usize {
    let n = 18;
    n * 2
}
```

## Section 19 — sample heading 19

This is paragraph one of section 19. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 19 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-19).

- 列表项 A of section 19
- 列表项 B of section 19
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 19

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 19a   | 19b   | 19c   |
| 中文 19 | English 19 | 🚀 19 |

```rust
fn section_19() -> usize {
    let n = 19;
    n * 2
}
```

## Section 20 — sample heading 20

This is paragraph one of section 20. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 20 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-20).

- 列表项 A of section 20
- 列表项 B of section 20
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 20

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 20a   | 20b   | 20c   |
| 中文 20 | English 20 | 🚀 20 |

```rust
fn section_20() -> usize {
    let n = 20;
    n * 2
}
```

## Section 21 — sample heading 21

This is paragraph one of section 21. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 21 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-21).

- 列表项 A of section 21
- 列表项 B of section 21
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 21

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 21a   | 21b   | 21c   |
| 中文 21 | English 21 | 🚀 21 |

```rust
fn section_21() -> usize {
    let n = 21;
    n * 2
}
```

## Section 22 — sample heading 22

This is paragraph one of section 22. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 22 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-22).

- 列表项 A of section 22
- 列表项 B of section 22
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 22

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 22a   | 22b   | 22c   |
| 中文 22 | English 22 | 🚀 22 |

```rust
fn section_22() -> usize {
    let n = 22;
    n * 2
}
```

## Section 23 — sample heading 23

This is paragraph one of section 23. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 23 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-23).

- 列表项 A of section 23
- 列表项 B of section 23
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 23

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 23a   | 23b   | 23c   |
| 中文 23 | English 23 | 🚀 23 |

```rust
fn section_23() -> usize {
    let n = 23;
    n * 2
}
```

## Section 24 — sample heading 24

This is paragraph one of section 24. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 24 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-24).

- 列表项 A of section 24
- 列表项 B of section 24
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 24

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 24a   | 24b   | 24c   |
| 中文 24 | English 24 | 🚀 24 |

```rust
fn section_24() -> usize {
    let n = 24;
    n * 2
}
```

## Section 25 — sample heading 25

This is paragraph one of section 25. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 25 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-25).

- 列表项 A of section 25
- 列表项 B of section 25
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 25

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 25a   | 25b   | 25c   |
| 中文 25 | English 25 | 🚀 25 |

```rust
fn section_25() -> usize {
    let n = 25;
    n * 2
}
```

## Section 26 — sample heading 26

This is paragraph one of section 26. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 26 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-26).

- 列表项 A of section 26
- 列表项 B of section 26
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 26

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 26a   | 26b   | 26c   |
| 中文 26 | English 26 | 🚀 26 |

```rust
fn section_26() -> usize {
    let n = 26;
    n * 2
}
```

## Section 27 — sample heading 27

This is paragraph one of section 27. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 27 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-27).

- 列表项 A of section 27
- 列表项 B of section 27
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 27

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 27a   | 27b   | 27c   |
| 中文 27 | English 27 | 🚀 27 |

```rust
fn section_27() -> usize {
    let n = 27;
    n * 2
}
```

## Section 28 — sample heading 28

This is paragraph one of section 28. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 28 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-28).

- 列表项 A of section 28
- 列表项 B of section 28
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 28

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 28a   | 28b   | 28c   |
| 中文 28 | English 28 | 🚀 28 |

```rust
fn section_28() -> usize {
    let n = 28;
    n * 2
}
```

## Section 29 — sample heading 29

This is paragraph one of section 29. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 29 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-29).

- 列表项 A of section 29
- 列表项 B of section 29
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 29

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 29a   | 29b   | 29c   |
| 中文 29 | English 29 | 🚀 29 |

```rust
fn section_29() -> usize {
    let n = 29;
    n * 2
}
```

## Section 30 — sample heading 30

This is paragraph one of section 30. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 30 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-30).

- 列表项 A of section 30
- 列表项 B of section 30
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 30

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 30a   | 30b   | 30c   |
| 中文 30 | English 30 | 🚀 30 |

```rust
fn section_30() -> usize {
    let n = 30;
    n * 2
}
```

## Section 31 — sample heading 31

This is paragraph one of section 31. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 31 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-31).

- 列表项 A of section 31
- 列表项 B of section 31
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 31

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 31a   | 31b   | 31c   |
| 中文 31 | English 31 | 🚀 31 |

```rust
fn section_31() -> usize {
    let n = 31;
    n * 2
}
```

## Section 32 — sample heading 32

This is paragraph one of section 32. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 32 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-32).

- 列表项 A of section 32
- 列表项 B of section 32
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 32

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 32a   | 32b   | 32c   |
| 中文 32 | English 32 | 🚀 32 |

```rust
fn section_32() -> usize {
    let n = 32;
    n * 2
}
```

## Section 33 — sample heading 33

This is paragraph one of section 33. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 33 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-33).

- 列表项 A of section 33
- 列表项 B of section 33
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 33

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 33a   | 33b   | 33c   |
| 中文 33 | English 33 | 🚀 33 |

```rust
fn section_33() -> usize {
    let n = 33;
    n * 2
}
```

## Section 34 — sample heading 34

This is paragraph one of section 34. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 34 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-34).

- 列表项 A of section 34
- 列表项 B of section 34
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 34

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 34a   | 34b   | 34c   |
| 中文 34 | English 34 | 🚀 34 |

```rust
fn section_34() -> usize {
    let n = 34;
    n * 2
}
```

## Section 35 — sample heading 35

This is paragraph one of section 35. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 35 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-35).

- 列表项 A of section 35
- 列表项 B of section 35
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 35

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 35a   | 35b   | 35c   |
| 中文 35 | English 35 | 🚀 35 |

```rust
fn section_35() -> usize {
    let n = 35;
    n * 2
}
```

## Section 36 — sample heading 36

This is paragraph one of section 36. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 36 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-36).

- 列表项 A of section 36
- 列表项 B of section 36
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 36

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 36a   | 36b   | 36c   |
| 中文 36 | English 36 | 🚀 36 |

```rust
fn section_36() -> usize {
    let n = 36;
    n * 2
}
```

## Section 37 — sample heading 37

This is paragraph one of section 37. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 37 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-37).

- 列表项 A of section 37
- 列表项 B of section 37
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 37

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 37a   | 37b   | 37c   |
| 中文 37 | English 37 | 🚀 37 |

```rust
fn section_37() -> usize {
    let n = 37;
    n * 2
}
```

## Section 38 — sample heading 38

This is paragraph one of section 38. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 38 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-38).

- 列表项 A of section 38
- 列表项 B of section 38
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 38

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 38a   | 38b   | 38c   |
| 中文 38 | English 38 | 🚀 38 |

```rust
fn section_38() -> usize {
    let n = 38;
    n * 2
}
```

## Section 39 — sample heading 39

This is paragraph one of section 39. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 39 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-39).

- 列表项 A of section 39
- 列表项 B of section 39
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 39

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 39a   | 39b   | 39c   |
| 中文 39 | English 39 | 🚀 39 |

```rust
fn section_39() -> usize {
    let n = 39;
    n * 2
}
```

## Section 40 — sample heading 40

This is paragraph one of section 40. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 40 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-40).

- 列表项 A of section 40
- 列表项 B of section 40
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 40

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 40a   | 40b   | 40c   |
| 中文 40 | English 40 | 🚀 40 |

```rust
fn section_40() -> usize {
    let n = 40;
    n * 2
}
```

## Section 41 — sample heading 41

This is paragraph one of section 41. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 41 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-41).

- 列表项 A of section 41
- 列表项 B of section 41
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 41

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 41a   | 41b   | 41c   |
| 中文 41 | English 41 | 🚀 41 |

```rust
fn section_41() -> usize {
    let n = 41;
    n * 2
}
```

## Section 42 — sample heading 42

This is paragraph one of section 42. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 42 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-42).

- 列表项 A of section 42
- 列表项 B of section 42
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 42

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 42a   | 42b   | 42c   |
| 中文 42 | English 42 | 🚀 42 |

```rust
fn section_42() -> usize {
    let n = 42;
    n * 2
}
```

## Section 43 — sample heading 43

This is paragraph one of section 43. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 43 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-43).

- 列表项 A of section 43
- 列表项 B of section 43
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 43

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 43a   | 43b   | 43c   |
| 中文 43 | English 43 | 🚀 43 |

```rust
fn section_43() -> usize {
    let n = 43;
    n * 2
}
```

## Section 44 — sample heading 44

This is paragraph one of section 44. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 44 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-44).

- 列表项 A of section 44
- 列表项 B of section 44
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 44

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 44a   | 44b   | 44c   |
| 中文 44 | English 44 | 🚀 44 |

```rust
fn section_44() -> usize {
    let n = 44;
    n * 2
}
```

## Section 45 — sample heading 45

This is paragraph one of section 45. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 45 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-45).

- 列表项 A of section 45
- 列表项 B of section 45
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 45

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 45a   | 45b   | 45c   |
| 中文 45 | English 45 | 🚀 45 |

```rust
fn section_45() -> usize {
    let n = 45;
    n * 2
}
```

## Section 46 — sample heading 46

This is paragraph one of section 46. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 46 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-46).

- 列表项 A of section 46
- 列表项 B of section 46
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 46

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 46a   | 46b   | 46c   |
| 中文 46 | English 46 | 🚀 46 |

```rust
fn section_46() -> usize {
    let n = 46;
    n * 2
}
```

## Section 47 — sample heading 47

This is paragraph one of section 47. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 47 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-47).

- 列表项 A of section 47
- 列表项 B of section 47
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 47

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 47a   | 47b   | 47c   |
| 中文 47 | English 47 | 🚀 47 |

```rust
fn section_47() -> usize {
    let n = 47;
    n * 2
}
```

## Section 48 — sample heading 48

This is paragraph one of section 48. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 48 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-48).

- 列表项 A of section 48
- 列表项 B of section 48
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 48

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 48a   | 48b   | 48c   |
| 中文 48 | English 48 | 🚀 48 |

```rust
fn section_48() -> usize {
    let n = 48;
    n * 2
}
```

## Section 49 — sample heading 49

This is paragraph one of section 49. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 49 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-49).

- 列表项 A of section 49
- 列表项 B of section 49
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 49

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 49a   | 49b   | 49c   |
| 中文 49 | English 49 | 🚀 49 |

```rust
fn section_49() -> usize {
    let n = 49;
    n * 2
}
```

## Section 50 — sample heading 50

This is paragraph one of section 50. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 50 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-50).

- 列表项 A of section 50
- 列表项 B of section 50
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 50

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 50a   | 50b   | 50c   |
| 中文 50 | English 50 | 🚀 50 |

```rust
fn section_50() -> usize {
    let n = 50;
    n * 2
}
```

### Section 50.1 — extra depth marker

Every 50th section gets an H3 sub-section to vary the heading depth
across the document.

## Section 51 — sample heading 51

This is paragraph one of section 51. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 51 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-51).

- 列表项 A of section 51
- 列表项 B of section 51
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 51

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 51a   | 51b   | 51c   |
| 中文 51 | English 51 | 🚀 51 |

```rust
fn section_51() -> usize {
    let n = 51;
    n * 2
}
```

## Section 52 — sample heading 52

This is paragraph one of section 52. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 52 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-52).

- 列表项 A of section 52
- 列表项 B of section 52
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 52

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 52a   | 52b   | 52c   |
| 中文 52 | English 52 | 🚀 52 |

```rust
fn section_52() -> usize {
    let n = 52;
    n * 2
}
```

## Section 53 — sample heading 53

This is paragraph one of section 53. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 53 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-53).

- 列表项 A of section 53
- 列表项 B of section 53
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 53

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 53a   | 53b   | 53c   |
| 中文 53 | English 53 | 🚀 53 |

```rust
fn section_53() -> usize {
    let n = 53;
    n * 2
}
```

## Section 54 — sample heading 54

This is paragraph one of section 54. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 54 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-54).

- 列表项 A of section 54
- 列表项 B of section 54
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 54

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 54a   | 54b   | 54c   |
| 中文 54 | English 54 | 🚀 54 |

```rust
fn section_54() -> usize {
    let n = 54;
    n * 2
}
```

## Section 55 — sample heading 55

This is paragraph one of section 55. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 55 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-55).

- 列表项 A of section 55
- 列表项 B of section 55
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 55

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 55a   | 55b   | 55c   |
| 中文 55 | English 55 | 🚀 55 |

```rust
fn section_55() -> usize {
    let n = 55;
    n * 2
}
```

## Section 56 — sample heading 56

This is paragraph one of section 56. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 56 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-56).

- 列表项 A of section 56
- 列表项 B of section 56
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 56

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 56a   | 56b   | 56c   |
| 中文 56 | English 56 | 🚀 56 |

```rust
fn section_56() -> usize {
    let n = 56;
    n * 2
}
```

## Section 57 — sample heading 57

This is paragraph one of section 57. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 57 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-57).

- 列表项 A of section 57
- 列表项 B of section 57
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 57

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 57a   | 57b   | 57c   |
| 中文 57 | English 57 | 🚀 57 |

```rust
fn section_57() -> usize {
    let n = 57;
    n * 2
}
```

## Section 58 — sample heading 58

This is paragraph one of section 58. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 58 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-58).

- 列表项 A of section 58
- 列表项 B of section 58
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 58

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 58a   | 58b   | 58c   |
| 中文 58 | English 58 | 🚀 58 |

```rust
fn section_58() -> usize {
    let n = 58;
    n * 2
}
```

## Section 59 — sample heading 59

This is paragraph one of section 59. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 59 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-59).

- 列表项 A of section 59
- 列表项 B of section 59
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 59

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 59a   | 59b   | 59c   |
| 中文 59 | English 59 | 🚀 59 |

```rust
fn section_59() -> usize {
    let n = 59;
    n * 2
}
```

## Section 60 — sample heading 60

This is paragraph one of section 60. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 60 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-60).

- 列表项 A of section 60
- 列表项 B of section 60
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 60

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 60a   | 60b   | 60c   |
| 中文 60 | English 60 | 🚀 60 |

```rust
fn section_60() -> usize {
    let n = 60;
    n * 2
}
```

## Section 61 — sample heading 61

This is paragraph one of section 61. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 61 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-61).

- 列表项 A of section 61
- 列表项 B of section 61
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 61

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 61a   | 61b   | 61c   |
| 中文 61 | English 61 | 🚀 61 |

```rust
fn section_61() -> usize {
    let n = 61;
    n * 2
}
```

## Section 62 — sample heading 62

This is paragraph one of section 62. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 62 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-62).

- 列表项 A of section 62
- 列表项 B of section 62
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 62

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 62a   | 62b   | 62c   |
| 中文 62 | English 62 | 🚀 62 |

```rust
fn section_62() -> usize {
    let n = 62;
    n * 2
}
```

## Section 63 — sample heading 63

This is paragraph one of section 63. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 63 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-63).

- 列表项 A of section 63
- 列表项 B of section 63
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 63

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 63a   | 63b   | 63c   |
| 中文 63 | English 63 | 🚀 63 |

```rust
fn section_63() -> usize {
    let n = 63;
    n * 2
}
```

## Section 64 — sample heading 64

This is paragraph one of section 64. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 64 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-64).

- 列表项 A of section 64
- 列表项 B of section 64
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 64

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 64a   | 64b   | 64c   |
| 中文 64 | English 64 | 🚀 64 |

```rust
fn section_64() -> usize {
    let n = 64;
    n * 2
}
```

## Section 65 — sample heading 65

This is paragraph one of section 65. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 65 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-65).

- 列表项 A of section 65
- 列表项 B of section 65
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 65

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 65a   | 65b   | 65c   |
| 中文 65 | English 65 | 🚀 65 |

```rust
fn section_65() -> usize {
    let n = 65;
    n * 2
}
```

## Section 66 — sample heading 66

This is paragraph one of section 66. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 66 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-66).

- 列表项 A of section 66
- 列表项 B of section 66
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 66

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 66a   | 66b   | 66c   |
| 中文 66 | English 66 | 🚀 66 |

```rust
fn section_66() -> usize {
    let n = 66;
    n * 2
}
```

## Section 67 — sample heading 67

This is paragraph one of section 67. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 67 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-67).

- 列表项 A of section 67
- 列表项 B of section 67
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 67

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 67a   | 67b   | 67c   |
| 中文 67 | English 67 | 🚀 67 |

```rust
fn section_67() -> usize {
    let n = 67;
    n * 2
}
```

## Section 68 — sample heading 68

This is paragraph one of section 68. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 68 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-68).

- 列表项 A of section 68
- 列表项 B of section 68
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 68

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 68a   | 68b   | 68c   |
| 中文 68 | English 68 | 🚀 68 |

```rust
fn section_68() -> usize {
    let n = 68;
    n * 2
}
```

## Section 69 — sample heading 69

This is paragraph one of section 69. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 69 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-69).

- 列表项 A of section 69
- 列表项 B of section 69
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 69

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 69a   | 69b   | 69c   |
| 中文 69 | English 69 | 🚀 69 |

```rust
fn section_69() -> usize {
    let n = 69;
    n * 2
}
```

## Section 70 — sample heading 70

This is paragraph one of section 70. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 70 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-70).

- 列表项 A of section 70
- 列表项 B of section 70
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 70

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 70a   | 70b   | 70c   |
| 中文 70 | English 70 | 🚀 70 |

```rust
fn section_70() -> usize {
    let n = 70;
    n * 2
}
```

## Section 71 — sample heading 71

This is paragraph one of section 71. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 71 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-71).

- 列表项 A of section 71
- 列表项 B of section 71
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 71

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 71a   | 71b   | 71c   |
| 中文 71 | English 71 | 🚀 71 |

```rust
fn section_71() -> usize {
    let n = 71;
    n * 2
}
```

## Section 72 — sample heading 72

This is paragraph one of section 72. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 72 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-72).

- 列表项 A of section 72
- 列表项 B of section 72
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 72

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 72a   | 72b   | 72c   |
| 中文 72 | English 72 | 🚀 72 |

```rust
fn section_72() -> usize {
    let n = 72;
    n * 2
}
```

## Section 73 — sample heading 73

This is paragraph one of section 73. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 73 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-73).

- 列表项 A of section 73
- 列表项 B of section 73
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 73

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 73a   | 73b   | 73c   |
| 中文 73 | English 73 | 🚀 73 |

```rust
fn section_73() -> usize {
    let n = 73;
    n * 2
}
```

## Section 74 — sample heading 74

This is paragraph one of section 74. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 74 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-74).

- 列表项 A of section 74
- 列表项 B of section 74
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 74

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 74a   | 74b   | 74c   |
| 中文 74 | English 74 | 🚀 74 |

```rust
fn section_74() -> usize {
    let n = 74;
    n * 2
}
```

## Section 75 — sample heading 75

This is paragraph one of section 75. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 75 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-75).

- 列表项 A of section 75
- 列表项 B of section 75
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 75

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 75a   | 75b   | 75c   |
| 中文 75 | English 75 | 🚀 75 |

```rust
fn section_75() -> usize {
    let n = 75;
    n * 2
}
```

## Section 76 — sample heading 76

This is paragraph one of section 76. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 76 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-76).

- 列表项 A of section 76
- 列表项 B of section 76
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 76

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 76a   | 76b   | 76c   |
| 中文 76 | English 76 | 🚀 76 |

```rust
fn section_76() -> usize {
    let n = 76;
    n * 2
}
```

## Section 77 — sample heading 77

This is paragraph one of section 77. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 77 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-77).

- 列表项 A of section 77
- 列表项 B of section 77
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 77

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 77a   | 77b   | 77c   |
| 中文 77 | English 77 | 🚀 77 |

```rust
fn section_77() -> usize {
    let n = 77;
    n * 2
}
```

## Section 78 — sample heading 78

This is paragraph one of section 78. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 78 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-78).

- 列表项 A of section 78
- 列表项 B of section 78
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 78

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 78a   | 78b   | 78c   |
| 中文 78 | English 78 | 🚀 78 |

```rust
fn section_78() -> usize {
    let n = 78;
    n * 2
}
```

## Section 79 — sample heading 79

This is paragraph one of section 79. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 79 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-79).

- 列表项 A of section 79
- 列表项 B of section 79
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 79

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 79a   | 79b   | 79c   |
| 中文 79 | English 79 | 🚀 79 |

```rust
fn section_79() -> usize {
    let n = 79;
    n * 2
}
```

## Section 80 — sample heading 80

This is paragraph one of section 80. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 80 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-80).

- 列表项 A of section 80
- 列表项 B of section 80
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 80

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 80a   | 80b   | 80c   |
| 中文 80 | English 80 | 🚀 80 |

```rust
fn section_80() -> usize {
    let n = 80;
    n * 2
}
```

## Section 81 — sample heading 81

This is paragraph one of section 81. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 81 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-81).

- 列表项 A of section 81
- 列表项 B of section 81
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 81

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 81a   | 81b   | 81c   |
| 中文 81 | English 81 | 🚀 81 |

```rust
fn section_81() -> usize {
    let n = 81;
    n * 2
}
```

## Section 82 — sample heading 82

This is paragraph one of section 82. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 82 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-82).

- 列表项 A of section 82
- 列表项 B of section 82
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 82

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 82a   | 82b   | 82c   |
| 中文 82 | English 82 | 🚀 82 |

```rust
fn section_82() -> usize {
    let n = 82;
    n * 2
}
```

## Section 83 — sample heading 83

This is paragraph one of section 83. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 83 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-83).

- 列表项 A of section 83
- 列表项 B of section 83
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 83

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 83a   | 83b   | 83c   |
| 中文 83 | English 83 | 🚀 83 |

```rust
fn section_83() -> usize {
    let n = 83;
    n * 2
}
```

## Section 84 — sample heading 84

This is paragraph one of section 84. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 84 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-84).

- 列表项 A of section 84
- 列表项 B of section 84
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 84

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 84a   | 84b   | 84c   |
| 中文 84 | English 84 | 🚀 84 |

```rust
fn section_84() -> usize {
    let n = 84;
    n * 2
}
```

## Section 85 — sample heading 85

This is paragraph one of section 85. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 85 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-85).

- 列表项 A of section 85
- 列表项 B of section 85
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 85

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 85a   | 85b   | 85c   |
| 中文 85 | English 85 | 🚀 85 |

```rust
fn section_85() -> usize {
    let n = 85;
    n * 2
}
```

## Section 86 — sample heading 86

This is paragraph one of section 86. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 86 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-86).

- 列表项 A of section 86
- 列表项 B of section 86
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 86

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 86a   | 86b   | 86c   |
| 中文 86 | English 86 | 🚀 86 |

```rust
fn section_86() -> usize {
    let n = 86;
    n * 2
}
```

## Section 87 — sample heading 87

This is paragraph one of section 87. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 87 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-87).

- 列表项 A of section 87
- 列表项 B of section 87
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 87

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 87a   | 87b   | 87c   |
| 中文 87 | English 87 | 🚀 87 |

```rust
fn section_87() -> usize {
    let n = 87;
    n * 2
}
```

## Section 88 — sample heading 88

This is paragraph one of section 88. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 88 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-88).

- 列表项 A of section 88
- 列表项 B of section 88
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 88

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 88a   | 88b   | 88c   |
| 中文 88 | English 88 | 🚀 88 |

```rust
fn section_88() -> usize {
    let n = 88;
    n * 2
}
```

## Section 89 — sample heading 89

This is paragraph one of section 89. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 89 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-89).

- 列表项 A of section 89
- 列表项 B of section 89
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 89

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 89a   | 89b   | 89c   |
| 中文 89 | English 89 | 🚀 89 |

```rust
fn section_89() -> usize {
    let n = 89;
    n * 2
}
```

## Section 90 — sample heading 90

This is paragraph one of section 90. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 90 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-90).

- 列表项 A of section 90
- 列表项 B of section 90
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 90

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 90a   | 90b   | 90c   |
| 中文 90 | English 90 | 🚀 90 |

```rust
fn section_90() -> usize {
    let n = 90;
    n * 2
}
```

## Section 91 — sample heading 91

This is paragraph one of section 91. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 91 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-91).

- 列表项 A of section 91
- 列表项 B of section 91
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 91

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 91a   | 91b   | 91c   |
| 中文 91 | English 91 | 🚀 91 |

```rust
fn section_91() -> usize {
    let n = 91;
    n * 2
}
```

## Section 92 — sample heading 92

This is paragraph one of section 92. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 92 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-92).

- 列表项 A of section 92
- 列表项 B of section 92
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 92

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 92a   | 92b   | 92c   |
| 中文 92 | English 92 | 🚀 92 |

```rust
fn section_92() -> usize {
    let n = 92;
    n * 2
}
```

## Section 93 — sample heading 93

This is paragraph one of section 93. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 93 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-93).

- 列表项 A of section 93
- 列表项 B of section 93
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 93

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 93a   | 93b   | 93c   |
| 中文 93 | English 93 | 🚀 93 |

```rust
fn section_93() -> usize {
    let n = 93;
    n * 2
}
```

## Section 94 — sample heading 94

This is paragraph one of section 94. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 94 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-94).

- 列表项 A of section 94
- 列表项 B of section 94
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 94

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 94a   | 94b   | 94c   |
| 中文 94 | English 94 | 🚀 94 |

```rust
fn section_94() -> usize {
    let n = 94;
    n * 2
}
```

## Section 95 — sample heading 95

This is paragraph one of section 95. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 95 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-95).

- 列表项 A of section 95
- 列表项 B of section 95
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 95

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 95a   | 95b   | 95c   |
| 中文 95 | English 95 | 🚀 95 |

```rust
fn section_95() -> usize {
    let n = 95;
    n * 2
}
```

## Section 96 — sample heading 96

This is paragraph one of section 96. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 96 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-96).

- 列表项 A of section 96
- 列表项 B of section 96
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 96

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 96a   | 96b   | 96c   |
| 中文 96 | English 96 | 🚀 96 |

```rust
fn section_96() -> usize {
    let n = 96;
    n * 2
}
```

## Section 97 — sample heading 97

This is paragraph one of section 97. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 97 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-97).

- 列表项 A of section 97
- 列表项 B of section 97
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 97

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 97a   | 97b   | 97c   |
| 中文 97 | English 97 | 🚀 97 |

```rust
fn section_97() -> usize {
    let n = 97;
    n * 2
}
```

## Section 98 — sample heading 98

This is paragraph one of section 98. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 98 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-98).

- 列表项 A of section 98
- 列表项 B of section 98
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 98

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 98a   | 98b   | 98c   |
| 中文 98 | English 98 | 🚀 98 |

```rust
fn section_98() -> usize {
    let n = 98;
    n * 2
}
```

## Section 99 — sample heading 99

This is paragraph one of section 99. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 99 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-99).

- 列表项 A of section 99
- 列表项 B of section 99
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 99

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 99a   | 99b   | 99c   |
| 中文 99 | English 99 | 🚀 99 |

```rust
fn section_99() -> usize {
    let n = 99;
    n * 2
}
```

## Section 100 — sample heading 100

This is paragraph one of section 100. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 100 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-100).

- 列表项 A of section 100
- 列表项 B of section 100
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 100

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 100a   | 100b   | 100c   |
| 中文 100 | English 100 | 🚀 100 |

```rust
fn section_100() -> usize {
    let n = 100;
    n * 2
}
```

### Section 100.1 — extra depth marker

Every 50th section gets an H3 sub-section to vary the heading depth
across the document.

## Section 101 — sample heading 101

This is paragraph one of section 101. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 101 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-101).

- 列表项 A of section 101
- 列表项 B of section 101
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 101

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 101a   | 101b   | 101c   |
| 中文 101 | English 101 | 🚀 101 |

```rust
fn section_101() -> usize {
    let n = 101;
    n * 2
}
```

## Section 102 — sample heading 102

This is paragraph one of section 102. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 102 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-102).

- 列表项 A of section 102
- 列表项 B of section 102
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 102

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 102a   | 102b   | 102c   |
| 中文 102 | English 102 | 🚀 102 |

```rust
fn section_102() -> usize {
    let n = 102;
    n * 2
}
```

## Section 103 — sample heading 103

This is paragraph one of section 103. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 103 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-103).

- 列表项 A of section 103
- 列表项 B of section 103
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 103

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 103a   | 103b   | 103c   |
| 中文 103 | English 103 | 🚀 103 |

```rust
fn section_103() -> usize {
    let n = 103;
    n * 2
}
```

## Section 104 — sample heading 104

This is paragraph one of section 104. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 104 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-104).

- 列表项 A of section 104
- 列表项 B of section 104
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 104

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 104a   | 104b   | 104c   |
| 中文 104 | English 104 | 🚀 104 |

```rust
fn section_104() -> usize {
    let n = 104;
    n * 2
}
```

## Section 105 — sample heading 105

This is paragraph one of section 105. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 105 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-105).

- 列表项 A of section 105
- 列表项 B of section 105
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 105

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 105a   | 105b   | 105c   |
| 中文 105 | English 105 | 🚀 105 |

```rust
fn section_105() -> usize {
    let n = 105;
    n * 2
}
```

## Section 106 — sample heading 106

This is paragraph one of section 106. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 106 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-106).

- 列表项 A of section 106
- 列表项 B of section 106
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 106

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 106a   | 106b   | 106c   |
| 中文 106 | English 106 | 🚀 106 |

```rust
fn section_106() -> usize {
    let n = 106;
    n * 2
}
```

## Section 107 — sample heading 107

This is paragraph one of section 107. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 107 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-107).

- 列表项 A of section 107
- 列表项 B of section 107
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 107

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 107a   | 107b   | 107c   |
| 中文 107 | English 107 | 🚀 107 |

```rust
fn section_107() -> usize {
    let n = 107;
    n * 2
}
```

## Section 108 — sample heading 108

This is paragraph one of section 108. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 108 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-108).

- 列表项 A of section 108
- 列表项 B of section 108
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 108

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 108a   | 108b   | 108c   |
| 中文 108 | English 108 | 🚀 108 |

```rust
fn section_108() -> usize {
    let n = 108;
    n * 2
}
```

## Section 109 — sample heading 109

This is paragraph one of section 109. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 109 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-109).

- 列表项 A of section 109
- 列表项 B of section 109
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 109

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 109a   | 109b   | 109c   |
| 中文 109 | English 109 | 🚀 109 |

```rust
fn section_109() -> usize {
    let n = 109;
    n * 2
}
```

## Section 110 — sample heading 110

This is paragraph one of section 110. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 110 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-110).

- 列表项 A of section 110
- 列表项 B of section 110
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 110

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 110a   | 110b   | 110c   |
| 中文 110 | English 110 | 🚀 110 |

```rust
fn section_110() -> usize {
    let n = 110;
    n * 2
}
```

## Section 111 — sample heading 111

This is paragraph one of section 111. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 111 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-111).

- 列表项 A of section 111
- 列表项 B of section 111
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 111

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 111a   | 111b   | 111c   |
| 中文 111 | English 111 | 🚀 111 |

```rust
fn section_111() -> usize {
    let n = 111;
    n * 2
}
```

## Section 112 — sample heading 112

This is paragraph one of section 112. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 112 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-112).

- 列表项 A of section 112
- 列表项 B of section 112
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 112

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 112a   | 112b   | 112c   |
| 中文 112 | English 112 | 🚀 112 |

```rust
fn section_112() -> usize {
    let n = 112;
    n * 2
}
```

## Section 113 — sample heading 113

This is paragraph one of section 113. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 113 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-113).

- 列表项 A of section 113
- 列表项 B of section 113
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 113

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 113a   | 113b   | 113c   |
| 中文 113 | English 113 | 🚀 113 |

```rust
fn section_113() -> usize {
    let n = 113;
    n * 2
}
```

## Section 114 — sample heading 114

This is paragraph one of section 114. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 114 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-114).

- 列表项 A of section 114
- 列表项 B of section 114
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 114

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 114a   | 114b   | 114c   |
| 中文 114 | English 114 | 🚀 114 |

```rust
fn section_114() -> usize {
    let n = 114;
    n * 2
}
```

## Section 115 — sample heading 115

This is paragraph one of section 115. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 115 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-115).

- 列表项 A of section 115
- 列表项 B of section 115
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 115

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 115a   | 115b   | 115c   |
| 中文 115 | English 115 | 🚀 115 |

```rust
fn section_115() -> usize {
    let n = 115;
    n * 2
}
```

## Section 116 — sample heading 116

This is paragraph one of section 116. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 116 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-116).

- 列表项 A of section 116
- 列表项 B of section 116
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 116

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 116a   | 116b   | 116c   |
| 中文 116 | English 116 | 🚀 116 |

```rust
fn section_116() -> usize {
    let n = 116;
    n * 2
}
```

## Section 117 — sample heading 117

This is paragraph one of section 117. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 117 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-117).

- 列表项 A of section 117
- 列表项 B of section 117
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 117

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 117a   | 117b   | 117c   |
| 中文 117 | English 117 | 🚀 117 |

```rust
fn section_117() -> usize {
    let n = 117;
    n * 2
}
```

## Section 118 — sample heading 118

This is paragraph one of section 118. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 118 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-118).

- 列表项 A of section 118
- 列表项 B of section 118
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 118

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 118a   | 118b   | 118c   |
| 中文 118 | English 118 | 🚀 118 |

```rust
fn section_118() -> usize {
    let n = 118;
    n * 2
}
```

## Section 119 — sample heading 119

This is paragraph one of section 119. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 119 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-119).

- 列表项 A of section 119
- 列表项 B of section 119
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 119

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 119a   | 119b   | 119c   |
| 中文 119 | English 119 | 🚀 119 |

```rust
fn section_119() -> usize {
    let n = 119;
    n * 2
}
```

## Section 120 — sample heading 120

This is paragraph one of section 120. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 120 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-120).

- 列表项 A of section 120
- 列表项 B of section 120
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 120

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 120a   | 120b   | 120c   |
| 中文 120 | English 120 | 🚀 120 |

```rust
fn section_120() -> usize {
    let n = 120;
    n * 2
}
```

## Section 121 — sample heading 121

This is paragraph one of section 121. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 121 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-121).

- 列表项 A of section 121
- 列表项 B of section 121
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 121

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 121a   | 121b   | 121c   |
| 中文 121 | English 121 | 🚀 121 |

```rust
fn section_121() -> usize {
    let n = 121;
    n * 2
}
```

## Section 122 — sample heading 122

This is paragraph one of section 122. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 122 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-122).

- 列表项 A of section 122
- 列表项 B of section 122
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 122

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 122a   | 122b   | 122c   |
| 中文 122 | English 122 | 🚀 122 |

```rust
fn section_122() -> usize {
    let n = 122;
    n * 2
}
```

## Section 123 — sample heading 123

This is paragraph one of section 123. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 123 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-123).

- 列表项 A of section 123
- 列表项 B of section 123
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 123

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 123a   | 123b   | 123c   |
| 中文 123 | English 123 | 🚀 123 |

```rust
fn section_123() -> usize {
    let n = 123;
    n * 2
}
```

## Section 124 — sample heading 124

This is paragraph one of section 124. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 124 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-124).

- 列表项 A of section 124
- 列表项 B of section 124
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 124

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 124a   | 124b   | 124c   |
| 中文 124 | English 124 | 🚀 124 |

```rust
fn section_124() -> usize {
    let n = 124;
    n * 2
}
```

## Section 125 — sample heading 125

This is paragraph one of section 125. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 125 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-125).

- 列表项 A of section 125
- 列表项 B of section 125
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 125

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 125a   | 125b   | 125c   |
| 中文 125 | English 125 | 🚀 125 |

```rust
fn section_125() -> usize {
    let n = 125;
    n * 2
}
```

## Section 126 — sample heading 126

This is paragraph one of section 126. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 126 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-126).

- 列表项 A of section 126
- 列表项 B of section 126
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 126

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 126a   | 126b   | 126c   |
| 中文 126 | English 126 | 🚀 126 |

```rust
fn section_126() -> usize {
    let n = 126;
    n * 2
}
```

## Section 127 — sample heading 127

This is paragraph one of section 127. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 127 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-127).

- 列表项 A of section 127
- 列表项 B of section 127
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 127

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 127a   | 127b   | 127c   |
| 中文 127 | English 127 | 🚀 127 |

```rust
fn section_127() -> usize {
    let n = 127;
    n * 2
}
```

## Section 128 — sample heading 128

This is paragraph one of section 128. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 128 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-128).

- 列表项 A of section 128
- 列表项 B of section 128
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 128

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 128a   | 128b   | 128c   |
| 中文 128 | English 128 | 🚀 128 |

```rust
fn section_128() -> usize {
    let n = 128;
    n * 2
}
```

## Section 129 — sample heading 129

This is paragraph one of section 129. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 129 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-129).

- 列表项 A of section 129
- 列表项 B of section 129
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 129

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 129a   | 129b   | 129c   |
| 中文 129 | English 129 | 🚀 129 |

```rust
fn section_129() -> usize {
    let n = 129;
    n * 2
}
```

## Section 130 — sample heading 130

This is paragraph one of section 130. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 130 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-130).

- 列表项 A of section 130
- 列表项 B of section 130
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 130

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 130a   | 130b   | 130c   |
| 中文 130 | English 130 | 🚀 130 |

```rust
fn section_130() -> usize {
    let n = 130;
    n * 2
}
```

## Section 131 — sample heading 131

This is paragraph one of section 131. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 131 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-131).

- 列表项 A of section 131
- 列表项 B of section 131
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 131

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 131a   | 131b   | 131c   |
| 中文 131 | English 131 | 🚀 131 |

```rust
fn section_131() -> usize {
    let n = 131;
    n * 2
}
```

## Section 132 — sample heading 132

This is paragraph one of section 132. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 132 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-132).

- 列表项 A of section 132
- 列表项 B of section 132
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 132

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 132a   | 132b   | 132c   |
| 中文 132 | English 132 | 🚀 132 |

```rust
fn section_132() -> usize {
    let n = 132;
    n * 2
}
```

## Section 133 — sample heading 133

This is paragraph one of section 133. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 133 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-133).

- 列表项 A of section 133
- 列表项 B of section 133
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 133

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 133a   | 133b   | 133c   |
| 中文 133 | English 133 | 🚀 133 |

```rust
fn section_133() -> usize {
    let n = 133;
    n * 2
}
```

## Section 134 — sample heading 134

This is paragraph one of section 134. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 134 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-134).

- 列表项 A of section 134
- 列表项 B of section 134
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 134

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 134a   | 134b   | 134c   |
| 中文 134 | English 134 | 🚀 134 |

```rust
fn section_134() -> usize {
    let n = 134;
    n * 2
}
```

## Section 135 — sample heading 135

This is paragraph one of section 135. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 135 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-135).

- 列表项 A of section 135
- 列表项 B of section 135
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 135

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 135a   | 135b   | 135c   |
| 中文 135 | English 135 | 🚀 135 |

```rust
fn section_135() -> usize {
    let n = 135;
    n * 2
}
```

## Section 136 — sample heading 136

This is paragraph one of section 136. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 136 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-136).

- 列表项 A of section 136
- 列表项 B of section 136
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 136

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 136a   | 136b   | 136c   |
| 中文 136 | English 136 | 🚀 136 |

```rust
fn section_136() -> usize {
    let n = 136;
    n * 2
}
```

## Section 137 — sample heading 137

This is paragraph one of section 137. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 137 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-137).

- 列表项 A of section 137
- 列表项 B of section 137
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 137

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 137a   | 137b   | 137c   |
| 中文 137 | English 137 | 🚀 137 |

```rust
fn section_137() -> usize {
    let n = 137;
    n * 2
}
```

## Section 138 — sample heading 138

This is paragraph one of section 138. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 138 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-138).

- 列表项 A of section 138
- 列表项 B of section 138
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 138

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 138a   | 138b   | 138c   |
| 中文 138 | English 138 | 🚀 138 |

```rust
fn section_138() -> usize {
    let n = 138;
    n * 2
}
```

## Section 139 — sample heading 139

This is paragraph one of section 139. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 139 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-139).

- 列表项 A of section 139
- 列表项 B of section 139
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 139

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 139a   | 139b   | 139c   |
| 中文 139 | English 139 | 🚀 139 |

```rust
fn section_139() -> usize {
    let n = 139;
    n * 2
}
```

## Section 140 — sample heading 140

This is paragraph one of section 140. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 140 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-140).

- 列表项 A of section 140
- 列表项 B of section 140
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 140

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 140a   | 140b   | 140c   |
| 中文 140 | English 140 | 🚀 140 |

```rust
fn section_140() -> usize {
    let n = 140;
    n * 2
}
```

## Section 141 — sample heading 141

This is paragraph one of section 141. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 141 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-141).

- 列表项 A of section 141
- 列表项 B of section 141
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 141

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 141a   | 141b   | 141c   |
| 中文 141 | English 141 | 🚀 141 |

```rust
fn section_141() -> usize {
    let n = 141;
    n * 2
}
```

## Section 142 — sample heading 142

This is paragraph one of section 142. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 142 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-142).

- 列表项 A of section 142
- 列表项 B of section 142
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 142

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 142a   | 142b   | 142c   |
| 中文 142 | English 142 | 🚀 142 |

```rust
fn section_142() -> usize {
    let n = 142;
    n * 2
}
```

## Section 143 — sample heading 143

This is paragraph one of section 143. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 143 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-143).

- 列表项 A of section 143
- 列表项 B of section 143
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 143

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 143a   | 143b   | 143c   |
| 中文 143 | English 143 | 🚀 143 |

```rust
fn section_143() -> usize {
    let n = 143;
    n * 2
}
```

## Section 144 — sample heading 144

This is paragraph one of section 144. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 144 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-144).

- 列表项 A of section 144
- 列表项 B of section 144
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 144

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 144a   | 144b   | 144c   |
| 中文 144 | English 144 | 🚀 144 |

```rust
fn section_144() -> usize {
    let n = 144;
    n * 2
}
```

## Section 145 — sample heading 145

This is paragraph one of section 145. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 145 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-145).

- 列表项 A of section 145
- 列表项 B of section 145
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 145

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 145a   | 145b   | 145c   |
| 中文 145 | English 145 | 🚀 145 |

```rust
fn section_145() -> usize {
    let n = 145;
    n * 2
}
```

## Section 146 — sample heading 146

This is paragraph one of section 146. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 146 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-146).

- 列表项 A of section 146
- 列表项 B of section 146
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 146

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 146a   | 146b   | 146c   |
| 中文 146 | English 146 | 🚀 146 |

```rust
fn section_146() -> usize {
    let n = 146;
    n * 2
}
```

## Section 147 — sample heading 147

This is paragraph one of section 147. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 147 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-147).

- 列表项 A of section 147
- 列表项 B of section 147
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 147

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 147a   | 147b   | 147c   |
| 中文 147 | English 147 | 🚀 147 |

```rust
fn section_147() -> usize {
    let n = 147;
    n * 2
}
```

## Section 148 — sample heading 148

This is paragraph one of section 148. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 148 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-148).

- 列表项 A of section 148
- 列表项 B of section 148
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 148

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 148a   | 148b   | 148c   |
| 中文 148 | English 148 | 🚀 148 |

```rust
fn section_148() -> usize {
    let n = 148;
    n * 2
}
```

## Section 149 — sample heading 149

This is paragraph one of section 149. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 149 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-149).

- 列表项 A of section 149
- 列表项 B of section 149
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 149

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 149a   | 149b   | 149c   |
| 中文 149 | English 149 | 🚀 149 |

```rust
fn section_149() -> usize {
    let n = 149;
    n * 2
}
```

## Section 150 — sample heading 150

This is paragraph one of section 150. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 150 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-150).

- 列表项 A of section 150
- 列表项 B of section 150
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 150

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 150a   | 150b   | 150c   |
| 中文 150 | English 150 | 🚀 150 |

```rust
fn section_150() -> usize {
    let n = 150;
    n * 2
}
```

### Section 150.1 — extra depth marker

Every 50th section gets an H3 sub-section to vary the heading depth
across the document.

## Section 151 — sample heading 151

This is paragraph one of section 151. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 151 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-151).

- 列表项 A of section 151
- 列表项 B of section 151
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 151

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 151a   | 151b   | 151c   |
| 中文 151 | English 151 | 🚀 151 |

```rust
fn section_151() -> usize {
    let n = 151;
    n * 2
}
```

## Section 152 — sample heading 152

This is paragraph one of section 152. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 152 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-152).

- 列表项 A of section 152
- 列表项 B of section 152
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 152

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 152a   | 152b   | 152c   |
| 中文 152 | English 152 | 🚀 152 |

```rust
fn section_152() -> usize {
    let n = 152;
    n * 2
}
```

## Section 153 — sample heading 153

This is paragraph one of section 153. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 153 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-153).

- 列表项 A of section 153
- 列表项 B of section 153
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 153

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 153a   | 153b   | 153c   |
| 中文 153 | English 153 | 🚀 153 |

```rust
fn section_153() -> usize {
    let n = 153;
    n * 2
}
```

## Section 154 — sample heading 154

This is paragraph one of section 154. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 154 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-154).

- 列表项 A of section 154
- 列表项 B of section 154
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 154

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 154a   | 154b   | 154c   |
| 中文 154 | English 154 | 🚀 154 |

```rust
fn section_154() -> usize {
    let n = 154;
    n * 2
}
```

## Section 155 — sample heading 155

This is paragraph one of section 155. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 155 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-155).

- 列表项 A of section 155
- 列表项 B of section 155
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 155

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 155a   | 155b   | 155c   |
| 中文 155 | English 155 | 🚀 155 |

```rust
fn section_155() -> usize {
    let n = 155;
    n * 2
}
```

## Section 156 — sample heading 156

This is paragraph one of section 156. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 156 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-156).

- 列表项 A of section 156
- 列表项 B of section 156
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 156

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 156a   | 156b   | 156c   |
| 中文 156 | English 156 | 🚀 156 |

```rust
fn section_156() -> usize {
    let n = 156;
    n * 2
}
```

## Section 157 — sample heading 157

This is paragraph one of section 157. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 157 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-157).

- 列表项 A of section 157
- 列表项 B of section 157
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 157

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 157a   | 157b   | 157c   |
| 中文 157 | English 157 | 🚀 157 |

```rust
fn section_157() -> usize {
    let n = 157;
    n * 2
}
```

## Section 158 — sample heading 158

This is paragraph one of section 158. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 158 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-158).

- 列表项 A of section 158
- 列表项 B of section 158
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 158

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 158a   | 158b   | 158c   |
| 中文 158 | English 158 | 🚀 158 |

```rust
fn section_158() -> usize {
    let n = 158;
    n * 2
}
```

## Section 159 — sample heading 159

This is paragraph one of section 159. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 159 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-159).

- 列表项 A of section 159
- 列表项 B of section 159
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 159

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 159a   | 159b   | 159c   |
| 中文 159 | English 159 | 🚀 159 |

```rust
fn section_159() -> usize {
    let n = 159;
    n * 2
}
```

## Section 160 — sample heading 160

This is paragraph one of section 160. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 160 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-160).

- 列表项 A of section 160
- 列表项 B of section 160
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 160

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 160a   | 160b   | 160c   |
| 中文 160 | English 160 | 🚀 160 |

```rust
fn section_160() -> usize {
    let n = 160;
    n * 2
}
```

## Section 161 — sample heading 161

This is paragraph one of section 161. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 161 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-161).

- 列表项 A of section 161
- 列表项 B of section 161
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 161

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 161a   | 161b   | 161c   |
| 中文 161 | English 161 | 🚀 161 |

```rust
fn section_161() -> usize {
    let n = 161;
    n * 2
}
```

## Section 162 — sample heading 162

This is paragraph one of section 162. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 162 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-162).

- 列表项 A of section 162
- 列表项 B of section 162
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 162

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 162a   | 162b   | 162c   |
| 中文 162 | English 162 | 🚀 162 |

```rust
fn section_162() -> usize {
    let n = 162;
    n * 2
}
```

## Section 163 — sample heading 163

This is paragraph one of section 163. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 163 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-163).

- 列表项 A of section 163
- 列表项 B of section 163
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 163

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 163a   | 163b   | 163c   |
| 中文 163 | English 163 | 🚀 163 |

```rust
fn section_163() -> usize {
    let n = 163;
    n * 2
}
```

## Section 164 — sample heading 164

This is paragraph one of section 164. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 164 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-164).

- 列表项 A of section 164
- 列表项 B of section 164
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 164

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 164a   | 164b   | 164c   |
| 中文 164 | English 164 | 🚀 164 |

```rust
fn section_164() -> usize {
    let n = 164;
    n * 2
}
```

## Section 165 — sample heading 165

This is paragraph one of section 165. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 165 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-165).

- 列表项 A of section 165
- 列表项 B of section 165
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 165

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 165a   | 165b   | 165c   |
| 中文 165 | English 165 | 🚀 165 |

```rust
fn section_165() -> usize {
    let n = 165;
    n * 2
}
```

## Section 166 — sample heading 166

This is paragraph one of section 166. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 166 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-166).

- 列表项 A of section 166
- 列表项 B of section 166
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 166

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 166a   | 166b   | 166c   |
| 中文 166 | English 166 | 🚀 166 |

```rust
fn section_166() -> usize {
    let n = 166;
    n * 2
}
```

## Section 167 — sample heading 167

This is paragraph one of section 167. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 167 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-167).

- 列表项 A of section 167
- 列表项 B of section 167
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 167

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 167a   | 167b   | 167c   |
| 中文 167 | English 167 | 🚀 167 |

```rust
fn section_167() -> usize {
    let n = 167;
    n * 2
}
```

## Section 168 — sample heading 168

This is paragraph one of section 168. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 168 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-168).

- 列表项 A of section 168
- 列表项 B of section 168
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 168

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 168a   | 168b   | 168c   |
| 中文 168 | English 168 | 🚀 168 |

```rust
fn section_168() -> usize {
    let n = 168;
    n * 2
}
```

## Section 169 — sample heading 169

This is paragraph one of section 169. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 169 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-169).

- 列表项 A of section 169
- 列表项 B of section 169
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 169

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 169a   | 169b   | 169c   |
| 中文 169 | English 169 | 🚀 169 |

```rust
fn section_169() -> usize {
    let n = 169;
    n * 2
}
```

## Section 170 — sample heading 170

This is paragraph one of section 170. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 170 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-170).

- 列表项 A of section 170
- 列表项 B of section 170
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 170

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 170a   | 170b   | 170c   |
| 中文 170 | English 170 | 🚀 170 |

```rust
fn section_170() -> usize {
    let n = 170;
    n * 2
}
```

## Section 171 — sample heading 171

This is paragraph one of section 171. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 171 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-171).

- 列表项 A of section 171
- 列表项 B of section 171
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 171

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 171a   | 171b   | 171c   |
| 中文 171 | English 171 | 🚀 171 |

```rust
fn section_171() -> usize {
    let n = 171;
    n * 2
}
```

## Section 172 — sample heading 172

This is paragraph one of section 172. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 172 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-172).

- 列表项 A of section 172
- 列表项 B of section 172
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 172

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 172a   | 172b   | 172c   |
| 中文 172 | English 172 | 🚀 172 |

```rust
fn section_172() -> usize {
    let n = 172;
    n * 2
}
```

## Section 173 — sample heading 173

This is paragraph one of section 173. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 173 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-173).

- 列表项 A of section 173
- 列表项 B of section 173
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 173

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 173a   | 173b   | 173c   |
| 中文 173 | English 173 | 🚀 173 |

```rust
fn section_173() -> usize {
    let n = 173;
    n * 2
}
```

## Section 174 — sample heading 174

This is paragraph one of section 174. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 174 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-174).

- 列表项 A of section 174
- 列表项 B of section 174
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 174

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 174a   | 174b   | 174c   |
| 中文 174 | English 174 | 🚀 174 |

```rust
fn section_174() -> usize {
    let n = 174;
    n * 2
}
```

## Section 175 — sample heading 175

This is paragraph one of section 175. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 175 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-175).

- 列表项 A of section 175
- 列表项 B of section 175
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 175

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 175a   | 175b   | 175c   |
| 中文 175 | English 175 | 🚀 175 |

```rust
fn section_175() -> usize {
    let n = 175;
    n * 2
}
```

## Section 176 — sample heading 176

This is paragraph one of section 176. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 176 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-176).

- 列表项 A of section 176
- 列表项 B of section 176
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 176

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 176a   | 176b   | 176c   |
| 中文 176 | English 176 | 🚀 176 |

```rust
fn section_176() -> usize {
    let n = 176;
    n * 2
}
```

## Section 177 — sample heading 177

This is paragraph one of section 177. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 177 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-177).

- 列表项 A of section 177
- 列表项 B of section 177
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 177

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 177a   | 177b   | 177c   |
| 中文 177 | English 177 | 🚀 177 |

```rust
fn section_177() -> usize {
    let n = 177;
    n * 2
}
```

## Section 178 — sample heading 178

This is paragraph one of section 178. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 178 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-178).

- 列表项 A of section 178
- 列表项 B of section 178
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 178

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 178a   | 178b   | 178c   |
| 中文 178 | English 178 | 🚀 178 |

```rust
fn section_178() -> usize {
    let n = 178;
    n * 2
}
```

## Section 179 — sample heading 179

This is paragraph one of section 179. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 179 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-179).

- 列表项 A of section 179
- 列表项 B of section 179
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 179

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 179a   | 179b   | 179c   |
| 中文 179 | English 179 | 🚀 179 |

```rust
fn section_179() -> usize {
    let n = 179;
    n * 2
}
```

## Section 180 — sample heading 180

This is paragraph one of section 180. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 180 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-180).

- 列表项 A of section 180
- 列表项 B of section 180
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 180

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 180a   | 180b   | 180c   |
| 中文 180 | English 180 | 🚀 180 |

```rust
fn section_180() -> usize {
    let n = 180;
    n * 2
}
```

## Section 181 — sample heading 181

This is paragraph one of section 181. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 181 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-181).

- 列表项 A of section 181
- 列表项 B of section 181
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 181

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 181a   | 181b   | 181c   |
| 中文 181 | English 181 | 🚀 181 |

```rust
fn section_181() -> usize {
    let n = 181;
    n * 2
}
```

## Section 182 — sample heading 182

This is paragraph one of section 182. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 182 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-182).

- 列表项 A of section 182
- 列表项 B of section 182
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 182

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 182a   | 182b   | 182c   |
| 中文 182 | English 182 | 🚀 182 |

```rust
fn section_182() -> usize {
    let n = 182;
    n * 2
}
```

## Section 183 — sample heading 183

This is paragraph one of section 183. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 183 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-183).

- 列表项 A of section 183
- 列表项 B of section 183
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 183

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 183a   | 183b   | 183c   |
| 中文 183 | English 183 | 🚀 183 |

```rust
fn section_183() -> usize {
    let n = 183;
    n * 2
}
```

## Section 184 — sample heading 184

This is paragraph one of section 184. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 184 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-184).

- 列表项 A of section 184
- 列表项 B of section 184
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 184

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 184a   | 184b   | 184c   |
| 中文 184 | English 184 | 🚀 184 |

```rust
fn section_184() -> usize {
    let n = 184;
    n * 2
}
```

## Section 185 — sample heading 185

This is paragraph one of section 185. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 185 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-185).

- 列表项 A of section 185
- 列表项 B of section 185
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 185

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 185a   | 185b   | 185c   |
| 中文 185 | English 185 | 🚀 185 |

```rust
fn section_185() -> usize {
    let n = 185;
    n * 2
}
```

## Section 186 — sample heading 186

This is paragraph one of section 186. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 186 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-186).

- 列表项 A of section 186
- 列表项 B of section 186
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 186

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 186a   | 186b   | 186c   |
| 中文 186 | English 186 | 🚀 186 |

```rust
fn section_186() -> usize {
    let n = 186;
    n * 2
}
```

## Section 187 — sample heading 187

This is paragraph one of section 187. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 187 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-187).

- 列表项 A of section 187
- 列表项 B of section 187
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 187

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 187a   | 187b   | 187c   |
| 中文 187 | English 187 | 🚀 187 |

```rust
fn section_187() -> usize {
    let n = 187;
    n * 2
}
```

## Section 188 — sample heading 188

This is paragraph one of section 188. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 188 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-188).

- 列表项 A of section 188
- 列表项 B of section 188
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 188

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 188a   | 188b   | 188c   |
| 中文 188 | English 188 | 🚀 188 |

```rust
fn section_188() -> usize {
    let n = 188;
    n * 2
}
```

## Section 189 — sample heading 189

This is paragraph one of section 189. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 189 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-189).

- 列表项 A of section 189
- 列表项 B of section 189
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 189

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 189a   | 189b   | 189c   |
| 中文 189 | English 189 | 🚀 189 |

```rust
fn section_189() -> usize {
    let n = 189;
    n * 2
}
```

## Section 190 — sample heading 190

This is paragraph one of section 190. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 190 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-190).

- 列表项 A of section 190
- 列表项 B of section 190
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 190

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 190a   | 190b   | 190c   |
| 中文 190 | English 190 | 🚀 190 |

```rust
fn section_190() -> usize {
    let n = 190;
    n * 2
}
```

## Section 191 — sample heading 191

This is paragraph one of section 191. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 191 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-191).

- 列表项 A of section 191
- 列表项 B of section 191
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 191

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 191a   | 191b   | 191c   |
| 中文 191 | English 191 | 🚀 191 |

```rust
fn section_191() -> usize {
    let n = 191;
    n * 2
}
```

## Section 192 — sample heading 192

This is paragraph one of section 192. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 192 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-192).

- 列表项 A of section 192
- 列表项 B of section 192
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 192

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 192a   | 192b   | 192c   |
| 中文 192 | English 192 | 🚀 192 |

```rust
fn section_192() -> usize {
    let n = 192;
    n * 2
}
```

## Section 193 — sample heading 193

This is paragraph one of section 193. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 193 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-193).

- 列表项 A of section 193
- 列表项 B of section 193
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 193

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 193a   | 193b   | 193c   |
| 中文 193 | English 193 | 🚀 193 |

```rust
fn section_193() -> usize {
    let n = 193;
    n * 2
}
```

## Section 194 — sample heading 194

This is paragraph one of section 194. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 194 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-194).

- 列表项 A of section 194
- 列表项 B of section 194
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 194

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 194a   | 194b   | 194c   |
| 中文 194 | English 194 | 🚀 194 |

```rust
fn section_194() -> usize {
    let n = 194;
    n * 2
}
```

## Section 195 — sample heading 195

This is paragraph one of section 195. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 195 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-195).

- 列表项 A of section 195
- 列表项 B of section 195
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 195

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 195a   | 195b   | 195c   |
| 中文 195 | English 195 | 🚀 195 |

```rust
fn section_195() -> usize {
    let n = 195;
    n * 2
}
```

## Section 196 — sample heading 196

This is paragraph one of section 196. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 196 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-196).

- 列表项 A of section 196
- 列表项 B of section 196
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 196

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 196a   | 196b   | 196c   |
| 中文 196 | English 196 | 🚀 196 |

```rust
fn section_196() -> usize {
    let n = 196;
    n * 2
}
```

## Section 197 — sample heading 197

This is paragraph one of section 197. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 197 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-197).

- 列表项 A of section 197
- 列表项 B of section 197
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 197

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 197a   | 197b   | 197c   |
| 中文 197 | English 197 | 🚀 197 |

```rust
fn section_197() -> usize {
    let n = 197;
    n * 2
}
```

## Section 198 — sample heading 198

This is paragraph one of section 198. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 198 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-198).

- 列表项 A of section 198
- 列表项 B of section 198
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 198

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 198a   | 198b   | 198c   |
| 中文 198 | English 198 | 🚀 198 |

```rust
fn section_198() -> usize {
    let n = 198;
    n * 2
}
```

## Section 199 — sample heading 199

This is paragraph one of section 199. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 199 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-199).

- 列表项 A of section 199
- 列表项 B of section 199
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 199

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 199a   | 199b   | 199c   |
| 中文 199 | English 199 | 🚀 199 |

```rust
fn section_199() -> usize {
    let n = 199;
    n * 2
}
```

## Section 200 — sample heading 200

This is paragraph one of section 200. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 200 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-200).

- 列表项 A of section 200
- 列表项 B of section 200
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 200

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 200a   | 200b   | 200c   |
| 中文 200 | English 200 | 🚀 200 |

```rust
fn section_200() -> usize {
    let n = 200;
    n * 2
}
```

### Section 200.1 — extra depth marker

Every 50th section gets an H3 sub-section to vary the heading depth
across the document.

## Section 201 — sample heading 201

This is paragraph one of section 201. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 201 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-201).

- 列表项 A of section 201
- 列表项 B of section 201
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 201

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 201a   | 201b   | 201c   |
| 中文 201 | English 201 | 🚀 201 |

```rust
fn section_201() -> usize {
    let n = 201;
    n * 2
}
```

## Section 202 — sample heading 202

This is paragraph one of section 202. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 202 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-202).

- 列表项 A of section 202
- 列表项 B of section 202
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 202

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 202a   | 202b   | 202c   |
| 中文 202 | English 202 | 🚀 202 |

```rust
fn section_202() -> usize {
    let n = 202;
    n * 2
}
```

## Section 203 — sample heading 203

This is paragraph one of section 203. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 203 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-203).

- 列表项 A of section 203
- 列表项 B of section 203
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 203

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 203a   | 203b   | 203c   |
| 中文 203 | English 203 | 🚀 203 |

```rust
fn section_203() -> usize {
    let n = 203;
    n * 2
}
```

## Section 204 — sample heading 204

This is paragraph one of section 204. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 204 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-204).

- 列表项 A of section 204
- 列表项 B of section 204
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 204

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 204a   | 204b   | 204c   |
| 中文 204 | English 204 | 🚀 204 |

```rust
fn section_204() -> usize {
    let n = 204;
    n * 2
}
```

## Section 205 — sample heading 205

This is paragraph one of section 205. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 205 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-205).

- 列表项 A of section 205
- 列表项 B of section 205
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 205

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 205a   | 205b   | 205c   |
| 中文 205 | English 205 | 🚀 205 |

```rust
fn section_205() -> usize {
    let n = 205;
    n * 2
}
```

## Section 206 — sample heading 206

This is paragraph one of section 206. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 206 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-206).

- 列表项 A of section 206
- 列表项 B of section 206
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 206

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 206a   | 206b   | 206c   |
| 中文 206 | English 206 | 🚀 206 |

```rust
fn section_206() -> usize {
    let n = 206;
    n * 2
}
```

## Section 207 — sample heading 207

This is paragraph one of section 207. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 207 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-207).

- 列表项 A of section 207
- 列表项 B of section 207
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 207

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 207a   | 207b   | 207c   |
| 中文 207 | English 207 | 🚀 207 |

```rust
fn section_207() -> usize {
    let n = 207;
    n * 2
}
```

## Section 208 — sample heading 208

This is paragraph one of section 208. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 208 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-208).

- 列表项 A of section 208
- 列表项 B of section 208
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 208

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 208a   | 208b   | 208c   |
| 中文 208 | English 208 | 🚀 208 |

```rust
fn section_208() -> usize {
    let n = 208;
    n * 2
}
```

## Section 209 — sample heading 209

This is paragraph one of section 209. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 209 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-209).

- 列表项 A of section 209
- 列表项 B of section 209
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 209

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 209a   | 209b   | 209c   |
| 中文 209 | English 209 | 🚀 209 |

```rust
fn section_209() -> usize {
    let n = 209;
    n * 2
}
```

## Section 210 — sample heading 210

This is paragraph one of section 210. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 210 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-210).

- 列表项 A of section 210
- 列表项 B of section 210
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 210

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 210a   | 210b   | 210c   |
| 中文 210 | English 210 | 🚀 210 |

```rust
fn section_210() -> usize {
    let n = 210;
    n * 2
}
```

## Section 211 — sample heading 211

This is paragraph one of section 211. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 211 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-211).

- 列表项 A of section 211
- 列表项 B of section 211
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 211

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 211a   | 211b   | 211c   |
| 中文 211 | English 211 | 🚀 211 |

```rust
fn section_211() -> usize {
    let n = 211;
    n * 2
}
```

## Section 212 — sample heading 212

This is paragraph one of section 212. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 212 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-212).

- 列表项 A of section 212
- 列表项 B of section 212
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 212

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 212a   | 212b   | 212c   |
| 中文 212 | English 212 | 🚀 212 |

```rust
fn section_212() -> usize {
    let n = 212;
    n * 2
}
```

## Section 213 — sample heading 213

This is paragraph one of section 213. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 213 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-213).

- 列表项 A of section 213
- 列表项 B of section 213
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 213

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 213a   | 213b   | 213c   |
| 中文 213 | English 213 | 🚀 213 |

```rust
fn section_213() -> usize {
    let n = 213;
    n * 2
}
```

## Section 214 — sample heading 214

This is paragraph one of section 214. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 214 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-214).

- 列表项 A of section 214
- 列表项 B of section 214
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 214

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 214a   | 214b   | 214c   |
| 中文 214 | English 214 | 🚀 214 |

```rust
fn section_214() -> usize {
    let n = 214;
    n * 2
}
```

## Section 215 — sample heading 215

This is paragraph one of section 215. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 215 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-215).

- 列表项 A of section 215
- 列表项 B of section 215
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 215

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 215a   | 215b   | 215c   |
| 中文 215 | English 215 | 🚀 215 |

```rust
fn section_215() -> usize {
    let n = 215;
    n * 2
}
```

## Section 216 — sample heading 216

This is paragraph one of section 216. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 216 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-216).

- 列表项 A of section 216
- 列表项 B of section 216
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 216

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 216a   | 216b   | 216c   |
| 中文 216 | English 216 | 🚀 216 |

```rust
fn section_216() -> usize {
    let n = 216;
    n * 2
}
```

## Section 217 — sample heading 217

This is paragraph one of section 217. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 217 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-217).

- 列表项 A of section 217
- 列表项 B of section 217
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 217

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 217a   | 217b   | 217c   |
| 中文 217 | English 217 | 🚀 217 |

```rust
fn section_217() -> usize {
    let n = 217;
    n * 2
}
```

## Section 218 — sample heading 218

This is paragraph one of section 218. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 218 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-218).

- 列表项 A of section 218
- 列表项 B of section 218
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 218

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 218a   | 218b   | 218c   |
| 中文 218 | English 218 | 🚀 218 |

```rust
fn section_218() -> usize {
    let n = 218;
    n * 2
}
```

## Section 219 — sample heading 219

This is paragraph one of section 219. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 219 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-219).

- 列表项 A of section 219
- 列表项 B of section 219
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 219

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 219a   | 219b   | 219c   |
| 中文 219 | English 219 | 🚀 219 |

```rust
fn section_219() -> usize {
    let n = 219;
    n * 2
}
```

## Section 220 — sample heading 220

This is paragraph one of section 220. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 220 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-220).

- 列表项 A of section 220
- 列表项 B of section 220
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 220

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 220a   | 220b   | 220c   |
| 中文 220 | English 220 | 🚀 220 |

```rust
fn section_220() -> usize {
    let n = 220;
    n * 2
}
```

## Section 221 — sample heading 221

This is paragraph one of section 221. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 221 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-221).

- 列表项 A of section 221
- 列表项 B of section 221
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 221

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 221a   | 221b   | 221c   |
| 中文 221 | English 221 | 🚀 221 |

```rust
fn section_221() -> usize {
    let n = 221;
    n * 2
}
```

## Section 222 — sample heading 222

This is paragraph one of section 222. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 222 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-222).

- 列表项 A of section 222
- 列表项 B of section 222
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 222

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 222a   | 222b   | 222c   |
| 中文 222 | English 222 | 🚀 222 |

```rust
fn section_222() -> usize {
    let n = 222;
    n * 2
}
```

## Section 223 — sample heading 223

This is paragraph one of section 223. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 223 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-223).

- 列表项 A of section 223
- 列表项 B of section 223
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 223

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 223a   | 223b   | 223c   |
| 中文 223 | English 223 | 🚀 223 |

```rust
fn section_223() -> usize {
    let n = 223;
    n * 2
}
```

## Section 224 — sample heading 224

This is paragraph one of section 224. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 224 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-224).

- 列表项 A of section 224
- 列表项 B of section 224
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 224

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 224a   | 224b   | 224c   |
| 中文 224 | English 224 | 🚀 224 |

```rust
fn section_224() -> usize {
    let n = 224;
    n * 2
}
```

## Section 225 — sample heading 225

This is paragraph one of section 225. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 225 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-225).

- 列表项 A of section 225
- 列表项 B of section 225
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 225

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 225a   | 225b   | 225c   |
| 中文 225 | English 225 | 🚀 225 |

```rust
fn section_225() -> usize {
    let n = 225;
    n * 2
}
```

## Section 226 — sample heading 226

This is paragraph one of section 226. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 226 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-226).

- 列表项 A of section 226
- 列表项 B of section 226
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 226

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 226a   | 226b   | 226c   |
| 中文 226 | English 226 | 🚀 226 |

```rust
fn section_226() -> usize {
    let n = 226;
    n * 2
}
```

## Section 227 — sample heading 227

This is paragraph one of section 227. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 227 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-227).

- 列表项 A of section 227
- 列表项 B of section 227
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 227

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 227a   | 227b   | 227c   |
| 中文 227 | English 227 | 🚀 227 |

```rust
fn section_227() -> usize {
    let n = 227;
    n * 2
}
```

## Section 228 — sample heading 228

This is paragraph one of section 228. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 228 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-228).

- 列表项 A of section 228
- 列表项 B of section 228
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 228

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 228a   | 228b   | 228c   |
| 中文 228 | English 228 | 🚀 228 |

```rust
fn section_228() -> usize {
    let n = 228;
    n * 2
}
```

## Section 229 — sample heading 229

This is paragraph one of section 229. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 229 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-229).

- 列表项 A of section 229
- 列表项 B of section 229
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 229

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 229a   | 229b   | 229c   |
| 中文 229 | English 229 | 🚀 229 |

```rust
fn section_229() -> usize {
    let n = 229;
    n * 2
}
```

## Section 230 — sample heading 230

This is paragraph one of section 230. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 230 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-230).

- 列表项 A of section 230
- 列表项 B of section 230
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 230

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 230a   | 230b   | 230c   |
| 中文 230 | English 230 | 🚀 230 |

```rust
fn section_230() -> usize {
    let n = 230;
    n * 2
}
```

## Section 231 — sample heading 231

This is paragraph one of section 231. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 231 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-231).

- 列表项 A of section 231
- 列表项 B of section 231
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 231

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 231a   | 231b   | 231c   |
| 中文 231 | English 231 | 🚀 231 |

```rust
fn section_231() -> usize {
    let n = 231;
    n * 2
}
```

## Section 232 — sample heading 232

This is paragraph one of section 232. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 232 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-232).

- 列表项 A of section 232
- 列表项 B of section 232
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 232

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 232a   | 232b   | 232c   |
| 中文 232 | English 232 | 🚀 232 |

```rust
fn section_232() -> usize {
    let n = 232;
    n * 2
}
```

## Section 233 — sample heading 233

This is paragraph one of section 233. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 233 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-233).

- 列表项 A of section 233
- 列表项 B of section 233
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 233

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 233a   | 233b   | 233c   |
| 中文 233 | English 233 | 🚀 233 |

```rust
fn section_233() -> usize {
    let n = 233;
    n * 2
}
```

## Section 234 — sample heading 234

This is paragraph one of section 234. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 234 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-234).

- 列表项 A of section 234
- 列表项 B of section 234
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 234

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 234a   | 234b   | 234c   |
| 中文 234 | English 234 | 🚀 234 |

```rust
fn section_234() -> usize {
    let n = 234;
    n * 2
}
```

## Section 235 — sample heading 235

This is paragraph one of section 235. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 235 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-235).

- 列表项 A of section 235
- 列表项 B of section 235
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 235

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 235a   | 235b   | 235c   |
| 中文 235 | English 235 | 🚀 235 |

```rust
fn section_235() -> usize {
    let n = 235;
    n * 2
}
```

## Section 236 — sample heading 236

This is paragraph one of section 236. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 236 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-236).

- 列表项 A of section 236
- 列表项 B of section 236
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 236

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 236a   | 236b   | 236c   |
| 中文 236 | English 236 | 🚀 236 |

```rust
fn section_236() -> usize {
    let n = 236;
    n * 2
}
```

## Section 237 — sample heading 237

This is paragraph one of section 237. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 237 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-237).

- 列表项 A of section 237
- 列表项 B of section 237
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 237

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 237a   | 237b   | 237c   |
| 中文 237 | English 237 | 🚀 237 |

```rust
fn section_237() -> usize {
    let n = 237;
    n * 2
}
```

## Section 238 — sample heading 238

This is paragraph one of section 238. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 238 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-238).

- 列表项 A of section 238
- 列表项 B of section 238
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 238

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 238a   | 238b   | 238c   |
| 中文 238 | English 238 | 🚀 238 |

```rust
fn section_238() -> usize {
    let n = 238;
    n * 2
}
```

## Section 239 — sample heading 239

This is paragraph one of section 239. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 239 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-239).

- 列表项 A of section 239
- 列表项 B of section 239
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 239

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 239a   | 239b   | 239c   |
| 中文 239 | English 239 | 🚀 239 |

```rust
fn section_239() -> usize {
    let n = 239;
    n * 2
}
```

## Section 240 — sample heading 240

This is paragraph one of section 240. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 240 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-240).

- 列表项 A of section 240
- 列表项 B of section 240
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 240

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 240a   | 240b   | 240c   |
| 中文 240 | English 240 | 🚀 240 |

```rust
fn section_240() -> usize {
    let n = 240;
    n * 2
}
```

## Section 241 — sample heading 241

This is paragraph one of section 241. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 241 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-241).

- 列表项 A of section 241
- 列表项 B of section 241
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 241

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 241a   | 241b   | 241c   |
| 中文 241 | English 241 | 🚀 241 |

```rust
fn section_241() -> usize {
    let n = 241;
    n * 2
}
```

## Section 242 — sample heading 242

This is paragraph one of section 242. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 242 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-242).

- 列表项 A of section 242
- 列表项 B of section 242
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 242

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 242a   | 242b   | 242c   |
| 中文 242 | English 242 | 🚀 242 |

```rust
fn section_242() -> usize {
    let n = 242;
    n * 2
}
```

## Section 243 — sample heading 243

This is paragraph one of section 243. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 243 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-243).

- 列表项 A of section 243
- 列表项 B of section 243
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 243

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 243a   | 243b   | 243c   |
| 中文 243 | English 243 | 🚀 243 |

```rust
fn section_243() -> usize {
    let n = 243;
    n * 2
}
```

## Section 244 — sample heading 244

This is paragraph one of section 244. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 244 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-244).

- 列表项 A of section 244
- 列表项 B of section 244
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 244

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 244a   | 244b   | 244c   |
| 中文 244 | English 244 | 🚀 244 |

```rust
fn section_244() -> usize {
    let n = 244;
    n * 2
}
```

## Section 245 — sample heading 245

This is paragraph one of section 245. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 245 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-245).

- 列表项 A of section 245
- 列表项 B of section 245
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 245

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 245a   | 245b   | 245c   |
| 中文 245 | English 245 | 🚀 245 |

```rust
fn section_245() -> usize {
    let n = 245;
    n * 2
}
```

## Section 246 — sample heading 246

This is paragraph one of section 246. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 246 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-246).

- 列表项 A of section 246
- 列表项 B of section 246
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 246

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 246a   | 246b   | 246c   |
| 中文 246 | English 246 | 🚀 246 |

```rust
fn section_246() -> usize {
    let n = 246;
    n * 2
}
```

## Section 247 — sample heading 247

This is paragraph one of section 247. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 247 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-247).

- 列表项 A of section 247
- 列表项 B of section 247
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 247

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 247a   | 247b   | 247c   |
| 中文 247 | English 247 | 🚀 247 |

```rust
fn section_247() -> usize {
    let n = 247;
    n * 2
}
```

## Section 248 — sample heading 248

This is paragraph one of section 248. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 248 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-248).

- 列表项 A of section 248
- 列表项 B of section 248
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 248

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 248a   | 248b   | 248c   |
| 中文 248 | English 248 | 🚀 248 |

```rust
fn section_248() -> usize {
    let n = 248;
    n * 2
}
```

## Section 249 — sample heading 249

This is paragraph one of section 249. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 249 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-249).

- 列表项 A of section 249
- 列表项 B of section 249
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 249

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 249a   | 249b   | 249c   |
| 中文 249 | English 249 | 🚀 249 |

```rust
fn section_249() -> usize {
    let n = 249;
    n * 2
}
```

## Section 250 — sample heading 250

This is paragraph one of section 250. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 250 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-250).

- 列表项 A of section 250
- 列表项 B of section 250
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 250

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 250a   | 250b   | 250c   |
| 中文 250 | English 250 | 🚀 250 |

```rust
fn section_250() -> usize {
    let n = 250;
    n * 2
}
```

### Section 250.1 — extra depth marker

Every 50th section gets an H3 sub-section to vary the heading depth
across the document.

## Section 251 — sample heading 251

This is paragraph one of section 251. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 251 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-251).

- 列表项 A of section 251
- 列表项 B of section 251
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 251

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 251a   | 251b   | 251c   |
| 中文 251 | English 251 | 🚀 251 |

```rust
fn section_251() -> usize {
    let n = 251;
    n * 2
}
```

## Section 252 — sample heading 252

This is paragraph one of section 252. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 252 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-252).

- 列表项 A of section 252
- 列表项 B of section 252
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 252

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 252a   | 252b   | 252c   |
| 中文 252 | English 252 | 🚀 252 |

```rust
fn section_252() -> usize {
    let n = 252;
    n * 2
}
```

## Section 253 — sample heading 253

This is paragraph one of section 253. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 253 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-253).

- 列表项 A of section 253
- 列表项 B of section 253
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 253

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 253a   | 253b   | 253c   |
| 中文 253 | English 253 | 🚀 253 |

```rust
fn section_253() -> usize {
    let n = 253;
    n * 2
}
```

## Section 254 — sample heading 254

This is paragraph one of section 254. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 254 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-254).

- 列表项 A of section 254
- 列表项 B of section 254
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 254

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 254a   | 254b   | 254c   |
| 中文 254 | English 254 | 🚀 254 |

```rust
fn section_254() -> usize {
    let n = 254;
    n * 2
}
```

## Section 255 — sample heading 255

This is paragraph one of section 255. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 255 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-255).

- 列表项 A of section 255
- 列表项 B of section 255
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 255

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 255a   | 255b   | 255c   |
| 中文 255 | English 255 | 🚀 255 |

```rust
fn section_255() -> usize {
    let n = 255;
    n * 2
}
```

## Section 256 — sample heading 256

This is paragraph one of section 256. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 256 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-256).

- 列表项 A of section 256
- 列表项 B of section 256
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 256

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 256a   | 256b   | 256c   |
| 中文 256 | English 256 | 🚀 256 |

```rust
fn section_256() -> usize {
    let n = 256;
    n * 2
}
```

## Section 257 — sample heading 257

This is paragraph one of section 257. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 257 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-257).

- 列表项 A of section 257
- 列表项 B of section 257
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 257

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 257a   | 257b   | 257c   |
| 中文 257 | English 257 | 🚀 257 |

```rust
fn section_257() -> usize {
    let n = 257;
    n * 2
}
```

## Section 258 — sample heading 258

This is paragraph one of section 258. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 258 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-258).

- 列表项 A of section 258
- 列表项 B of section 258
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 258

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 258a   | 258b   | 258c   |
| 中文 258 | English 258 | 🚀 258 |

```rust
fn section_258() -> usize {
    let n = 258;
    n * 2
}
```

## Section 259 — sample heading 259

This is paragraph one of section 259. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 259 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-259).

- 列表项 A of section 259
- 列表项 B of section 259
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 259

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 259a   | 259b   | 259c   |
| 中文 259 | English 259 | 🚀 259 |

```rust
fn section_259() -> usize {
    let n = 259;
    n * 2
}
```

## Section 260 — sample heading 260

This is paragraph one of section 260. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 260 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-260).

- 列表项 A of section 260
- 列表项 B of section 260
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 260

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 260a   | 260b   | 260c   |
| 中文 260 | English 260 | 🚀 260 |

```rust
fn section_260() -> usize {
    let n = 260;
    n * 2
}
```

## Section 261 — sample heading 261

This is paragraph one of section 261. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 261 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-261).

- 列表项 A of section 261
- 列表项 B of section 261
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 261

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 261a   | 261b   | 261c   |
| 中文 261 | English 261 | 🚀 261 |

```rust
fn section_261() -> usize {
    let n = 261;
    n * 2
}
```

## Section 262 — sample heading 262

This is paragraph one of section 262. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 262 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-262).

- 列表项 A of section 262
- 列表项 B of section 262
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 262

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 262a   | 262b   | 262c   |
| 中文 262 | English 262 | 🚀 262 |

```rust
fn section_262() -> usize {
    let n = 262;
    n * 2
}
```

## Section 263 — sample heading 263

This is paragraph one of section 263. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 263 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-263).

- 列表项 A of section 263
- 列表项 B of section 263
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 263

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 263a   | 263b   | 263c   |
| 中文 263 | English 263 | 🚀 263 |

```rust
fn section_263() -> usize {
    let n = 263;
    n * 2
}
```

## Section 264 — sample heading 264

This is paragraph one of section 264. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 264 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-264).

- 列表项 A of section 264
- 列表项 B of section 264
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 264

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 264a   | 264b   | 264c   |
| 中文 264 | English 264 | 🚀 264 |

```rust
fn section_264() -> usize {
    let n = 264;
    n * 2
}
```

## Section 265 — sample heading 265

This is paragraph one of section 265. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 265 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-265).

- 列表项 A of section 265
- 列表项 B of section 265
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 265

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 265a   | 265b   | 265c   |
| 中文 265 | English 265 | 🚀 265 |

```rust
fn section_265() -> usize {
    let n = 265;
    n * 2
}
```

## Section 266 — sample heading 266

This is paragraph one of section 266. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 266 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-266).

- 列表项 A of section 266
- 列表项 B of section 266
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 266

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 266a   | 266b   | 266c   |
| 中文 266 | English 266 | 🚀 266 |

```rust
fn section_266() -> usize {
    let n = 266;
    n * 2
}
```

## Section 267 — sample heading 267

This is paragraph one of section 267. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 267 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-267).

- 列表项 A of section 267
- 列表项 B of section 267
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 267

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 267a   | 267b   | 267c   |
| 中文 267 | English 267 | 🚀 267 |

```rust
fn section_267() -> usize {
    let n = 267;
    n * 2
}
```

## Section 268 — sample heading 268

This is paragraph one of section 268. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 268 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-268).

- 列表项 A of section 268
- 列表项 B of section 268
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 268

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 268a   | 268b   | 268c   |
| 中文 268 | English 268 | 🚀 268 |

```rust
fn section_268() -> usize {
    let n = 268;
    n * 2
}
```

## Section 269 — sample heading 269

This is paragraph one of section 269. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 269 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-269).

- 列表项 A of section 269
- 列表项 B of section 269
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 269

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 269a   | 269b   | 269c   |
| 中文 269 | English 269 | 🚀 269 |

```rust
fn section_269() -> usize {
    let n = 269;
    n * 2
}
```

## Section 270 — sample heading 270

This is paragraph one of section 270. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 270 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-270).

- 列表项 A of section 270
- 列表项 B of section 270
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 270

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 270a   | 270b   | 270c   |
| 中文 270 | English 270 | 🚀 270 |

```rust
fn section_270() -> usize {
    let n = 270;
    n * 2
}
```

## Section 271 — sample heading 271

This is paragraph one of section 271. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 271 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-271).

- 列表项 A of section 271
- 列表项 B of section 271
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 271

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 271a   | 271b   | 271c   |
| 中文 271 | English 271 | 🚀 271 |

```rust
fn section_271() -> usize {
    let n = 271;
    n * 2
}
```

## Section 272 — sample heading 272

This is paragraph one of section 272. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 272 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-272).

- 列表项 A of section 272
- 列表项 B of section 272
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 272

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 272a   | 272b   | 272c   |
| 中文 272 | English 272 | 🚀 272 |

```rust
fn section_272() -> usize {
    let n = 272;
    n * 2
}
```

## Section 273 — sample heading 273

This is paragraph one of section 273. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 273 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-273).

- 列表项 A of section 273
- 列表项 B of section 273
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 273

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 273a   | 273b   | 273c   |
| 中文 273 | English 273 | 🚀 273 |

```rust
fn section_273() -> usize {
    let n = 273;
    n * 2
}
```

## Section 274 — sample heading 274

This is paragraph one of section 274. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 274 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-274).

- 列表项 A of section 274
- 列表项 B of section 274
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 274

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 274a   | 274b   | 274c   |
| 中文 274 | English 274 | 🚀 274 |

```rust
fn section_274() -> usize {
    let n = 274;
    n * 2
}
```

## Section 275 — sample heading 275

This is paragraph one of section 275. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 275 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-275).

- 列表项 A of section 275
- 列表项 B of section 275
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 275

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 275a   | 275b   | 275c   |
| 中文 275 | English 275 | 🚀 275 |

```rust
fn section_275() -> usize {
    let n = 275;
    n * 2
}
```

## Section 276 — sample heading 276

This is paragraph one of section 276. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 276 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-276).

- 列表项 A of section 276
- 列表项 B of section 276
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 276

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 276a   | 276b   | 276c   |
| 中文 276 | English 276 | 🚀 276 |

```rust
fn section_276() -> usize {
    let n = 276;
    n * 2
}
```

## Section 277 — sample heading 277

This is paragraph one of section 277. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 277 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-277).

- 列表项 A of section 277
- 列表项 B of section 277
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 277

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 277a   | 277b   | 277c   |
| 中文 277 | English 277 | 🚀 277 |

```rust
fn section_277() -> usize {
    let n = 277;
    n * 2
}
```

## Section 278 — sample heading 278

This is paragraph one of section 278. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 278 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-278).

- 列表项 A of section 278
- 列表项 B of section 278
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 278

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 278a   | 278b   | 278c   |
| 中文 278 | English 278 | 🚀 278 |

```rust
fn section_278() -> usize {
    let n = 278;
    n * 2
}
```

## Section 279 — sample heading 279

This is paragraph one of section 279. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 279 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-279).

- 列表项 A of section 279
- 列表项 B of section 279
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 279

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 279a   | 279b   | 279c   |
| 中文 279 | English 279 | 🚀 279 |

```rust
fn section_279() -> usize {
    let n = 279;
    n * 2
}
```

## Section 280 — sample heading 280

This is paragraph one of section 280. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 280 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-280).

- 列表项 A of section 280
- 列表项 B of section 280
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 280

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 280a   | 280b   | 280c   |
| 中文 280 | English 280 | 🚀 280 |

```rust
fn section_280() -> usize {
    let n = 280;
    n * 2
}
```

## Section 281 — sample heading 281

This is paragraph one of section 281. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 281 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-281).

- 列表项 A of section 281
- 列表项 B of section 281
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 281

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 281a   | 281b   | 281c   |
| 中文 281 | English 281 | 🚀 281 |

```rust
fn section_281() -> usize {
    let n = 281;
    n * 2
}
```

## Section 282 — sample heading 282

This is paragraph one of section 282. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 282 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-282).

- 列表项 A of section 282
- 列表项 B of section 282
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 282

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 282a   | 282b   | 282c   |
| 中文 282 | English 282 | 🚀 282 |

```rust
fn section_282() -> usize {
    let n = 282;
    n * 2
}
```

## Section 283 — sample heading 283

This is paragraph one of section 283. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 283 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-283).

- 列表项 A of section 283
- 列表项 B of section 283
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 283

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 283a   | 283b   | 283c   |
| 中文 283 | English 283 | 🚀 283 |

```rust
fn section_283() -> usize {
    let n = 283;
    n * 2
}
```

## Section 284 — sample heading 284

This is paragraph one of section 284. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 284 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-284).

- 列表项 A of section 284
- 列表项 B of section 284
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 284

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 284a   | 284b   | 284c   |
| 中文 284 | English 284 | 🚀 284 |

```rust
fn section_284() -> usize {
    let n = 284;
    n * 2
}
```

## Section 285 — sample heading 285

This is paragraph one of section 285. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 285 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-285).

- 列表项 A of section 285
- 列表项 B of section 285
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 285

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 285a   | 285b   | 285c   |
| 中文 285 | English 285 | 🚀 285 |

```rust
fn section_285() -> usize {
    let n = 285;
    n * 2
}
```

## Section 286 — sample heading 286

This is paragraph one of section 286. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 286 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-286).

- 列表项 A of section 286
- 列表项 B of section 286
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 286

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 286a   | 286b   | 286c   |
| 中文 286 | English 286 | 🚀 286 |

```rust
fn section_286() -> usize {
    let n = 286;
    n * 2
}
```

## Section 287 — sample heading 287

This is paragraph one of section 287. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 287 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-287).

- 列表项 A of section 287
- 列表项 B of section 287
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 287

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 287a   | 287b   | 287c   |
| 中文 287 | English 287 | 🚀 287 |

```rust
fn section_287() -> usize {
    let n = 287;
    n * 2
}
```

## Section 288 — sample heading 288

This is paragraph one of section 288. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 288 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-288).

- 列表项 A of section 288
- 列表项 B of section 288
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 288

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 288a   | 288b   | 288c   |
| 中文 288 | English 288 | 🚀 288 |

```rust
fn section_288() -> usize {
    let n = 288;
    n * 2
}
```

## Section 289 — sample heading 289

This is paragraph one of section 289. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 289 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-289).

- 列表项 A of section 289
- 列表项 B of section 289
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 289

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 289a   | 289b   | 289c   |
| 中文 289 | English 289 | 🚀 289 |

```rust
fn section_289() -> usize {
    let n = 289;
    n * 2
}
```

## Section 290 — sample heading 290

This is paragraph one of section 290. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 290 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-290).

- 列表项 A of section 290
- 列表项 B of section 290
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 290

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 290a   | 290b   | 290c   |
| 中文 290 | English 290 | 🚀 290 |

```rust
fn section_290() -> usize {
    let n = 290;
    n * 2
}
```

## Section 291 — sample heading 291

This is paragraph one of section 291. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 291 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-291).

- 列表项 A of section 291
- 列表项 B of section 291
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 291

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 291a   | 291b   | 291c   |
| 中文 291 | English 291 | 🚀 291 |

```rust
fn section_291() -> usize {
    let n = 291;
    n * 2
}
```

## Section 292 — sample heading 292

This is paragraph one of section 292. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 292 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-292).

- 列表项 A of section 292
- 列表项 B of section 292
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 292

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 292a   | 292b   | 292c   |
| 中文 292 | English 292 | 🚀 292 |

```rust
fn section_292() -> usize {
    let n = 292;
    n * 2
}
```

## Section 293 — sample heading 293

This is paragraph one of section 293. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 293 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-293).

- 列表项 A of section 293
- 列表项 B of section 293
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 293

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 293a   | 293b   | 293c   |
| 中文 293 | English 293 | 🚀 293 |

```rust
fn section_293() -> usize {
    let n = 293;
    n * 2
}
```

## Section 294 — sample heading 294

This is paragraph one of section 294. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 294 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-294).

- 列表项 A of section 294
- 列表项 B of section 294
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 294

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 294a   | 294b   | 294c   |
| 中文 294 | English 294 | 🚀 294 |

```rust
fn section_294() -> usize {
    let n = 294;
    n * 2
}
```

## Section 295 — sample heading 295

This is paragraph one of section 295. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 295 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-295).

- 列表项 A of section 295
- 列表项 B of section 295
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 295

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 295a   | 295b   | 295c   |
| 中文 295 | English 295 | 🚀 295 |

```rust
fn section_295() -> usize {
    let n = 295;
    n * 2
}
```

## Section 296 — sample heading 296

This is paragraph one of section 296. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 296 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-296).

- 列表项 A of section 296
- 列表项 B of section 296
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 296

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 296a   | 296b   | 296c   |
| 中文 296 | English 296 | 🚀 296 |

```rust
fn section_296() -> usize {
    let n = 296;
    n * 2
}
```

## Section 297 — sample heading 297

This is paragraph one of section 297. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 297 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-297).

- 列表项 A of section 297
- 列表项 B of section 297
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 297

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 297a   | 297b   | 297c   |
| 中文 297 | English 297 | 🚀 297 |

```rust
fn section_297() -> usize {
    let n = 297;
    n * 2
}
```

## Section 298 — sample heading 298

This is paragraph one of section 298. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 298 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-298).

- 列表项 A of section 298
- 列表项 B of section 298
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 298

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 298a   | 298b   | 298c   |
| 中文 298 | English 298 | 🚀 298 |

```rust
fn section_298() -> usize {
    let n = 298;
    n * 2
}
```

## Section 299 — sample heading 299

This is paragraph one of section 299. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 299 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-299).

- 列表项 A of section 299
- 列表项 B of section 299
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 299

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 299a   | 299b   | 299c   |
| 中文 299 | English 299 | 🚀 299 |

```rust
fn section_299() -> usize {
    let n = 299;
    n * 2
}
```

## Section 300 — sample heading 300

This is paragraph one of section 300. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 300 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-300).

- 列表项 A of section 300
- 列表项 B of section 300
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 300

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 300a   | 300b   | 300c   |
| 中文 300 | English 300 | 🚀 300 |

```rust
fn section_300() -> usize {
    let n = 300;
    n * 2
}
```

### Section 300.1 — extra depth marker

Every 50th section gets an H3 sub-section to vary the heading depth
across the document.

## Section 301 — sample heading 301

This is paragraph one of section 301. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 301 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-301).

- 列表项 A of section 301
- 列表项 B of section 301
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 301

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 301a   | 301b   | 301c   |
| 中文 301 | English 301 | 🚀 301 |

```rust
fn section_301() -> usize {
    let n = 301;
    n * 2
}
```

## Section 302 — sample heading 302

This is paragraph one of section 302. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 302 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-302).

- 列表项 A of section 302
- 列表项 B of section 302
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 302

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 302a   | 302b   | 302c   |
| 中文 302 | English 302 | 🚀 302 |

```rust
fn section_302() -> usize {
    let n = 302;
    n * 2
}
```

## Section 303 — sample heading 303

This is paragraph one of section 303. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 303 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-303).

- 列表项 A of section 303
- 列表项 B of section 303
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 303

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 303a   | 303b   | 303c   |
| 中文 303 | English 303 | 🚀 303 |

```rust
fn section_303() -> usize {
    let n = 303;
    n * 2
}
```

## Section 304 — sample heading 304

This is paragraph one of section 304. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 304 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-304).

- 列表项 A of section 304
- 列表项 B of section 304
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 304

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 304a   | 304b   | 304c   |
| 中文 304 | English 304 | 🚀 304 |

```rust
fn section_304() -> usize {
    let n = 304;
    n * 2
}
```

## Section 305 — sample heading 305

This is paragraph one of section 305. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 305 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-305).

- 列表项 A of section 305
- 列表项 B of section 305
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 305

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 305a   | 305b   | 305c   |
| 中文 305 | English 305 | 🚀 305 |

```rust
fn section_305() -> usize {
    let n = 305;
    n * 2
}
```

## Section 306 — sample heading 306

This is paragraph one of section 306. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 306 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-306).

- 列表项 A of section 306
- 列表项 B of section 306
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 306

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 306a   | 306b   | 306c   |
| 中文 306 | English 306 | 🚀 306 |

```rust
fn section_306() -> usize {
    let n = 306;
    n * 2
}
```

## Section 307 — sample heading 307

This is paragraph one of section 307. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 307 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-307).

- 列表项 A of section 307
- 列表项 B of section 307
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 307

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 307a   | 307b   | 307c   |
| 中文 307 | English 307 | 🚀 307 |

```rust
fn section_307() -> usize {
    let n = 307;
    n * 2
}
```

## Section 308 — sample heading 308

This is paragraph one of section 308. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 308 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-308).

- 列表项 A of section 308
- 列表项 B of section 308
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 308

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 308a   | 308b   | 308c   |
| 中文 308 | English 308 | 🚀 308 |

```rust
fn section_308() -> usize {
    let n = 308;
    n * 2
}
```

## Section 309 — sample heading 309

This is paragraph one of section 309. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 309 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-309).

- 列表项 A of section 309
- 列表项 B of section 309
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 309

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 309a   | 309b   | 309c   |
| 中文 309 | English 309 | 🚀 309 |

```rust
fn section_309() -> usize {
    let n = 309;
    n * 2
}
```

## Section 310 — sample heading 310

This is paragraph one of section 310. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 310 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-310).

- 列表项 A of section 310
- 列表项 B of section 310
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 310

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 310a   | 310b   | 310c   |
| 中文 310 | English 310 | 🚀 310 |

```rust
fn section_310() -> usize {
    let n = 310;
    n * 2
}
```

## Section 311 — sample heading 311

This is paragraph one of section 311. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 311 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-311).

- 列表项 A of section 311
- 列表项 B of section 311
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 311

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 311a   | 311b   | 311c   |
| 中文 311 | English 311 | 🚀 311 |

```rust
fn section_311() -> usize {
    let n = 311;
    n * 2
}
```

## Section 312 — sample heading 312

This is paragraph one of section 312. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 312 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-312).

- 列表项 A of section 312
- 列表项 B of section 312
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 312

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 312a   | 312b   | 312c   |
| 中文 312 | English 312 | 🚀 312 |

```rust
fn section_312() -> usize {
    let n = 312;
    n * 2
}
```

## Section 313 — sample heading 313

This is paragraph one of section 313. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 313 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-313).

- 列表项 A of section 313
- 列表项 B of section 313
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 313

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 313a   | 313b   | 313c   |
| 中文 313 | English 313 | 🚀 313 |

```rust
fn section_313() -> usize {
    let n = 313;
    n * 2
}
```

## Section 314 — sample heading 314

This is paragraph one of section 314. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 314 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-314).

- 列表项 A of section 314
- 列表项 B of section 314
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 314

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 314a   | 314b   | 314c   |
| 中文 314 | English 314 | 🚀 314 |

```rust
fn section_314() -> usize {
    let n = 314;
    n * 2
}
```

## Section 315 — sample heading 315

This is paragraph one of section 315. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 315 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-315).

- 列表项 A of section 315
- 列表项 B of section 315
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 315

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 315a   | 315b   | 315c   |
| 中文 315 | English 315 | 🚀 315 |

```rust
fn section_315() -> usize {
    let n = 315;
    n * 2
}
```

## Section 316 — sample heading 316

This is paragraph one of section 316. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 316 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-316).

- 列表项 A of section 316
- 列表项 B of section 316
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 316

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 316a   | 316b   | 316c   |
| 中文 316 | English 316 | 🚀 316 |

```rust
fn section_316() -> usize {
    let n = 316;
    n * 2
}
```

## Section 317 — sample heading 317

This is paragraph one of section 317. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 317 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-317).

- 列表项 A of section 317
- 列表项 B of section 317
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 317

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 317a   | 317b   | 317c   |
| 中文 317 | English 317 | 🚀 317 |

```rust
fn section_317() -> usize {
    let n = 317;
    n * 2
}
```

## Section 318 — sample heading 318

This is paragraph one of section 318. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 318 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-318).

- 列表项 A of section 318
- 列表项 B of section 318
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 318

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 318a   | 318b   | 318c   |
| 中文 318 | English 318 | 🚀 318 |

```rust
fn section_318() -> usize {
    let n = 318;
    n * 2
}
```

## Section 319 — sample heading 319

This is paragraph one of section 319. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 319 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-319).

- 列表项 A of section 319
- 列表项 B of section 319
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 319

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 319a   | 319b   | 319c   |
| 中文 319 | English 319 | 🚀 319 |

```rust
fn section_319() -> usize {
    let n = 319;
    n * 2
}
```

## Section 320 — sample heading 320

This is paragraph one of section 320. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 320 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-320).

- 列表项 A of section 320
- 列表项 B of section 320
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 320

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 320a   | 320b   | 320c   |
| 中文 320 | English 320 | 🚀 320 |

```rust
fn section_320() -> usize {
    let n = 320;
    n * 2
}
```

## Section 321 — sample heading 321

This is paragraph one of section 321. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 321 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-321).

- 列表项 A of section 321
- 列表项 B of section 321
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 321

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 321a   | 321b   | 321c   |
| 中文 321 | English 321 | 🚀 321 |

```rust
fn section_321() -> usize {
    let n = 321;
    n * 2
}
```

## Section 322 — sample heading 322

This is paragraph one of section 322. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 322 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-322).

- 列表项 A of section 322
- 列表项 B of section 322
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 322

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 322a   | 322b   | 322c   |
| 中文 322 | English 322 | 🚀 322 |

```rust
fn section_322() -> usize {
    let n = 322;
    n * 2
}
```

## Section 323 — sample heading 323

This is paragraph one of section 323. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 323 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-323).

- 列表项 A of section 323
- 列表项 B of section 323
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 323

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 323a   | 323b   | 323c   |
| 中文 323 | English 323 | 🚀 323 |

```rust
fn section_323() -> usize {
    let n = 323;
    n * 2
}
```

## Section 324 — sample heading 324

This is paragraph one of section 324. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 324 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-324).

- 列表项 A of section 324
- 列表项 B of section 324
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 324

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 324a   | 324b   | 324c   |
| 中文 324 | English 324 | 🚀 324 |

```rust
fn section_324() -> usize {
    let n = 324;
    n * 2
}
```

## Section 325 — sample heading 325

This is paragraph one of section 325. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 325 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-325).

- 列表项 A of section 325
- 列表项 B of section 325
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 325

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 325a   | 325b   | 325c   |
| 中文 325 | English 325 | 🚀 325 |

```rust
fn section_325() -> usize {
    let n = 325;
    n * 2
}
```

## Section 326 — sample heading 326

This is paragraph one of section 326. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 326 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-326).

- 列表项 A of section 326
- 列表项 B of section 326
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 326

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 326a   | 326b   | 326c   |
| 中文 326 | English 326 | 🚀 326 |

```rust
fn section_326() -> usize {
    let n = 326;
    n * 2
}
```

## Section 327 — sample heading 327

This is paragraph one of section 327. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 327 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-327).

- 列表项 A of section 327
- 列表项 B of section 327
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 327

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 327a   | 327b   | 327c   |
| 中文 327 | English 327 | 🚀 327 |

```rust
fn section_327() -> usize {
    let n = 327;
    n * 2
}
```

## Section 328 — sample heading 328

This is paragraph one of section 328. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 328 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-328).

- 列表项 A of section 328
- 列表项 B of section 328
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 328

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 328a   | 328b   | 328c   |
| 中文 328 | English 328 | 🚀 328 |

```rust
fn section_328() -> usize {
    let n = 328;
    n * 2
}
```

## Section 329 — sample heading 329

This is paragraph one of section 329. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 329 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-329).

- 列表项 A of section 329
- 列表项 B of section 329
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 329

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 329a   | 329b   | 329c   |
| 中文 329 | English 329 | 🚀 329 |

```rust
fn section_329() -> usize {
    let n = 329;
    n * 2
}
```

## Section 330 — sample heading 330

This is paragraph one of section 330. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 330 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-330).

- 列表项 A of section 330
- 列表项 B of section 330
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 330

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 330a   | 330b   | 330c   |
| 中文 330 | English 330 | 🚀 330 |

```rust
fn section_330() -> usize {
    let n = 330;
    n * 2
}
```

## Section 331 — sample heading 331

This is paragraph one of section 331. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 331 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-331).

- 列表项 A of section 331
- 列表项 B of section 331
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 331

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 331a   | 331b   | 331c   |
| 中文 331 | English 331 | 🚀 331 |

```rust
fn section_331() -> usize {
    let n = 331;
    n * 2
}
```

## Section 332 — sample heading 332

This is paragraph one of section 332. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 332 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-332).

- 列表项 A of section 332
- 列表项 B of section 332
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 332

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 332a   | 332b   | 332c   |
| 中文 332 | English 332 | 🚀 332 |

```rust
fn section_332() -> usize {
    let n = 332;
    n * 2
}
```

## Section 333 — sample heading 333

This is paragraph one of section 333. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 333 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-333).

- 列表项 A of section 333
- 列表项 B of section 333
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 333

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 333a   | 333b   | 333c   |
| 中文 333 | English 333 | 🚀 333 |

```rust
fn section_333() -> usize {
    let n = 333;
    n * 2
}
```

## Section 334 — sample heading 334

This is paragraph one of section 334. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 334 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-334).

- 列表项 A of section 334
- 列表项 B of section 334
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 334

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 334a   | 334b   | 334c   |
| 中文 334 | English 334 | 🚀 334 |

```rust
fn section_334() -> usize {
    let n = 334;
    n * 2
}
```

## Section 335 — sample heading 335

This is paragraph one of section 335. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 335 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-335).

- 列表项 A of section 335
- 列表项 B of section 335
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 335

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 335a   | 335b   | 335c   |
| 中文 335 | English 335 | 🚀 335 |

```rust
fn section_335() -> usize {
    let n = 335;
    n * 2
}
```

## Section 336 — sample heading 336

This is paragraph one of section 336. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 336 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-336).

- 列表项 A of section 336
- 列表项 B of section 336
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 336

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 336a   | 336b   | 336c   |
| 中文 336 | English 336 | 🚀 336 |

```rust
fn section_336() -> usize {
    let n = 336;
    n * 2
}
```

## Section 337 — sample heading 337

This is paragraph one of section 337. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 337 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-337).

- 列表项 A of section 337
- 列表项 B of section 337
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 337

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 337a   | 337b   | 337c   |
| 中文 337 | English 337 | 🚀 337 |

```rust
fn section_337() -> usize {
    let n = 337;
    n * 2
}
```

## Section 338 — sample heading 338

This is paragraph one of section 338. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 338 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-338).

- 列表项 A of section 338
- 列表项 B of section 338
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 338

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 338a   | 338b   | 338c   |
| 中文 338 | English 338 | 🚀 338 |

```rust
fn section_338() -> usize {
    let n = 338;
    n * 2
}
```

## Section 339 — sample heading 339

This is paragraph one of section 339. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 339 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-339).

- 列表项 A of section 339
- 列表项 B of section 339
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 339

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 339a   | 339b   | 339c   |
| 中文 339 | English 339 | 🚀 339 |

```rust
fn section_339() -> usize {
    let n = 339;
    n * 2
}
```

## Section 340 — sample heading 340

This is paragraph one of section 340. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 340 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-340).

- 列表项 A of section 340
- 列表项 B of section 340
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 340

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 340a   | 340b   | 340c   |
| 中文 340 | English 340 | 🚀 340 |

```rust
fn section_340() -> usize {
    let n = 340;
    n * 2
}
```

## Section 341 — sample heading 341

This is paragraph one of section 341. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 341 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-341).

- 列表项 A of section 341
- 列表项 B of section 341
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 341

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 341a   | 341b   | 341c   |
| 中文 341 | English 341 | 🚀 341 |

```rust
fn section_341() -> usize {
    let n = 341;
    n * 2
}
```

## Section 342 — sample heading 342

This is paragraph one of section 342. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 342 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-342).

- 列表项 A of section 342
- 列表项 B of section 342
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 342

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 342a   | 342b   | 342c   |
| 中文 342 | English 342 | 🚀 342 |

```rust
fn section_342() -> usize {
    let n = 342;
    n * 2
}
```

## Section 343 — sample heading 343

This is paragraph one of section 343. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 343 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-343).

- 列表项 A of section 343
- 列表项 B of section 343
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 343

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 343a   | 343b   | 343c   |
| 中文 343 | English 343 | 🚀 343 |

```rust
fn section_343() -> usize {
    let n = 343;
    n * 2
}
```

## Section 344 — sample heading 344

This is paragraph one of section 344. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 344 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-344).

- 列表项 A of section 344
- 列表项 B of section 344
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 344

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 344a   | 344b   | 344c   |
| 中文 344 | English 344 | 🚀 344 |

```rust
fn section_344() -> usize {
    let n = 344;
    n * 2
}
```

## Section 345 — sample heading 345

This is paragraph one of section 345. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 345 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-345).

- 列表项 A of section 345
- 列表项 B of section 345
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 345

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 345a   | 345b   | 345c   |
| 中文 345 | English 345 | 🚀 345 |

```rust
fn section_345() -> usize {
    let n = 345;
    n * 2
}
```

## Section 346 — sample heading 346

This is paragraph one of section 346. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 346 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-346).

- 列表项 A of section 346
- 列表项 B of section 346
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 346

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 346a   | 346b   | 346c   |
| 中文 346 | English 346 | 🚀 346 |

```rust
fn section_346() -> usize {
    let n = 346;
    n * 2
}
```

## Section 347 — sample heading 347

This is paragraph one of section 347. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 347 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-347).

- 列表项 A of section 347
- 列表项 B of section 347
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 347

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 347a   | 347b   | 347c   |
| 中文 347 | English 347 | 🚀 347 |

```rust
fn section_347() -> usize {
    let n = 347;
    n * 2
}
```

## Section 348 — sample heading 348

This is paragraph one of section 348. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 348 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-348).

- 列表项 A of section 348
- 列表项 B of section 348
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 348

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 348a   | 348b   | 348c   |
| 中文 348 | English 348 | 🚀 348 |

```rust
fn section_348() -> usize {
    let n = 348;
    n * 2
}
```

## Section 349 — sample heading 349

This is paragraph one of section 349. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 349 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-349).

- 列表项 A of section 349
- 列表项 B of section 349
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 349

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 349a   | 349b   | 349c   |
| 中文 349 | English 349 | 🚀 349 |

```rust
fn section_349() -> usize {
    let n = 349;
    n * 2
}
```

## Section 350 — sample heading 350

This is paragraph one of section 350. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 350 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-350).

- 列表项 A of section 350
- 列表项 B of section 350
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 350

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 350a   | 350b   | 350c   |
| 中文 350 | English 350 | 🚀 350 |

```rust
fn section_350() -> usize {
    let n = 350;
    n * 2
}
```

### Section 350.1 — extra depth marker

Every 50th section gets an H3 sub-section to vary the heading depth
across the document.

## Section 351 — sample heading 351

This is paragraph one of section 351. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 351 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-351).

- 列表项 A of section 351
- 列表项 B of section 351
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 351

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 351a   | 351b   | 351c   |
| 中文 351 | English 351 | 🚀 351 |

```rust
fn section_351() -> usize {
    let n = 351;
    n * 2
}
```

## Section 352 — sample heading 352

This is paragraph one of section 352. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 352 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-352).

- 列表项 A of section 352
- 列表项 B of section 352
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 352

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 352a   | 352b   | 352c   |
| 中文 352 | English 352 | 🚀 352 |

```rust
fn section_352() -> usize {
    let n = 352;
    n * 2
}
```

## Section 353 — sample heading 353

This is paragraph one of section 353. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 353 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-353).

- 列表项 A of section 353
- 列表项 B of section 353
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 353

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 353a   | 353b   | 353c   |
| 中文 353 | English 353 | 🚀 353 |

```rust
fn section_353() -> usize {
    let n = 353;
    n * 2
}
```

## Section 354 — sample heading 354

This is paragraph one of section 354. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 354 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-354).

- 列表项 A of section 354
- 列表项 B of section 354
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 354

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 354a   | 354b   | 354c   |
| 中文 354 | English 354 | 🚀 354 |

```rust
fn section_354() -> usize {
    let n = 354;
    n * 2
}
```

## Section 355 — sample heading 355

This is paragraph one of section 355. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 355 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-355).

- 列表项 A of section 355
- 列表项 B of section 355
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 355

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 355a   | 355b   | 355c   |
| 中文 355 | English 355 | 🚀 355 |

```rust
fn section_355() -> usize {
    let n = 355;
    n * 2
}
```

## Section 356 — sample heading 356

This is paragraph one of section 356. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 356 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-356).

- 列表项 A of section 356
- 列表项 B of section 356
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 356

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 356a   | 356b   | 356c   |
| 中文 356 | English 356 | 🚀 356 |

```rust
fn section_356() -> usize {
    let n = 356;
    n * 2
}
```

## Section 357 — sample heading 357

This is paragraph one of section 357. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 357 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-357).

- 列表项 A of section 357
- 列表项 B of section 357
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 357

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 357a   | 357b   | 357c   |
| 中文 357 | English 357 | 🚀 357 |

```rust
fn section_357() -> usize {
    let n = 357;
    n * 2
}
```

## Section 358 — sample heading 358

This is paragraph one of section 358. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 358 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-358).

- 列表项 A of section 358
- 列表项 B of section 358
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 358

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 358a   | 358b   | 358c   |
| 中文 358 | English 358 | 🚀 358 |

```rust
fn section_358() -> usize {
    let n = 358;
    n * 2
}
```

## Section 359 — sample heading 359

This is paragraph one of section 359. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 359 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-359).

- 列表项 A of section 359
- 列表项 B of section 359
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 359

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 359a   | 359b   | 359c   |
| 中文 359 | English 359 | 🚀 359 |

```rust
fn section_359() -> usize {
    let n = 359;
    n * 2
}
```

## Section 360 — sample heading 360

This is paragraph one of section 360. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 360 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-360).

- 列表项 A of section 360
- 列表项 B of section 360
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 360

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 360a   | 360b   | 360c   |
| 中文 360 | English 360 | 🚀 360 |

```rust
fn section_360() -> usize {
    let n = 360;
    n * 2
}
```

## Section 361 — sample heading 361

This is paragraph one of section 361. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 361 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-361).

- 列表项 A of section 361
- 列表项 B of section 361
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 361

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 361a   | 361b   | 361c   |
| 中文 361 | English 361 | 🚀 361 |

```rust
fn section_361() -> usize {
    let n = 361;
    n * 2
}
```

## Section 362 — sample heading 362

This is paragraph one of section 362. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 362 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-362).

- 列表项 A of section 362
- 列表项 B of section 362
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 362

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 362a   | 362b   | 362c   |
| 中文 362 | English 362 | 🚀 362 |

```rust
fn section_362() -> usize {
    let n = 362;
    n * 2
}
```

## Section 363 — sample heading 363

This is paragraph one of section 363. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 363 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-363).

- 列表项 A of section 363
- 列表项 B of section 363
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 363

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 363a   | 363b   | 363c   |
| 中文 363 | English 363 | 🚀 363 |

```rust
fn section_363() -> usize {
    let n = 363;
    n * 2
}
```

## Section 364 — sample heading 364

This is paragraph one of section 364. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 364 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-364).

- 列表项 A of section 364
- 列表项 B of section 364
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 364

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 364a   | 364b   | 364c   |
| 中文 364 | English 364 | 🚀 364 |

```rust
fn section_364() -> usize {
    let n = 364;
    n * 2
}
```

## Section 365 — sample heading 365

This is paragraph one of section 365. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 365 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-365).

- 列表项 A of section 365
- 列表项 B of section 365
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 365

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 365a   | 365b   | 365c   |
| 中文 365 | English 365 | 🚀 365 |

```rust
fn section_365() -> usize {
    let n = 365;
    n * 2
}
```

## Section 366 — sample heading 366

This is paragraph one of section 366. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 366 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-366).

- 列表项 A of section 366
- 列表项 B of section 366
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 366

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 366a   | 366b   | 366c   |
| 中文 366 | English 366 | 🚀 366 |

```rust
fn section_366() -> usize {
    let n = 366;
    n * 2
}
```

## Section 367 — sample heading 367

This is paragraph one of section 367. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 367 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-367).

- 列表项 A of section 367
- 列表项 B of section 367
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 367

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 367a   | 367b   | 367c   |
| 中文 367 | English 367 | 🚀 367 |

```rust
fn section_367() -> usize {
    let n = 367;
    n * 2
}
```

## Section 368 — sample heading 368

This is paragraph one of section 368. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 368 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-368).

- 列表项 A of section 368
- 列表项 B of section 368
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 368

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 368a   | 368b   | 368c   |
| 中文 368 | English 368 | 🚀 368 |

```rust
fn section_368() -> usize {
    let n = 368;
    n * 2
}
```

## Section 369 — sample heading 369

This is paragraph one of section 369. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 369 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-369).

- 列表项 A of section 369
- 列表项 B of section 369
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 369

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 369a   | 369b   | 369c   |
| 中文 369 | English 369 | 🚀 369 |

```rust
fn section_369() -> usize {
    let n = 369;
    n * 2
}
```

## Section 370 — sample heading 370

This is paragraph one of section 370. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 370 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-370).

- 列表项 A of section 370
- 列表项 B of section 370
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 370

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 370a   | 370b   | 370c   |
| 中文 370 | English 370 | 🚀 370 |

```rust
fn section_370() -> usize {
    let n = 370;
    n * 2
}
```

## Section 371 — sample heading 371

This is paragraph one of section 371. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 371 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-371).

- 列表项 A of section 371
- 列表项 B of section 371
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 371

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 371a   | 371b   | 371c   |
| 中文 371 | English 371 | 🚀 371 |

```rust
fn section_371() -> usize {
    let n = 371;
    n * 2
}
```

## Section 372 — sample heading 372

This is paragraph one of section 372. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 372 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-372).

- 列表项 A of section 372
- 列表项 B of section 372
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 372

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 372a   | 372b   | 372c   |
| 中文 372 | English 372 | 🚀 372 |

```rust
fn section_372() -> usize {
    let n = 372;
    n * 2
}
```

## Section 373 — sample heading 373

This is paragraph one of section 373. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 373 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-373).

- 列表项 A of section 373
- 列表项 B of section 373
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 373

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 373a   | 373b   | 373c   |
| 中文 373 | English 373 | 🚀 373 |

```rust
fn section_373() -> usize {
    let n = 373;
    n * 2
}
```

## Section 374 — sample heading 374

This is paragraph one of section 374. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 374 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-374).

- 列表项 A of section 374
- 列表项 B of section 374
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 374

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 374a   | 374b   | 374c   |
| 中文 374 | English 374 | 🚀 374 |

```rust
fn section_374() -> usize {
    let n = 374;
    n * 2
}
```

## Section 375 — sample heading 375

This is paragraph one of section 375. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 375 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-375).

- 列表项 A of section 375
- 列表项 B of section 375
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 375

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 375a   | 375b   | 375c   |
| 中文 375 | English 375 | 🚀 375 |

```rust
fn section_375() -> usize {
    let n = 375;
    n * 2
}
```

## Section 376 — sample heading 376

This is paragraph one of section 376. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 376 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-376).

- 列表项 A of section 376
- 列表项 B of section 376
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 376

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 376a   | 376b   | 376c   |
| 中文 376 | English 376 | 🚀 376 |

```rust
fn section_376() -> usize {
    let n = 376;
    n * 2
}
```

## Section 377 — sample heading 377

This is paragraph one of section 377. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 377 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-377).

- 列表项 A of section 377
- 列表项 B of section 377
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 377

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 377a   | 377b   | 377c   |
| 中文 377 | English 377 | 🚀 377 |

```rust
fn section_377() -> usize {
    let n = 377;
    n * 2
}
```

## Section 378 — sample heading 378

This is paragraph one of section 378. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 378 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-378).

- 列表项 A of section 378
- 列表项 B of section 378
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 378

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 378a   | 378b   | 378c   |
| 中文 378 | English 378 | 🚀 378 |

```rust
fn section_378() -> usize {
    let n = 378;
    n * 2
}
```

## Section 379 — sample heading 379

This is paragraph one of section 379. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 379 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-379).

- 列表项 A of section 379
- 列表项 B of section 379
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 379

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 379a   | 379b   | 379c   |
| 中文 379 | English 379 | 🚀 379 |

```rust
fn section_379() -> usize {
    let n = 379;
    n * 2
}
```

## Section 380 — sample heading 380

This is paragraph one of section 380. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 380 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-380).

- 列表项 A of section 380
- 列表项 B of section 380
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 380

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 380a   | 380b   | 380c   |
| 中文 380 | English 380 | 🚀 380 |

```rust
fn section_380() -> usize {
    let n = 380;
    n * 2
}
```

## Section 381 — sample heading 381

This is paragraph one of section 381. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 381 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-381).

- 列表项 A of section 381
- 列表项 B of section 381
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 381

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 381a   | 381b   | 381c   |
| 中文 381 | English 381 | 🚀 381 |

```rust
fn section_381() -> usize {
    let n = 381;
    n * 2
}
```

## Section 382 — sample heading 382

This is paragraph one of section 382. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 382 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-382).

- 列表项 A of section 382
- 列表项 B of section 382
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 382

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 382a   | 382b   | 382c   |
| 中文 382 | English 382 | 🚀 382 |

```rust
fn section_382() -> usize {
    let n = 382;
    n * 2
}
```

## Section 383 — sample heading 383

This is paragraph one of section 383. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 383 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-383).

- 列表项 A of section 383
- 列表项 B of section 383
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 383

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 383a   | 383b   | 383c   |
| 中文 383 | English 383 | 🚀 383 |

```rust
fn section_383() -> usize {
    let n = 383;
    n * 2
}
```

## Section 384 — sample heading 384

This is paragraph one of section 384. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 384 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-384).

- 列表项 A of section 384
- 列表项 B of section 384
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 384

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 384a   | 384b   | 384c   |
| 中文 384 | English 384 | 🚀 384 |

```rust
fn section_384() -> usize {
    let n = 384;
    n * 2
}
```

## Section 385 — sample heading 385

This is paragraph one of section 385. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 385 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-385).

- 列表项 A of section 385
- 列表项 B of section 385
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 385

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 385a   | 385b   | 385c   |
| 中文 385 | English 385 | 🚀 385 |

```rust
fn section_385() -> usize {
    let n = 385;
    n * 2
}
```

## Section 386 — sample heading 386

This is paragraph one of section 386. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 386 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-386).

- 列表项 A of section 386
- 列表项 B of section 386
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 386

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 386a   | 386b   | 386c   |
| 中文 386 | English 386 | 🚀 386 |

```rust
fn section_386() -> usize {
    let n = 386;
    n * 2
}
```

## Section 387 — sample heading 387

This is paragraph one of section 387. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 387 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-387).

- 列表项 A of section 387
- 列表项 B of section 387
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 387

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 387a   | 387b   | 387c   |
| 中文 387 | English 387 | 🚀 387 |

```rust
fn section_387() -> usize {
    let n = 387;
    n * 2
}
```

## Section 388 — sample heading 388

This is paragraph one of section 388. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 388 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-388).

- 列表项 A of section 388
- 列表项 B of section 388
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 388

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 388a   | 388b   | 388c   |
| 中文 388 | English 388 | 🚀 388 |

```rust
fn section_388() -> usize {
    let n = 388;
    n * 2
}
```

## Section 389 — sample heading 389

This is paragraph one of section 389. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 389 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-389).

- 列表项 A of section 389
- 列表项 B of section 389
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 389

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 389a   | 389b   | 389c   |
| 中文 389 | English 389 | 🚀 389 |

```rust
fn section_389() -> usize {
    let n = 389;
    n * 2
}
```

## Section 390 — sample heading 390

This is paragraph one of section 390. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 390 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-390).

- 列表项 A of section 390
- 列表项 B of section 390
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 390

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 390a   | 390b   | 390c   |
| 中文 390 | English 390 | 🚀 390 |

```rust
fn section_390() -> usize {
    let n = 390;
    n * 2
}
```

## Section 391 — sample heading 391

This is paragraph one of section 391. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 391 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-391).

- 列表项 A of section 391
- 列表项 B of section 391
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 391

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 391a   | 391b   | 391c   |
| 中文 391 | English 391 | 🚀 391 |

```rust
fn section_391() -> usize {
    let n = 391;
    n * 2
}
```

## Section 392 — sample heading 392

This is paragraph one of section 392. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 392 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-392).

- 列表项 A of section 392
- 列表项 B of section 392
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 392

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 392a   | 392b   | 392c   |
| 中文 392 | English 392 | 🚀 392 |

```rust
fn section_392() -> usize {
    let n = 392;
    n * 2
}
```

## Section 393 — sample heading 393

This is paragraph one of section 393. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 393 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-393).

- 列表项 A of section 393
- 列表项 B of section 393
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 393

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 393a   | 393b   | 393c   |
| 中文 393 | English 393 | 🚀 393 |

```rust
fn section_393() -> usize {
    let n = 393;
    n * 2
}
```

## Section 394 — sample heading 394

This is paragraph one of section 394. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 394 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-394).

- 列表项 A of section 394
- 列表项 B of section 394
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 394

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 394a   | 394b   | 394c   |
| 中文 394 | English 394 | 🚀 394 |

```rust
fn section_394() -> usize {
    let n = 394;
    n * 2
}
```

## Section 395 — sample heading 395

This is paragraph one of section 395. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 395 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-395).

- 列表项 A of section 395
- 列表项 B of section 395
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 395

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 395a   | 395b   | 395c   |
| 中文 395 | English 395 | 🚀 395 |

```rust
fn section_395() -> usize {
    let n = 395;
    n * 2
}
```

## Section 396 — sample heading 396

This is paragraph one of section 396. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 396 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-396).

- 列表项 A of section 396
- 列表项 B of section 396
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 396

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 396a   | 396b   | 396c   |
| 中文 396 | English 396 | 🚀 396 |

```rust
fn section_396() -> usize {
    let n = 396;
    n * 2
}
```

## Section 397 — sample heading 397

This is paragraph one of section 397. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 397 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-397).

- 列表项 A of section 397
- 列表项 B of section 397
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 397

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 397a   | 397b   | 397c   |
| 中文 397 | English 397 | 🚀 397 |

```rust
fn section_397() -> usize {
    let n = 397;
    n * 2
}
```

## Section 398 — sample heading 398

This is paragraph one of section 398. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 398 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-398).

- 列表项 A of section 398
- 列表项 B of section 398
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 398

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 398a   | 398b   | 398c   |
| 中文 398 | English 398 | 🚀 398 |

```rust
fn section_398() -> usize {
    let n = 398;
    n * 2
}
```

## Section 399 — sample heading 399

This is paragraph one of section 399. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 399 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-399).

- 列表项 A of section 399
- 列表项 B of section 399
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 399

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 399a   | 399b   | 399c   |
| 中文 399 | English 399 | 🚀 399 |

```rust
fn section_399() -> usize {
    let n = 399;
    n * 2
}
```

## Section 400 — sample heading 400

This is paragraph one of section 400. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 400 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-400).

- 列表项 A of section 400
- 列表项 B of section 400
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 400

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 400a   | 400b   | 400c   |
| 中文 400 | English 400 | 🚀 400 |

```rust
fn section_400() -> usize {
    let n = 400;
    n * 2
}
```

### Section 400.1 — extra depth marker

Every 50th section gets an H3 sub-section to vary the heading depth
across the document.

## Section 401 — sample heading 401

This is paragraph one of section 401. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 401 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-401).

- 列表项 A of section 401
- 列表项 B of section 401
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 401

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 401a   | 401b   | 401c   |
| 中文 401 | English 401 | 🚀 401 |

```rust
fn section_401() -> usize {
    let n = 401;
    n * 2
}
```

## Section 402 — sample heading 402

This is paragraph one of section 402. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 402 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-402).

- 列表项 A of section 402
- 列表项 B of section 402
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 402

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 402a   | 402b   | 402c   |
| 中文 402 | English 402 | 🚀 402 |

```rust
fn section_402() -> usize {
    let n = 402;
    n * 2
}
```

## Section 403 — sample heading 403

This is paragraph one of section 403. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 403 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-403).

- 列表项 A of section 403
- 列表项 B of section 403
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 403

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 403a   | 403b   | 403c   |
| 中文 403 | English 403 | 🚀 403 |

```rust
fn section_403() -> usize {
    let n = 403;
    n * 2
}
```

## Section 404 — sample heading 404

This is paragraph one of section 404. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 404 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-404).

- 列表项 A of section 404
- 列表项 B of section 404
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 404

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 404a   | 404b   | 404c   |
| 中文 404 | English 404 | 🚀 404 |

```rust
fn section_404() -> usize {
    let n = 404;
    n * 2
}
```

## Section 405 — sample heading 405

This is paragraph one of section 405. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 405 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-405).

- 列表项 A of section 405
- 列表项 B of section 405
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 405

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 405a   | 405b   | 405c   |
| 中文 405 | English 405 | 🚀 405 |

```rust
fn section_405() -> usize {
    let n = 405;
    n * 2
}
```

## Section 406 — sample heading 406

This is paragraph one of section 406. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 406 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-406).

- 列表项 A of section 406
- 列表项 B of section 406
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 406

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 406a   | 406b   | 406c   |
| 中文 406 | English 406 | 🚀 406 |

```rust
fn section_406() -> usize {
    let n = 406;
    n * 2
}
```

## Section 407 — sample heading 407

This is paragraph one of section 407. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 407 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-407).

- 列表项 A of section 407
- 列表项 B of section 407
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 407

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 407a   | 407b   | 407c   |
| 中文 407 | English 407 | 🚀 407 |

```rust
fn section_407() -> usize {
    let n = 407;
    n * 2
}
```

## Section 408 — sample heading 408

This is paragraph one of section 408. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 408 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-408).

- 列表项 A of section 408
- 列表项 B of section 408
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 408

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 408a   | 408b   | 408c   |
| 中文 408 | English 408 | 🚀 408 |

```rust
fn section_408() -> usize {
    let n = 408;
    n * 2
}
```

## Section 409 — sample heading 409

This is paragraph one of section 409. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 409 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-409).

- 列表项 A of section 409
- 列表项 B of section 409
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 409

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 409a   | 409b   | 409c   |
| 中文 409 | English 409 | 🚀 409 |

```rust
fn section_409() -> usize {
    let n = 409;
    n * 2
}
```

## Section 410 — sample heading 410

This is paragraph one of section 410. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 410 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-410).

- 列表项 A of section 410
- 列表项 B of section 410
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 410

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 410a   | 410b   | 410c   |
| 中文 410 | English 410 | 🚀 410 |

```rust
fn section_410() -> usize {
    let n = 410;
    n * 2
}
```

## Section 411 — sample heading 411

This is paragraph one of section 411. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 411 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-411).

- 列表项 A of section 411
- 列表项 B of section 411
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 411

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 411a   | 411b   | 411c   |
| 中文 411 | English 411 | 🚀 411 |

```rust
fn section_411() -> usize {
    let n = 411;
    n * 2
}
```

## Section 412 — sample heading 412

This is paragraph one of section 412. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 412 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-412).

- 列表项 A of section 412
- 列表项 B of section 412
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 412

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 412a   | 412b   | 412c   |
| 中文 412 | English 412 | 🚀 412 |

```rust
fn section_412() -> usize {
    let n = 412;
    n * 2
}
```

## Section 413 — sample heading 413

This is paragraph one of section 413. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 413 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-413).

- 列表项 A of section 413
- 列表项 B of section 413
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 413

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 413a   | 413b   | 413c   |
| 中文 413 | English 413 | 🚀 413 |

```rust
fn section_413() -> usize {
    let n = 413;
    n * 2
}
```

## Section 414 — sample heading 414

This is paragraph one of section 414. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 414 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-414).

- 列表项 A of section 414
- 列表项 B of section 414
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 414

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 414a   | 414b   | 414c   |
| 中文 414 | English 414 | 🚀 414 |

```rust
fn section_414() -> usize {
    let n = 414;
    n * 2
}
```

## Section 415 — sample heading 415

This is paragraph one of section 415. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 415 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-415).

- 列表项 A of section 415
- 列表项 B of section 415
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 415

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 415a   | 415b   | 415c   |
| 中文 415 | English 415 | 🚀 415 |

```rust
fn section_415() -> usize {
    let n = 415;
    n * 2
}
```

## Section 416 — sample heading 416

This is paragraph one of section 416. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 416 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-416).

- 列表项 A of section 416
- 列表项 B of section 416
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 416

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 416a   | 416b   | 416c   |
| 中文 416 | English 416 | 🚀 416 |

```rust
fn section_416() -> usize {
    let n = 416;
    n * 2
}
```

## Section 417 — sample heading 417

This is paragraph one of section 417. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 417 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-417).

- 列表项 A of section 417
- 列表项 B of section 417
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 417

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 417a   | 417b   | 417c   |
| 中文 417 | English 417 | 🚀 417 |

```rust
fn section_417() -> usize {
    let n = 417;
    n * 2
}
```

## Section 418 — sample heading 418

This is paragraph one of section 418. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 418 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-418).

- 列表项 A of section 418
- 列表项 B of section 418
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 418

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 418a   | 418b   | 418c   |
| 中文 418 | English 418 | 🚀 418 |

```rust
fn section_418() -> usize {
    let n = 418;
    n * 2
}
```

## Section 419 — sample heading 419

This is paragraph one of section 419. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 419 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-419).

- 列表项 A of section 419
- 列表项 B of section 419
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 419

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 419a   | 419b   | 419c   |
| 中文 419 | English 419 | 🚀 419 |

```rust
fn section_419() -> usize {
    let n = 419;
    n * 2
}
```

## Section 420 — sample heading 420

This is paragraph one of section 420. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 420 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-420).

- 列表项 A of section 420
- 列表项 B of section 420
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 420

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 420a   | 420b   | 420c   |
| 中文 420 | English 420 | 🚀 420 |

```rust
fn section_420() -> usize {
    let n = 420;
    n * 2
}
```

## Section 421 — sample heading 421

This is paragraph one of section 421. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 421 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-421).

- 列表项 A of section 421
- 列表项 B of section 421
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 421

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 421a   | 421b   | 421c   |
| 中文 421 | English 421 | 🚀 421 |

```rust
fn section_421() -> usize {
    let n = 421;
    n * 2
}
```

## Section 422 — sample heading 422

This is paragraph one of section 422. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 422 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-422).

- 列表项 A of section 422
- 列表项 B of section 422
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 422

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 422a   | 422b   | 422c   |
| 中文 422 | English 422 | 🚀 422 |

```rust
fn section_422() -> usize {
    let n = 422;
    n * 2
}
```

## Section 423 — sample heading 423

This is paragraph one of section 423. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 423 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-423).

- 列表项 A of section 423
- 列表项 B of section 423
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 423

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 423a   | 423b   | 423c   |
| 中文 423 | English 423 | 🚀 423 |

```rust
fn section_423() -> usize {
    let n = 423;
    n * 2
}
```

## Section 424 — sample heading 424

This is paragraph one of section 424. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 424 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-424).

- 列表项 A of section 424
- 列表项 B of section 424
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 424

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 424a   | 424b   | 424c   |
| 中文 424 | English 424 | 🚀 424 |

```rust
fn section_424() -> usize {
    let n = 424;
    n * 2
}
```

## Section 425 — sample heading 425

This is paragraph one of section 425. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 425 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-425).

- 列表项 A of section 425
- 列表项 B of section 425
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 425

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 425a   | 425b   | 425c   |
| 中文 425 | English 425 | 🚀 425 |

```rust
fn section_425() -> usize {
    let n = 425;
    n * 2
}
```

## Section 426 — sample heading 426

This is paragraph one of section 426. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 426 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-426).

- 列表项 A of section 426
- 列表项 B of section 426
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 426

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 426a   | 426b   | 426c   |
| 中文 426 | English 426 | 🚀 426 |

```rust
fn section_426() -> usize {
    let n = 426;
    n * 2
}
```

## Section 427 — sample heading 427

This is paragraph one of section 427. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 427 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-427).

- 列表项 A of section 427
- 列表项 B of section 427
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 427

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 427a   | 427b   | 427c   |
| 中文 427 | English 427 | 🚀 427 |

```rust
fn section_427() -> usize {
    let n = 427;
    n * 2
}
```

## Section 428 — sample heading 428

This is paragraph one of section 428. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 428 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-428).

- 列表项 A of section 428
- 列表项 B of section 428
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 428

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 428a   | 428b   | 428c   |
| 中文 428 | English 428 | 🚀 428 |

```rust
fn section_428() -> usize {
    let n = 428;
    n * 2
}
```

## Section 429 — sample heading 429

This is paragraph one of section 429. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 429 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-429).

- 列表项 A of section 429
- 列表项 B of section 429
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 429

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 429a   | 429b   | 429c   |
| 中文 429 | English 429 | 🚀 429 |

```rust
fn section_429() -> usize {
    let n = 429;
    n * 2
}
```

## Section 430 — sample heading 430

This is paragraph one of section 430. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 430 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-430).

- 列表项 A of section 430
- 列表项 B of section 430
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 430

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 430a   | 430b   | 430c   |
| 中文 430 | English 430 | 🚀 430 |

```rust
fn section_430() -> usize {
    let n = 430;
    n * 2
}
```

## Section 431 — sample heading 431

This is paragraph one of section 431. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 431 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-431).

- 列表项 A of section 431
- 列表项 B of section 431
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 431

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 431a   | 431b   | 431c   |
| 中文 431 | English 431 | 🚀 431 |

```rust
fn section_431() -> usize {
    let n = 431;
    n * 2
}
```

## Section 432 — sample heading 432

This is paragraph one of section 432. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 432 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-432).

- 列表项 A of section 432
- 列表项 B of section 432
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 432

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 432a   | 432b   | 432c   |
| 中文 432 | English 432 | 🚀 432 |

```rust
fn section_432() -> usize {
    let n = 432;
    n * 2
}
```

## Section 433 — sample heading 433

This is paragraph one of section 433. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 433 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-433).

- 列表项 A of section 433
- 列表项 B of section 433
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 433

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 433a   | 433b   | 433c   |
| 中文 433 | English 433 | 🚀 433 |

```rust
fn section_433() -> usize {
    let n = 433;
    n * 2
}
```

## Section 434 — sample heading 434

This is paragraph one of section 434. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 434 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-434).

- 列表项 A of section 434
- 列表项 B of section 434
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 434

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 434a   | 434b   | 434c   |
| 中文 434 | English 434 | 🚀 434 |

```rust
fn section_434() -> usize {
    let n = 434;
    n * 2
}
```

## Section 435 — sample heading 435

This is paragraph one of section 435. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 435 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-435).

- 列表项 A of section 435
- 列表项 B of section 435
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 435

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 435a   | 435b   | 435c   |
| 中文 435 | English 435 | 🚀 435 |

```rust
fn section_435() -> usize {
    let n = 435;
    n * 2
}
```

## Section 436 — sample heading 436

This is paragraph one of section 436. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 436 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-436).

- 列表项 A of section 436
- 列表项 B of section 436
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 436

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 436a   | 436b   | 436c   |
| 中文 436 | English 436 | 🚀 436 |

```rust
fn section_436() -> usize {
    let n = 436;
    n * 2
}
```

## Section 437 — sample heading 437

This is paragraph one of section 437. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 437 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-437).

- 列表项 A of section 437
- 列表项 B of section 437
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 437

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 437a   | 437b   | 437c   |
| 中文 437 | English 437 | 🚀 437 |

```rust
fn section_437() -> usize {
    let n = 437;
    n * 2
}
```

## Section 438 — sample heading 438

This is paragraph one of section 438. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 438 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-438).

- 列表项 A of section 438
- 列表项 B of section 438
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 438

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 438a   | 438b   | 438c   |
| 中文 438 | English 438 | 🚀 438 |

```rust
fn section_438() -> usize {
    let n = 438;
    n * 2
}
```

## Section 439 — sample heading 439

This is paragraph one of section 439. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 439 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-439).

- 列表项 A of section 439
- 列表项 B of section 439
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 439

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 439a   | 439b   | 439c   |
| 中文 439 | English 439 | 🚀 439 |

```rust
fn section_439() -> usize {
    let n = 439;
    n * 2
}
```

## Section 440 — sample heading 440

This is paragraph one of section 440. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 440 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-440).

- 列表项 A of section 440
- 列表项 B of section 440
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 440

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 440a   | 440b   | 440c   |
| 中文 440 | English 440 | 🚀 440 |

```rust
fn section_440() -> usize {
    let n = 440;
    n * 2
}
```

## Section 441 — sample heading 441

This is paragraph one of section 441. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 441 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-441).

- 列表项 A of section 441
- 列表项 B of section 441
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 441

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 441a   | 441b   | 441c   |
| 中文 441 | English 441 | 🚀 441 |

```rust
fn section_441() -> usize {
    let n = 441;
    n * 2
}
```

## Section 442 — sample heading 442

This is paragraph one of section 442. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 442 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-442).

- 列表项 A of section 442
- 列表项 B of section 442
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 442

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 442a   | 442b   | 442c   |
| 中文 442 | English 442 | 🚀 442 |

```rust
fn section_442() -> usize {
    let n = 442;
    n * 2
}
```

## Section 443 — sample heading 443

This is paragraph one of section 443. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 443 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-443).

- 列表项 A of section 443
- 列表项 B of section 443
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 443

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 443a   | 443b   | 443c   |
| 中文 443 | English 443 | 🚀 443 |

```rust
fn section_443() -> usize {
    let n = 443;
    n * 2
}
```

## Section 444 — sample heading 444

This is paragraph one of section 444. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 444 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-444).

- 列表项 A of section 444
- 列表项 B of section 444
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 444

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 444a   | 444b   | 444c   |
| 中文 444 | English 444 | 🚀 444 |

```rust
fn section_444() -> usize {
    let n = 444;
    n * 2
}
```

## Section 445 — sample heading 445

This is paragraph one of section 445. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 445 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-445).

- 列表项 A of section 445
- 列表项 B of section 445
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 445

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 445a   | 445b   | 445c   |
| 中文 445 | English 445 | 🚀 445 |

```rust
fn section_445() -> usize {
    let n = 445;
    n * 2
}
```

## Section 446 — sample heading 446

This is paragraph one of section 446. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 446 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-446).

- 列表项 A of section 446
- 列表项 B of section 446
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 446

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 446a   | 446b   | 446c   |
| 中文 446 | English 446 | 🚀 446 |

```rust
fn section_446() -> usize {
    let n = 446;
    n * 2
}
```

## Section 447 — sample heading 447

This is paragraph one of section 447. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 447 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-447).

- 列表项 A of section 447
- 列表项 B of section 447
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 447

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 447a   | 447b   | 447c   |
| 中文 447 | English 447 | 🚀 447 |

```rust
fn section_447() -> usize {
    let n = 447;
    n * 2
}
```

## Section 448 — sample heading 448

This is paragraph one of section 448. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 448 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-448).

- 列表项 A of section 448
- 列表项 B of section 448
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 448

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 448a   | 448b   | 448c   |
| 中文 448 | English 448 | 🚀 448 |

```rust
fn section_448() -> usize {
    let n = 448;
    n * 2
}
```

## Section 449 — sample heading 449

This is paragraph one of section 449. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 449 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-449).

- 列表项 A of section 449
- 列表项 B of section 449
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 449

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 449a   | 449b   | 449c   |
| 中文 449 | English 449 | 🚀 449 |

```rust
fn section_449() -> usize {
    let n = 449;
    n * 2
}
```

## Section 450 — sample heading 450

This is paragraph one of section 450. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 450 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-450).

- 列表项 A of section 450
- 列表项 B of section 450
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 450

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 450a   | 450b   | 450c   |
| 中文 450 | English 450 | 🚀 450 |

```rust
fn section_450() -> usize {
    let n = 450;
    n * 2
}
```

### Section 450.1 — extra depth marker

Every 50th section gets an H3 sub-section to vary the heading depth
across the document.

## Section 451 — sample heading 451

This is paragraph one of section 451. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 451 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-451).

- 列表项 A of section 451
- 列表项 B of section 451
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 451

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 451a   | 451b   | 451c   |
| 中文 451 | English 451 | 🚀 451 |

```rust
fn section_451() -> usize {
    let n = 451;
    n * 2
}
```

## Section 452 — sample heading 452

This is paragraph one of section 452. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 452 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-452).

- 列表项 A of section 452
- 列表项 B of section 452
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 452

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 452a   | 452b   | 452c   |
| 中文 452 | English 452 | 🚀 452 |

```rust
fn section_452() -> usize {
    let n = 452;
    n * 2
}
```

## Section 453 — sample heading 453

This is paragraph one of section 453. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 453 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-453).

- 列表项 A of section 453
- 列表项 B of section 453
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 453

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 453a   | 453b   | 453c   |
| 中文 453 | English 453 | 🚀 453 |

```rust
fn section_453() -> usize {
    let n = 453;
    n * 2
}
```

## Section 454 — sample heading 454

This is paragraph one of section 454. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 454 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-454).

- 列表项 A of section 454
- 列表项 B of section 454
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 454

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 454a   | 454b   | 454c   |
| 中文 454 | English 454 | 🚀 454 |

```rust
fn section_454() -> usize {
    let n = 454;
    n * 2
}
```

## Section 455 — sample heading 455

This is paragraph one of section 455. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 455 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-455).

- 列表项 A of section 455
- 列表项 B of section 455
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 455

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 455a   | 455b   | 455c   |
| 中文 455 | English 455 | 🚀 455 |

```rust
fn section_455() -> usize {
    let n = 455;
    n * 2
}
```

## Section 456 — sample heading 456

This is paragraph one of section 456. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 456 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-456).

- 列表项 A of section 456
- 列表项 B of section 456
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 456

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 456a   | 456b   | 456c   |
| 中文 456 | English 456 | 🚀 456 |

```rust
fn section_456() -> usize {
    let n = 456;
    n * 2
}
```

## Section 457 — sample heading 457

This is paragraph one of section 457. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 457 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-457).

- 列表项 A of section 457
- 列表项 B of section 457
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 457

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 457a   | 457b   | 457c   |
| 中文 457 | English 457 | 🚀 457 |

```rust
fn section_457() -> usize {
    let n = 457;
    n * 2
}
```

## Section 458 — sample heading 458

This is paragraph one of section 458. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 458 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-458).

- 列表项 A of section 458
- 列表项 B of section 458
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 458

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 458a   | 458b   | 458c   |
| 中文 458 | English 458 | 🚀 458 |

```rust
fn section_458() -> usize {
    let n = 458;
    n * 2
}
```

## Section 459 — sample heading 459

This is paragraph one of section 459. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 459 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-459).

- 列表项 A of section 459
- 列表项 B of section 459
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 459

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 459a   | 459b   | 459c   |
| 中文 459 | English 459 | 🚀 459 |

```rust
fn section_459() -> usize {
    let n = 459;
    n * 2
}
```

## Section 460 — sample heading 460

This is paragraph one of section 460. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 460 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-460).

- 列表项 A of section 460
- 列表项 B of section 460
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 460

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 460a   | 460b   | 460c   |
| 中文 460 | English 460 | 🚀 460 |

```rust
fn section_460() -> usize {
    let n = 460;
    n * 2
}
```

## Section 461 — sample heading 461

This is paragraph one of section 461. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 461 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-461).

- 列表项 A of section 461
- 列表项 B of section 461
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 461

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 461a   | 461b   | 461c   |
| 中文 461 | English 461 | 🚀 461 |

```rust
fn section_461() -> usize {
    let n = 461;
    n * 2
}
```

## Section 462 — sample heading 462

This is paragraph one of section 462. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 462 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-462).

- 列表项 A of section 462
- 列表项 B of section 462
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 462

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 462a   | 462b   | 462c   |
| 中文 462 | English 462 | 🚀 462 |

```rust
fn section_462() -> usize {
    let n = 462;
    n * 2
}
```

## Section 463 — sample heading 463

This is paragraph one of section 463. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 463 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-463).

- 列表项 A of section 463
- 列表项 B of section 463
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 463

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 463a   | 463b   | 463c   |
| 中文 463 | English 463 | 🚀 463 |

```rust
fn section_463() -> usize {
    let n = 463;
    n * 2
}
```

## Section 464 — sample heading 464

This is paragraph one of section 464. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 464 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-464).

- 列表项 A of section 464
- 列表项 B of section 464
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 464

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 464a   | 464b   | 464c   |
| 中文 464 | English 464 | 🚀 464 |

```rust
fn section_464() -> usize {
    let n = 464;
    n * 2
}
```

## Section 465 — sample heading 465

This is paragraph one of section 465. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 465 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-465).

- 列表项 A of section 465
- 列表项 B of section 465
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 465

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 465a   | 465b   | 465c   |
| 中文 465 | English 465 | 🚀 465 |

```rust
fn section_465() -> usize {
    let n = 465;
    n * 2
}
```

## Section 466 — sample heading 466

This is paragraph one of section 466. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 466 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-466).

- 列表项 A of section 466
- 列表项 B of section 466
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 466

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 466a   | 466b   | 466c   |
| 中文 466 | English 466 | 🚀 466 |

```rust
fn section_466() -> usize {
    let n = 466;
    n * 2
}
```

## Section 467 — sample heading 467

This is paragraph one of section 467. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 467 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-467).

- 列表项 A of section 467
- 列表项 B of section 467
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 467

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 467a   | 467b   | 467c   |
| 中文 467 | English 467 | 🚀 467 |

```rust
fn section_467() -> usize {
    let n = 467;
    n * 2
}
```

## Section 468 — sample heading 468

This is paragraph one of section 468. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 468 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-468).

- 列表项 A of section 468
- 列表项 B of section 468
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 468

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 468a   | 468b   | 468c   |
| 中文 468 | English 468 | 🚀 468 |

```rust
fn section_468() -> usize {
    let n = 468;
    n * 2
}
```

## Section 469 — sample heading 469

This is paragraph one of section 469. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 469 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-469).

- 列表项 A of section 469
- 列表项 B of section 469
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 469

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 469a   | 469b   | 469c   |
| 中文 469 | English 469 | 🚀 469 |

```rust
fn section_469() -> usize {
    let n = 469;
    n * 2
}
```

## Section 470 — sample heading 470

This is paragraph one of section 470. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 470 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-470).

- 列表项 A of section 470
- 列表项 B of section 470
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 470

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 470a   | 470b   | 470c   |
| 中文 470 | English 470 | 🚀 470 |

```rust
fn section_470() -> usize {
    let n = 470;
    n * 2
}
```

## Section 471 — sample heading 471

This is paragraph one of section 471. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 471 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-471).

- 列表项 A of section 471
- 列表项 B of section 471
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 471

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 471a   | 471b   | 471c   |
| 中文 471 | English 471 | 🚀 471 |

```rust
fn section_471() -> usize {
    let n = 471;
    n * 2
}
```

## Section 472 — sample heading 472

This is paragraph one of section 472. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 472 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-472).

- 列表项 A of section 472
- 列表项 B of section 472
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 472

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 472a   | 472b   | 472c   |
| 中文 472 | English 472 | 🚀 472 |

```rust
fn section_472() -> usize {
    let n = 472;
    n * 2
}
```

## Section 473 — sample heading 473

This is paragraph one of section 473. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 473 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-473).

- 列表项 A of section 473
- 列表项 B of section 473
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 473

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 473a   | 473b   | 473c   |
| 中文 473 | English 473 | 🚀 473 |

```rust
fn section_473() -> usize {
    let n = 473;
    n * 2
}
```

## Section 474 — sample heading 474

This is paragraph one of section 474. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 474 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-474).

- 列表项 A of section 474
- 列表项 B of section 474
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 474

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 474a   | 474b   | 474c   |
| 中文 474 | English 474 | 🚀 474 |

```rust
fn section_474() -> usize {
    let n = 474;
    n * 2
}
```

## Section 475 — sample heading 475

This is paragraph one of section 475. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 475 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-475).

- 列表项 A of section 475
- 列表项 B of section 475
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 475

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 475a   | 475b   | 475c   |
| 中文 475 | English 475 | 🚀 475 |

```rust
fn section_475() -> usize {
    let n = 475;
    n * 2
}
```

## Section 476 — sample heading 476

This is paragraph one of section 476. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 476 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-476).

- 列表项 A of section 476
- 列表项 B of section 476
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 476

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 476a   | 476b   | 476c   |
| 中文 476 | English 476 | 🚀 476 |

```rust
fn section_476() -> usize {
    let n = 476;
    n * 2
}
```

## Section 477 — sample heading 477

This is paragraph one of section 477. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 477 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-477).

- 列表项 A of section 477
- 列表项 B of section 477
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 477

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 477a   | 477b   | 477c   |
| 中文 477 | English 477 | 🚀 477 |

```rust
fn section_477() -> usize {
    let n = 477;
    n * 2
}
```

## Section 478 — sample heading 478

This is paragraph one of section 478. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 478 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-478).

- 列表项 A of section 478
- 列表项 B of section 478
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 478

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 478a   | 478b   | 478c   |
| 中文 478 | English 478 | 🚀 478 |

```rust
fn section_478() -> usize {
    let n = 478;
    n * 2
}
```

## Section 479 — sample heading 479

This is paragraph one of section 479. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 479 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-479).

- 列表项 A of section 479
- 列表项 B of section 479
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 479

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 479a   | 479b   | 479c   |
| 中文 479 | English 479 | 🚀 479 |

```rust
fn section_479() -> usize {
    let n = 479;
    n * 2
}
```

## Section 480 — sample heading 480

This is paragraph one of section 480. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 480 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-480).

- 列表项 A of section 480
- 列表项 B of section 480
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 480

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 480a   | 480b   | 480c   |
| 中文 480 | English 480 | 🚀 480 |

```rust
fn section_480() -> usize {
    let n = 480;
    n * 2
}
```

## Section 481 — sample heading 481

This is paragraph one of section 481. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 481 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-481).

- 列表项 A of section 481
- 列表项 B of section 481
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 481

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 481a   | 481b   | 481c   |
| 中文 481 | English 481 | 🚀 481 |

```rust
fn section_481() -> usize {
    let n = 481;
    n * 2
}
```

## Section 482 — sample heading 482

This is paragraph one of section 482. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 482 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-482).

- 列表项 A of section 482
- 列表项 B of section 482
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 482

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 482a   | 482b   | 482c   |
| 中文 482 | English 482 | 🚀 482 |

```rust
fn section_482() -> usize {
    let n = 482;
    n * 2
}
```

## Section 483 — sample heading 483

This is paragraph one of section 483. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 483 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-483).

- 列表项 A of section 483
- 列表项 B of section 483
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 483

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 483a   | 483b   | 483c   |
| 中文 483 | English 483 | 🚀 483 |

```rust
fn section_483() -> usize {
    let n = 483;
    n * 2
}
```

## Section 484 — sample heading 484

This is paragraph one of section 484. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 484 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-484).

- 列表项 A of section 484
- 列表项 B of section 484
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 484

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 484a   | 484b   | 484c   |
| 中文 484 | English 484 | 🚀 484 |

```rust
fn section_484() -> usize {
    let n = 484;
    n * 2
}
```

## Section 485 — sample heading 485

This is paragraph one of section 485. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 485 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-485).

- 列表项 A of section 485
- 列表项 B of section 485
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 485

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 485a   | 485b   | 485c   |
| 中文 485 | English 485 | 🚀 485 |

```rust
fn section_485() -> usize {
    let n = 485;
    n * 2
}
```

## Section 486 — sample heading 486

This is paragraph one of section 486. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 486 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-486).

- 列表项 A of section 486
- 列表项 B of section 486
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 486

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 486a   | 486b   | 486c   |
| 中文 486 | English 486 | 🚀 486 |

```rust
fn section_486() -> usize {
    let n = 486;
    n * 2
}
```

## Section 487 — sample heading 487

This is paragraph one of section 487. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 487 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-487).

- 列表项 A of section 487
- 列表项 B of section 487
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 487

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 487a   | 487b   | 487c   |
| 中文 487 | English 487 | 🚀 487 |

```rust
fn section_487() -> usize {
    let n = 487;
    n * 2
}
```

## Section 488 — sample heading 488

This is paragraph one of section 488. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 488 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-488).

- 列表项 A of section 488
- 列表项 B of section 488
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 488

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 488a   | 488b   | 488c   |
| 中文 488 | English 488 | 🚀 488 |

```rust
fn section_488() -> usize {
    let n = 488;
    n * 2
}
```

## Section 489 — sample heading 489

This is paragraph one of section 489. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 489 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-489).

- 列表项 A of section 489
- 列表项 B of section 489
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 489

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 489a   | 489b   | 489c   |
| 中文 489 | English 489 | 🚀 489 |

```rust
fn section_489() -> usize {
    let n = 489;
    n * 2
}
```

## Section 490 — sample heading 490

This is paragraph one of section 490. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 490 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-490).

- 列表项 A of section 490
- 列表项 B of section 490
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 490

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 490a   | 490b   | 490c   |
| 中文 490 | English 490 | 🚀 490 |

```rust
fn section_490() -> usize {
    let n = 490;
    n * 2
}
```

## Section 491 — sample heading 491

This is paragraph one of section 491. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 491 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-491).

- 列表项 A of section 491
- 列表项 B of section 491
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 491

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 491a   | 491b   | 491c   |
| 中文 491 | English 491 | 🚀 491 |

```rust
fn section_491() -> usize {
    let n = 491;
    n * 2
}
```

## Section 492 — sample heading 492

This is paragraph one of section 492. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 492 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-492).

- 列表项 A of section 492
- 列表项 B of section 492
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 492

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 492a   | 492b   | 492c   |
| 中文 492 | English 492 | 🚀 492 |

```rust
fn section_492() -> usize {
    let n = 492;
    n * 2
}
```

## Section 493 — sample heading 493

This is paragraph one of section 493. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 493 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-493).

- 列表项 A of section 493
- 列表项 B of section 493
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 493

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 493a   | 493b   | 493c   |
| 中文 493 | English 493 | 🚀 493 |

```rust
fn section_493() -> usize {
    let n = 493;
    n * 2
}
```

## Section 494 — sample heading 494

This is paragraph one of section 494. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 494 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-494).

- 列表项 A of section 494
- 列表项 B of section 494
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 494

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 494a   | 494b   | 494c   |
| 中文 494 | English 494 | 🚀 494 |

```rust
fn section_494() -> usize {
    let n = 494;
    n * 2
}
```

## Section 495 — sample heading 495

This is paragraph one of section 495. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 495 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-495).

- 列表项 A of section 495
- 列表项 B of section 495
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 495

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 495a   | 495b   | 495c   |
| 中文 495 | English 495 | 🚀 495 |

```rust
fn section_495() -> usize {
    let n = 495;
    n * 2
}
```

## Section 496 — sample heading 496

This is paragraph one of section 496. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 496 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-496).

- 列表项 A of section 496
- 列表项 B of section 496
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 496

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 496a   | 496b   | 496c   |
| 中文 496 | English 496 | 🚀 496 |

```rust
fn section_496() -> usize {
    let n = 496;
    n * 2
}
```

## Section 497 — sample heading 497

This is paragraph one of section 497. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 497 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-497).

- 列表项 A of section 497
- 列表项 B of section 497
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 497

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 497a   | 497b   | 497c   |
| 中文 497 | English 497 | 🚀 497 |

```rust
fn section_497() -> usize {
    let n = 497;
    n * 2
}
```

## Section 498 — sample heading 498

This is paragraph one of section 498. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 498 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-498).

- 列表项 A of section 498
- 列表项 B of section 498
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 498

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 498a   | 498b   | 498c   |
| 中文 498 | English 498 | 🚀 498 |

```rust
fn section_498() -> usize {
    let n = 498;
    n * 2
}
```

## Section 499 — sample heading 499

This is paragraph one of section 499. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 499 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-499).

- 列表项 A of section 499
- 列表项 B of section 499
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 499

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 499a   | 499b   | 499c   |
| 中文 499 | English 499 | 🚀 499 |

```rust
fn section_499() -> usize {
    let n = 499;
    n * 2
}
```

## Section 500 — sample heading 500

This is paragraph one of section 500. It mixes English and 中文 so width
calculation and bidi handling get exercised on every section. Lorem ipsum dolor
sit amet, consectetur adipiscing elit. 这一段额外加一些汉字以保证每节正文都不止
一行，让大文件里的换行/滚动路径也被覆盖。

This is paragraph two of section 500 with **bold**, *italic*, `inline code`,
and a [link](https://example.com/section-500).

- 列表项 A of section 500
- 列表项 B of section 500
  - nested B.1
  - nested B.2 with a longer line so it wraps inside narrow terminals and we
    can spot indent bugs on the large fixture too
- 列表项 C of section 500

| Col 1 | Col 2 | Col 3 |
| ----- | ----- | ----- |
| 500a   | 500b   | 500c   |
| 中文 500 | English 500 | 🚀 500 |

```rust
fn section_500() -> usize {
    let n = 500;
    n * 2
}
```

### Section 500.1 — extra depth marker

Every 50th section gets an H3 sub-section to vary the heading depth
across the document.

## Tail — Large Table Stress

The following table has many rows to exercise table rendering on long inputs.

| Index | Label     | Value | Description (mixed scripts)             |
| ----- | --------- | ----- | --------------------------------------- |
|     1 | row-1     |     7 | 描述行 1 / description row 1            |
|     2 | row-2     |    14 | 描述行 2 / description row 2            |
|     3 | row-3     |    21 | 描述行 3 / description row 3            |
|     4 | row-4     |    28 | 描述行 4 / description row 4            |
|     5 | row-5     |    35 | 描述行 5 / description row 5            |
|     6 | row-6     |    42 | 描述行 6 / description row 6            |
|     7 | row-7     |    49 | 描述行 7 / description row 7            |
|     8 | row-8     |    56 | 描述行 8 / description row 8            |
|     9 | row-9     |    63 | 描述行 9 / description row 9            |
|    10 | row-10    |    70 | 描述行 10 / description row 10            |
|    11 | row-11    |    77 | 描述行 11 / description row 11            |
|    12 | row-12    |    84 | 描述行 12 / description row 12            |
|    13 | row-13    |    91 | 描述行 13 / description row 13            |
|    14 | row-14    |    98 | 描述行 14 / description row 14            |
|    15 | row-15    |   105 | 描述行 15 / description row 15            |
|    16 | row-16    |   112 | 描述行 16 / description row 16            |
|    17 | row-17    |   119 | 描述行 17 / description row 17            |
|    18 | row-18    |   126 | 描述行 18 / description row 18            |
|    19 | row-19    |   133 | 描述行 19 / description row 19            |
|    20 | row-20    |   140 | 描述行 20 / description row 20            |
|    21 | row-21    |   147 | 描述行 21 / description row 21            |
|    22 | row-22    |   154 | 描述行 22 / description row 22            |
|    23 | row-23    |   161 | 描述行 23 / description row 23            |
|    24 | row-24    |   168 | 描述行 24 / description row 24            |
|    25 | row-25    |   175 | 描述行 25 / description row 25            |
|    26 | row-26    |   182 | 描述行 26 / description row 26            |
|    27 | row-27    |   189 | 描述行 27 / description row 27            |
|    28 | row-28    |   196 | 描述行 28 / description row 28            |
|    29 | row-29    |   203 | 描述行 29 / description row 29            |
|    30 | row-30    |   210 | 描述行 30 / description row 30            |
|    31 | row-31    |   217 | 描述行 31 / description row 31            |
|    32 | row-32    |   224 | 描述行 32 / description row 32            |
|    33 | row-33    |   231 | 描述行 33 / description row 33            |
|    34 | row-34    |   238 | 描述行 34 / description row 34            |
|    35 | row-35    |   245 | 描述行 35 / description row 35            |
|    36 | row-36    |   252 | 描述行 36 / description row 36            |
|    37 | row-37    |   259 | 描述行 37 / description row 37            |
|    38 | row-38    |   266 | 描述行 38 / description row 38            |
|    39 | row-39    |   273 | 描述行 39 / description row 39            |
|    40 | row-40    |   280 | 描述行 40 / description row 40            |
|    41 | row-41    |   287 | 描述行 41 / description row 41            |
|    42 | row-42    |   294 | 描述行 42 / description row 42            |
|    43 | row-43    |   301 | 描述行 43 / description row 43            |
|    44 | row-44    |   308 | 描述行 44 / description row 44            |
|    45 | row-45    |   315 | 描述行 45 / description row 45            |
|    46 | row-46    |   322 | 描述行 46 / description row 46            |
|    47 | row-47    |   329 | 描述行 47 / description row 47            |
|    48 | row-48    |   336 | 描述行 48 / description row 48            |
|    49 | row-49    |   343 | 描述行 49 / description row 49            |
|    50 | row-50    |   350 | 描述行 50 / description row 50            |
|    51 | row-51    |   357 | 描述行 51 / description row 51            |
|    52 | row-52    |   364 | 描述行 52 / description row 52            |
|    53 | row-53    |   371 | 描述行 53 / description row 53            |
|    54 | row-54    |   378 | 描述行 54 / description row 54            |
|    55 | row-55    |   385 | 描述行 55 / description row 55            |
|    56 | row-56    |   392 | 描述行 56 / description row 56            |
|    57 | row-57    |   399 | 描述行 57 / description row 57            |
|    58 | row-58    |   406 | 描述行 58 / description row 58            |
|    59 | row-59    |   413 | 描述行 59 / description row 59            |
|    60 | row-60    |   420 | 描述行 60 / description row 60            |
|    61 | row-61    |   427 | 描述行 61 / description row 61            |
|    62 | row-62    |   434 | 描述行 62 / description row 62            |
|    63 | row-63    |   441 | 描述行 63 / description row 63            |
|    64 | row-64    |   448 | 描述行 64 / description row 64            |
|    65 | row-65    |   455 | 描述行 65 / description row 65            |
|    66 | row-66    |   462 | 描述行 66 / description row 66            |
|    67 | row-67    |   469 | 描述行 67 / description row 67            |
|    68 | row-68    |   476 | 描述行 68 / description row 68            |
|    69 | row-69    |   483 | 描述行 69 / description row 69            |
|    70 | row-70    |   490 | 描述行 70 / description row 70            |
|    71 | row-71    |   497 | 描述行 71 / description row 71            |
|    72 | row-72    |   504 | 描述行 72 / description row 72            |
|    73 | row-73    |   511 | 描述行 73 / description row 73            |
|    74 | row-74    |   518 | 描述行 74 / description row 74            |
|    75 | row-75    |   525 | 描述行 75 / description row 75            |
|    76 | row-76    |   532 | 描述行 76 / description row 76            |
|    77 | row-77    |   539 | 描述行 77 / description row 77            |
|    78 | row-78    |   546 | 描述行 78 / description row 78            |
|    79 | row-79    |   553 | 描述行 79 / description row 79            |
|    80 | row-80    |   560 | 描述行 80 / description row 80            |
|    81 | row-81    |   567 | 描述行 81 / description row 81            |
|    82 | row-82    |   574 | 描述行 82 / description row 82            |
|    83 | row-83    |   581 | 描述行 83 / description row 83            |
|    84 | row-84    |   588 | 描述行 84 / description row 84            |
|    85 | row-85    |   595 | 描述行 85 / description row 85            |
|    86 | row-86    |   602 | 描述行 86 / description row 86            |
|    87 | row-87    |   609 | 描述行 87 / description row 87            |
|    88 | row-88    |   616 | 描述行 88 / description row 88            |
|    89 | row-89    |   623 | 描述行 89 / description row 89            |
|    90 | row-90    |   630 | 描述行 90 / description row 90            |
|    91 | row-91    |   637 | 描述行 91 / description row 91            |
|    92 | row-92    |   644 | 描述行 92 / description row 92            |
|    93 | row-93    |   651 | 描述行 93 / description row 93            |
|    94 | row-94    |   658 | 描述行 94 / description row 94            |
|    95 | row-95    |   665 | 描述行 95 / description row 95            |
|    96 | row-96    |   672 | 描述行 96 / description row 96            |
|    97 | row-97    |   679 | 描述行 97 / description row 97            |
|    98 | row-98    |   686 | 描述行 98 / description row 98            |
|    99 | row-99    |   693 | 描述行 99 / description row 99            |
|   100 | row-100   |   700 | 描述行 100 / description row 100            |
|   101 | row-101   |   707 | 描述行 101 / description row 101            |
|   102 | row-102   |   714 | 描述行 102 / description row 102            |
|   103 | row-103   |   721 | 描述行 103 / description row 103            |
|   104 | row-104   |   728 | 描述行 104 / description row 104            |
|   105 | row-105   |   735 | 描述行 105 / description row 105            |
|   106 | row-106   |   742 | 描述行 106 / description row 106            |
|   107 | row-107   |   749 | 描述行 107 / description row 107            |
|   108 | row-108   |   756 | 描述行 108 / description row 108            |
|   109 | row-109   |   763 | 描述行 109 / description row 109            |
|   110 | row-110   |   770 | 描述行 110 / description row 110            |
|   111 | row-111   |   777 | 描述行 111 / description row 111            |
|   112 | row-112   |   784 | 描述行 112 / description row 112            |
|   113 | row-113   |   791 | 描述行 113 / description row 113            |
|   114 | row-114   |   798 | 描述行 114 / description row 114            |
|   115 | row-115   |   805 | 描述行 115 / description row 115            |
|   116 | row-116   |   812 | 描述行 116 / description row 116            |
|   117 | row-117   |   819 | 描述行 117 / description row 117            |
|   118 | row-118   |   826 | 描述行 118 / description row 118            |
|   119 | row-119   |   833 | 描述行 119 / description row 119            |
|   120 | row-120   |   840 | 描述行 120 / description row 120            |
|   121 | row-121   |   847 | 描述行 121 / description row 121            |
|   122 | row-122   |   854 | 描述行 122 / description row 122            |
|   123 | row-123   |   861 | 描述行 123 / description row 123            |
|   124 | row-124   |   868 | 描述行 124 / description row 124            |
|   125 | row-125   |   875 | 描述行 125 / description row 125            |
|   126 | row-126   |   882 | 描述行 126 / description row 126            |
|   127 | row-127   |   889 | 描述行 127 / description row 127            |
|   128 | row-128   |   896 | 描述行 128 / description row 128            |
|   129 | row-129   |   903 | 描述行 129 / description row 129            |
|   130 | row-130   |   910 | 描述行 130 / description row 130            |
|   131 | row-131   |   917 | 描述行 131 / description row 131            |
|   132 | row-132   |   924 | 描述行 132 / description row 132            |
|   133 | row-133   |   931 | 描述行 133 / description row 133            |
|   134 | row-134   |   938 | 描述行 134 / description row 134            |
|   135 | row-135   |   945 | 描述行 135 / description row 135            |
|   136 | row-136   |   952 | 描述行 136 / description row 136            |
|   137 | row-137   |   959 | 描述行 137 / description row 137            |
|   138 | row-138   |   966 | 描述行 138 / description row 138            |
|   139 | row-139   |   973 | 描述行 139 / description row 139            |
|   140 | row-140   |   980 | 描述行 140 / description row 140            |
|   141 | row-141   |   987 | 描述行 141 / description row 141            |
|   142 | row-142   |   994 | 描述行 142 / description row 142            |
|   143 | row-143   |  1001 | 描述行 143 / description row 143            |
|   144 | row-144   |  1008 | 描述行 144 / description row 144            |
|   145 | row-145   |  1015 | 描述行 145 / description row 145            |
|   146 | row-146   |  1022 | 描述行 146 / description row 146            |
|   147 | row-147   |  1029 | 描述行 147 / description row 147            |
|   148 | row-148   |  1036 | 描述行 148 / description row 148            |
|   149 | row-149   |  1043 | 描述行 149 / description row 149            |
|   150 | row-150   |  1050 | 描述行 150 / description row 150            |
|   151 | row-151   |  1057 | 描述行 151 / description row 151            |
|   152 | row-152   |  1064 | 描述行 152 / description row 152            |
|   153 | row-153   |  1071 | 描述行 153 / description row 153            |
|   154 | row-154   |  1078 | 描述行 154 / description row 154            |
|   155 | row-155   |  1085 | 描述行 155 / description row 155            |
|   156 | row-156   |  1092 | 描述行 156 / description row 156            |
|   157 | row-157   |  1099 | 描述行 157 / description row 157            |
|   158 | row-158   |  1106 | 描述行 158 / description row 158            |
|   159 | row-159   |  1113 | 描述行 159 / description row 159            |
|   160 | row-160   |  1120 | 描述行 160 / description row 160            |
|   161 | row-161   |  1127 | 描述行 161 / description row 161            |
|   162 | row-162   |  1134 | 描述行 162 / description row 162            |
|   163 | row-163   |  1141 | 描述行 163 / description row 163            |
|   164 | row-164   |  1148 | 描述行 164 / description row 164            |
|   165 | row-165   |  1155 | 描述行 165 / description row 165            |
|   166 | row-166   |  1162 | 描述行 166 / description row 166            |
|   167 | row-167   |  1169 | 描述行 167 / description row 167            |
|   168 | row-168   |  1176 | 描述行 168 / description row 168            |
|   169 | row-169   |  1183 | 描述行 169 / description row 169            |
|   170 | row-170   |  1190 | 描述行 170 / description row 170            |
|   171 | row-171   |  1197 | 描述行 171 / description row 171            |
|   172 | row-172   |  1204 | 描述行 172 / description row 172            |
|   173 | row-173   |  1211 | 描述行 173 / description row 173            |
|   174 | row-174   |  1218 | 描述行 174 / description row 174            |
|   175 | row-175   |  1225 | 描述行 175 / description row 175            |
|   176 | row-176   |  1232 | 描述行 176 / description row 176            |
|   177 | row-177   |  1239 | 描述行 177 / description row 177            |
|   178 | row-178   |  1246 | 描述行 178 / description row 178            |
|   179 | row-179   |  1253 | 描述行 179 / description row 179            |
|   180 | row-180   |  1260 | 描述行 180 / description row 180            |
|   181 | row-181   |  1267 | 描述行 181 / description row 181            |
|   182 | row-182   |  1274 | 描述行 182 / description row 182            |
|   183 | row-183   |  1281 | 描述行 183 / description row 183            |
|   184 | row-184   |  1288 | 描述行 184 / description row 184            |
|   185 | row-185   |  1295 | 描述行 185 / description row 185            |
|   186 | row-186   |  1302 | 描述行 186 / description row 186            |
|   187 | row-187   |  1309 | 描述行 187 / description row 187            |
|   188 | row-188   |  1316 | 描述行 188 / description row 188            |
|   189 | row-189   |  1323 | 描述行 189 / description row 189            |
|   190 | row-190   |  1330 | 描述行 190 / description row 190            |
|   191 | row-191   |  1337 | 描述行 191 / description row 191            |
|   192 | row-192   |  1344 | 描述行 192 / description row 192            |
|   193 | row-193   |  1351 | 描述行 193 / description row 193            |
|   194 | row-194   |  1358 | 描述行 194 / description row 194            |
|   195 | row-195   |  1365 | 描述行 195 / description row 195            |
|   196 | row-196   |  1372 | 描述行 196 / description row 196            |
|   197 | row-197   |  1379 | 描述行 197 / description row 197            |
|   198 | row-198   |  1386 | 描述行 198 / description row 198            |
|   199 | row-199   |  1393 | 描述行 199 / description row 199            |
|   200 | row-200   |  1400 | 描述行 200 / description row 200            |

## End

Generated by `scripts/gen-large-fixture.sh`. Re-run after changing the script.
