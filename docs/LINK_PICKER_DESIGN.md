# Link Picker — Design Options

## Problem

Current `LinkSelect` mode (`src/tui/mod.rs:451-462`, `:924-932`):

- Collects every link in the viewport via `visible_links`.
- Status-bar overlay shows up to **9** labels (`take(9)` + `…`), keybinding only accepts digits `1`–`9`.
- When the viewport holds many links, or labels are long, the single status-bar row truncates and the 10th+ links are unselectable.

Two alternative designs are recorded here. Neither has been implemented yet.

---

## Option C — Vimium-style inline hints

Paint a short label directly next to each visible link, in the body itself.

### UX

1. User presses `Enter` on a viewport with ≥2 links.
2. Each link is prefixed with a styled label: `[1]`, `[2]`, … `[9]`, `[a]`, `[b]`, … For >26 links, use two-char labels like `[aa]`, `[ab]`.
3. User types the label. Single-char labels fire immediately; multi-char labels commit after the second keystroke. Partial matches stay pending and filter the remaining hints (like Vimium "linkHints filter").
4. `Esc` cancels.

### Pros

- Scales to arbitrarily many links.
- Positional: the user sees *which* link each label points at without scanning a status bar.
- Works even when the body is dense with links.

### Cons / open questions

- Rendering: labels must be injected as styled prefix spans inside `clipped_spans`. Must not shift the body layout's column alignment for heading images (`MARGIN_WIDTH` gutter) or break search-match byte offsets.
- If labels are injected as real text, the wrap cache needs to reflow; probably easier to inject labels *only* for the active frame via a render-time overlay, not as layout-level spans.
- Label alphabet and length policy: prefer "home-row first" (`a s d f j k l`) like Vimium, or keep `1–9` plus `a–z`? Pick before implementing.
- Interaction with kitty heading images — labels sit in text rows, so no conflict, but the column math for image placement must ignore the injected label width.

---

## Option D — Links side panel (parallel to ToC)

Reuse the existing ToC sidebar pattern. Add a `Mode::Links` (or reuse the TOC panel slot with a tab switch) toggled by `l`.

### UX

1. User presses `l` → a left panel opens (same 30-col width as ToC today, see `src/tui/mod.rs:805-842`).
2. Panel lists every link in the **whole document** (not just viewport), grouped visually by heading section, each entry formatted as `<link-text>  →  <url>` or similar. External vs local `.md` gets a type badge (`↗` external, `↪` local).
3. `j`/`k` (or arrows) moves the selection; `Enter` opens the selected link (follows existing `open_link_target` path).
4. `l` again, or `Esc`, closes the panel.
5. Selection state is per-doc (lives on `DocEntry`, like `toc_open`), so back/forward preserves where the user was in the link list.

### Pros

- No new overlay paradigm — mirrors the existing `t`/ToC interaction, low learning cost.
- Covers *all* links in the doc, not just the ones in the current viewport. Good for documents with many cross-references.
- Easy to show extra metadata inline (URL, type badge, maybe "visited" marker via history stack).
- Scrollable list; no label-alphabet cap.

### Cons / open questions

- Loses positional context — user sees a list, not "this specific link I'm looking at on the page."
- Body width shrinks while the panel is open; kitty image placements must re-register (already handled for ToC via `needs_full_redraw`).
- If user wants to pick a link that's currently under the cursor, going through a list feels heavier than pressing Enter once.
- Can `l` coexist with ToC open? Cleanest: only one left panel at a time; opening Links auto-closes ToC and vice-versa.

---

## Recommendation (not final)

Options C and D address different use cases:

- **C** optimizes for the *"I'm reading and want to follow a specific link I can see"* flow.
- **D** optimizes for the *"I want to survey every link in this doc"* flow.

They are complementary. A realistic plan could be:

1. Ship **D** first — it's a straightforward extension of existing ToC infrastructure and immediately unblocks the >9-link case.
2. Revisit **C** later if the inline-hint flow feels worth the rendering complexity.

Either way, the status-bar `take(9)` overlay should be retired once a replacement lands.
