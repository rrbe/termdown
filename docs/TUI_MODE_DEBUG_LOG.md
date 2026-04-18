# TUI Mode — Debug Log

Running log of bugs surfaced during manual QA of `--tui` and the fixes
applied. This document exists so that a fresh session can resume
debugging without re-deriving the investigation.

Paired with:
- `docs/TUI_MODE_DESIGN.md` — the authoritative design.
- `docs/TUI_MODE_RESEARCH.md` — library / protocol background.
- `docs/TUI_MODE_PLAN.md` — task-by-task implementation plan.

---

## Branch & Status

- Working branch: `feature/tui-mode` (37 implementation commits on top
  of `master`).
- `make check` is green — 77 tests, 0 clippy warnings.
- Phase 8.1 (manual QA in Ghostty/iTerm2) is the current step. All
  code-authored phases are complete but real-terminal testing has
  surfaced the bugs below.

---

## Round 1 — First Manual Run

**Symptom.** All heading images rendered stacked on row 0 across the
top of the terminal, side-by-side, overlapping each other. Right edge
showed fragments of heading text as single stacked letters.

**Root cause.** `src/render.rs::place` was encoding cell coordinates
into the Kitty APC `x=`/`y=` parameters. Those keys are
**source-image pixel offsets (for cropping)**, not terminal cells.
Kitty `a=p` places at the **current cursor position**; to place at a
specific cell you must first emit a CUP (`\x1b[R;CH`) and *then* send
`a=p`.

Also: `transmit` used `a=T` (transmit **and display**), polluting the
startup screen. The right form for the TUI lifecycle is `a=t`
(transmit only, place later via `a=p`).

**Fix (`30fc57e`).**

```rust
// src/render.rs
pub fn place<W: Write>(w: &mut W, id: u32, col: u16, row: u16) -> io::Result<()> {
    // Move cursor first (1-indexed), then place the image there.
    write!(w, "\x1b[{};{}H\x1b_Ga=p,i={id},q=2;\x1b\\", row + 1, col + 1)
}
```

And changed both `a=T` occurrences in `transmit` to `a=t`.

Tests were updated (`transmit_produces_a_eq_t_with_id`,
`place_produces_cursor_move_then_a_eq_p`) and the
`sync_enters_new_moves_and_leaves` test in `tui/kitty.rs` was updated
to expect the new CUP prefix.

---

## Round 2 — Second Manual Run

**Symptoms reported.**

1. First frame looked mostly correct, but:
   - Body text started at column 0 — no left margin (cat mode uses 4
     columns).
   - A red vertical pixel line appeared on the far-left edge — likely
     a Kitty image artifact bleeding past its row budget.
   - The bottom-most H2 heading ("TUI 模式" in the Chinese README)
     visibly overlapped the status bar.

2. After pressing `j`/`k` to scroll, the screen **degraded
   progressively** with each keypress:
   - Old text from previous frames persisted on screen while new text
     layered on top.
   - Right ~30–40% of the terminal went blank.
   - Status bar drifted upward out of the bottom row.
   - Chinese and English snippets stacked visibly.

**Hypotheses investigated** (opus pass).

| ID | Hypothesis | Verdict |
|----|------------|---------|
| H1 | Ratatui's incremental diff leaves stale cells when RLines change shape | Confirmed |
| H2 | `Viewport::width` cached once at startup, never resyncs on resize | Confirmed |
| H3 | No `MARGIN_WIDTH` prefix on TUI body RLines | Confirmed |
| H4 | Hardcoded `rows` estimates (H1=6, H2=4, H3=3) under-count real PNG cell height | Confirmed |
| H5 | Cat mode's output changed | Not changed — snapshots still pass |
| H6 | Kitty `a=p` advances cursor past image; can scroll the visible region when placing near bottom | **This was the biggest offender.** Needed `C=1` on every place. |

**Fix (`bec090a`).**

Multi-file change in `src/render.rs`, `src/layout.rs`,
`src/tui/kitty.rs`, `src/tui/mod.rs`. Highlights:

- `render_heading` now returns `Option<(Vec<u8>, u32, u32)>` (png +
  pixel width + pixel height). `HeadingImage` gained `px_width` /
  `px_height` fields. Layout threads these through.
- `place` now emits `C=1` (don't advance cursor after placement) in
  addition to the CUP prefix:
  `\x1b[{row+1};{col+1}H\x1b_Ga=p,i={id},q=2,C=1;\x1b\\`.
- New `ImageLifecycle::reset_placements` method — deletes every
  current placement without discarding the cached PNG data.
- Event loop now:
  - Re-queries `terminal.size()` each iteration, syncs
    `viewport.width`/`height`, invalidating the wrap cache on change.
  - Poll timeout raised from 16ms to 50ms (less CPU spin).
  - Every processed event sets `needs_full_redraw = true`; the next
    iteration calls `terminal.clear()` + `reset_placements` before
    `terminal.draw`. Belt-and-braces guard against the stale-cell
    class of bugs.
- `draw` prepends a 4-space `RSpan` to every body RLine so the text
  column aligns with image column (`MARGIN_WIDTH = 4`).
- Heading image `rows` are now computed from the queried terminal
  cell pixel height when available (via
  `crossterm::terminal::window_size()`):
  `rows = ceil(png_height_px / cell_pixel_height)`. Fall back to the
  hardcoded per-level estimates if the terminal reports pixel size 0.
- `ToggleToc` no longer manually adjusts viewport width — the
  event-loop resync handles it.

**Status.** Committed; `make check` green. **Not yet verified by the
user in a real terminal.** The user hit a usage limit before re-testing.

---

## Open Risks / What To Verify Next

Things that the Round 2 fix *should* resolve but haven't been
confirmed in a real TTY yet. Test these on both Ghostty and iTerm2:

1. **Red vertical line on the left edge.** Most plausibly a cascading
   side-effect of the cursor-advance bug (H6). Should be gone now.
   If it persists, suspect: a 1-pixel purple H1 background strip
   bleeding at an image edge, or a stale left-column placement that
   `reset_placements` isn't catching.

2. **Bottom-heading overlap with the status bar.** Should be resolved
   by the real-pixel-height-based row computation. If still present:
   the queried cell pixel height may be wrong on that terminal — add
   a generous floor (e.g. `rows.max(expected_level_floor)`), or
   switch to an APC-based cell-size query (`\x1b[16t`).

3. **Progressive corruption on scroll.** Fully invalidated by
   `terminal.clear() + reset_placements` on every event. If it
   *still* happens, the Kitty backend may be holding onto pixel data
   across `\x1b[2J` — investigate whether `reset_placements` is
   actually reaching the terminal (stdout flush ordering).

4. **Right-half blank area after scroll.** Should be resolved by
   per-frame viewport-width resync. If still present: check the
   `body_area` vs `viewport.width` calculation when ToC is toggled.

5. **Flicker.** The Round 2 fix calls `terminal.clear()` on every
   event. On some terminals this produces visible flicker. If so,
   move the dirty-flag logic to only set `needs_full_redraw` when the
   viewport top or mode actually changes — skip it for events that
   don't visually change anything (e.g. Ctrl-modifier-only keys).

6. **Terminals that don't report pixel size.** `window_size()` can
   return 0 for pixel fields. Our fallback keeps the old per-level
   estimates, which under-count. Consider bumping those fallbacks to
   H1=8, H2=6, H3=4, or add an APC-based `\x1b[16t` probe.

7. **Performance on long docs.** `reset_placements` emits one
   `delete_placement` per cached image every frame. For a long doc
   with dozens of headings and held-`j` scrolling, this is
   measurable. If it becomes a bottleneck, revert to the diff-based
   `sync` path on non-clear frames (only reset when `needs_full_redraw`
   was set).

---

## If Everything Is Fine After Round 2

- Close out Phase 8.1 and merge `feature/tui-mode` to `master`.
- Delete this file or archive it under `docs/archive/`.

## If Further Rounds Are Needed

- Add a new `### Round N — ...` section above this one.
- Always name the exact commit SHAs applied and the specific
  hypotheses tested.
- Keep `make check` green at each round; manual-only behavior fixes
  still need snapshot validation that cat mode is unaffected.
