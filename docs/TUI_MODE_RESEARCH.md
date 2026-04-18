# TUI Mode — Research & Approach Comparison

Background notes that led to the approach chosen for termdown's `--tui` mode.
The final design lives in `docs/superpowers/specs/` once it is written.

## Problem Statement

termdown today is a `cat`-like Markdown renderer: it prints once and exits.
Long documents exceed the terminal height and become hard to navigate.
We want a second mode — activated via `--tui` — that provides vim-style
paging, forward/back, and search so users can browse long documents.

## Pagers, Editors, TUIs — What's the Difference?

The term "TUI" loosely means "runs in alternate screen + raw mode + keyboard-driven".
Within that umbrella, the complexity varies widely.

| Tool | Category | Core model | Complexity |
|---|---|---|---|
| `less` | Minimal pager | `Vec<Line>` + viewport offset; hand-rolled input loop | Low |
| `mdfried` | Ratatui-based TUI | Widget tree + immediate-mode redraw each frame | Medium |
| `vim` | Modal editor | Full cursor control, modal state, windows, plugins | High |

termdown needs the middle ground: more than `less` (wants structured layout,
status bar, ToC, search highlight), less than `vim` (not editing).

## Rust TUI Library Landscape (early 2026)

| Library | Role | Activity | Used by |
|---|---|---|---|
| **ratatui** | De-facto standard immediate-mode framework | 🟢 Very active | gitui, bottom, atuin, yazi, helix's early UI, mdfried |
| **cursive** | Retained-mode widget tree | 🟡 Maintained, slow | Menu-driven apps (less suitable for free-scroll text) |
| **termwiz** | Low-level primitives + thin widget layer | 🟢 Active | wezterm itself |
| **crossterm** | Cross-platform terminal primitives (not a TUI lib) | 🟢 Active | ratatui and hand-rolled pagers |
| **termion** | Unix-only primitives, predecessor of crossterm | 🟡 | Legacy |

Fringe options (`iocraft`, `ratzilla`, dioxus TUI renderer) are either too
new or aimed at different targets and were not considered.

## Three Implementation Approaches

### Approach 1 — Pure `crossterm` (less-style, roll everything)

- Dependencies: `crossterm` only.
- Model: `Vec<RenderedLine>` + `top` offset; move cursor + write; hand-rolled
  wrap, status bar, search prompt, image placement tracking.
- Estimated size: 1500–2000 lines for v1.
- Pros: smallest dependency footprint; full control.
- Cons: reinvents everything ratatui provides; Kitty image lifecycle
  (clip, redraw, delete, resize) all DIY.

### Approach 2 — `ratatui` + `ratatui-image`

- Dependencies: ratatui, crossterm, ratatui-image, tui-textarea, regex.
- Model: every frame `terminal.draw(|f| {...})`; wrap, layout, borders from
  widgets; headings go through `ratatui-image::StatefulImage`.
- Estimated size: 800–1200 lines for v1.
- Pros: wrap, Layout split, scrollbar, status bar, search highlight all
  framework-native; path is proven (mdfried uses this stack).
- Cons: image-handling inherits `ratatui-image`'s synchronous encode +
  re-transmission behavior (see "Performance Investigation" below).

### Approach 3 — `ratatui` + self-managed Kitty images (**chosen**)

- Dependencies: ratatui, crossterm, tui-textarea, regex.
- Model: text goes through ratatui; heading images are transmitted to the
  terminal once with an id (Kitty `a=T, i=N`), and on each redraw we emit
  lightweight placement commands (`a=p, i=N, x=col, y=row`). No image bytes
  leave memory during scrolling.
- Estimated size: 1000–1500 lines for v1.
- Pros: control over image lifecycle; avoids the re-transmission cost that
  is the most plausible cause of mdfried's sluggishness; extends code
  already in `render.rs`.
- Cons: need to coordinate image placement with ratatui's per-frame clear;
  `a=d` delete for scroll-off cases needs careful state tracking.

## Performance Investigation — "Why does mdfried feel sluggish?"

User report: mdfried feels laggy when scrolling long documents.

Searched for public confirmation:

- `benjajaja/mdfried` issues page (10 open at time of search): **no
  performance-tagged issues**. Either the pool of users is small or people
  accept the lag as inherent to "TUI + images".
- `ratatui-image` README explicitly states: *"resizing and encoding is
  **blocking** by default, but it is possible to offload this to another
  thread or async task"* — acknowledges the cost.
- Same README: *"Kitty graphics protocol is essentially stateful, but at
  least provides a way to re-render an image that has been loaded, at a
  different or same position."* — i.e. the `a=p` placement path exists but
  is not the default widget behavior.
- `ratatui-image` exposes `Image` (stateless, re-encodes each frame) and
  `StatefulImage` (more robust but still synchronous encode).

### Likely causes of mdfried's lag (ranked)

1. **Per-frame image re-transmission.** ratatui's immediate-mode redraw
   doesn't know an image is unchanged; `ratatui-image` conservatively
   re-sends base64 PNG data each frame. One H1 ≈ 10–40 KB base64; three
   headings on-screen × 30 fps of held-j scrolling = MB/s of escape data
   pushed to the terminal.
2. **Wrap / layout in the draw loop.** If layout runs on every frame rather
   than once at load time, it's O(N lines) per frame.
3. **No scroll throttling / coalescing.** Key repeat (~30/s) drives a full
   redraw each event.
4. **Unbuffered stdout writes.** Each terminal write as a syscall amplifies
   the overhead of (1).

### How termdown's approach 3 avoids each

| Cause | Mitigation in approach 3 |
|---|---|
| Per-frame re-transmission | Transmit each heading PNG once with an id; subsequent frames emit placement-only commands (~dozens of bytes) |
| Wrap in draw loop | Generate `Vec<Line>` once at load; maintain a wrap cache keyed on terminal width |
| Scroll thrash | Coalesce scroll events within a tick; redraw once per frame budget |
| Unbuffered writes | Flush a `BufWriter` once per frame |

## Decision

**Approach 3** — ratatui for text layout and widgets, self-managed Kitty
image lifecycle via `a=T` + `a=p`.

Rationale:

- mdfried's observed lag matches exactly the default path `ratatui-image`
  documents. Avoiding that path is the point.
- termdown already owns half of the Kitty protocol plumbing in `render.rs`;
  extending it with id-based placement is a natural fit.
- ratatui's layout, wrap, and diff rendering handle every UI need except
  the one thing we want to control ourselves (images).
- Adds four dependencies; `strip + lto` cost expected ≤ 2–3 MB.

## Shared Layout Module

Orthogonal to the TUI-library decision: introduce `layout.rs` that turns a
pulldown-cmark event stream into a structured `Vec<Line>`. Both the cat
path and the TUI path consume it. This prevents the two rendering paths
from drifting apart as features are added.

## References

- [benjajaja/mdfried — issues](https://github.com/benjajaja/mdfried/issues)
- [ratatui-image — crates.io](https://crates.io/crates/ratatui-image)
- [ratatui-image — README](https://github.com/benjajaja/ratatui-image/blob/master/README.md)
- [ratatui — rendering concepts](https://ratatui.rs/concepts/rendering/)
- Kitty graphics protocol — image id + placement semantics (`a=T` transmit,
  `a=p` put, `a=d` delete)
