# TUI Mode — Design

Design for termdown's `--tui` mode. Research and approach comparison
that led to this design lives in `TUI_MODE_RESEARCH.md`. Read that first
if you want the "why this stack"; this doc is the "what we're building".

## Goals

- Browse Markdown documents larger than one screen with vim-style
  navigation (paging, `gg`/`G`, heading jumps, `/` search, `n`/`N`).
- Preserve termdown's Kitty-graphics heading rendering without the
  per-frame re-transmission cost that makes similar tools feel sluggish.
- Share the Markdown → rendered output pipeline between the cat path
  (default) and the TUI path; don't fork rendering logic.

## Non-Goals (v1)

- Regex search (literal substring only).
- Mouse support (mdfried's mouse-vs-text-selection tradeoff is worse
  than no mouse).
- Syntax highlighting inside code blocks.
- `--tui` piped from stdin (TUI needs the terminal for both key input
  and document data; requiring a file path avoids the conflict).
- Configurable key bindings.

## Activation

- Explicit `--tui` flag required. Default remains cat-style output.
- `termdown --tui FILE.md` — enters TUI on `FILE.md`.
- `termdown --tui` with no file or with `-` → error, exit non-zero.
- Future evolution (documented but not implemented in v1):
  - Automatic mode when output is a TTY and the rendered document
    exceeds terminal height (git-log-style).
  - `[tui]` section in `~/.termdown/config.toml` to opt into automatic
    mode or override defaults.

Single binary, no cargo feature flag. TUI code is always compiled in;
strip + LTO keep the binary growth acceptable (~2-3 MB expected).

## Module Layout

```
src/
├── main.rs          CLI dispatch: --tui → tui::run, else → cat::print
├── config.rs        (existing)
├── font.rs          (existing)
├── theme.rs         (existing)
├── style.rs         (existing; extend Colors with match-highlight slot)
├── render.rs        (existing; add transmit + place + delete_placement)
│
├── layout.rs        [new] pulldown-cmark → RenderedDoc (Vec<Line> +
│                    HeadingImage[] + HeadingEntry[]). Used by both cat
│                    and tui.
├── cat.rs           [new] RenderedDoc → stdout (replaces the stdout
│                    write logic currently in markdown.rs).
├── markdown.rs      (shrinks; event-handling logic migrates into layout.rs)
│
└── tui/
    ├── mod.rs       App struct, terminal setup, main event loop.
    ├── viewport.rs  Wrap cache, visible-line computation, scroll.
    ├── search.rs    SearchState, match list, highlight injection.
    ├── kitty.rs     Image id allocation, placement diff, a=T/a=p/a=d
    │                protocol operations, exit cleanup.
    └── input.rs     Key event → Action mapping.
```

New dependencies: `ratatui`, `crossterm`, `tui-textarea`, `regex` (used
for smart-case literal matching via escape; regex search itself is v2).

## Data Model

Core types live in `layout.rs` and are consumed by both cat and tui:

```rust
pub struct RenderedDoc {
    pub lines: Vec<Line>,
    pub headings: Vec<HeadingEntry>,
    pub images: Vec<HeadingImage>,
}

pub struct Line {
    pub spans: Vec<Span>,
    pub kind: LineKind,
}

pub enum LineKind {
    Body,
    Heading { level: u8, id: usize },
    CodeBlock { lang: Option<String> },
    BlockQuote { depth: u8 },
    ListItem { depth: u8 },
    Table,
    HorizontalRule,
    Blank,
}

pub enum Span {
    Text { content: String, style: Style },
    HeadingImage { id: u32, rows: u16 },
    Link { content: String, url: String, style: Style },
}

pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

pub struct HeadingEntry {
    pub level: u8,
    pub text: String,       // plain-text form, used by search & ToC
    pub line_index: usize,  // index into RenderedDoc.lines
}

pub struct HeadingImage {
    pub id: u32,
    pub png: Vec<u8>,
    pub cols: u16,
    pub rows: u16,
}
```

Key points:

- **Lines are logical (unwrapped).** One Markdown paragraph = one `Line`.
  Wrapping happens in `viewport.rs` against the current terminal width,
  cached, and re-run only on resize.
- **Spans carry structured `Style`, not ANSI strings.** cat converts
  `Style` to ANSI on output; tui converts to `ratatui::style::Style`.
  Search highlight injects a background-color override without parsing
  escapes.
- **`HeadingImage` is stored once and referenced by id.** The tui path
  transmits each PNG to the terminal once at load time and only emits
  placement commands on scroll.
- **`HeadingEntry` is the ToC data source.** No need to rescan `lines`
  to build the outline panel.

### Edge Cases

- **Code blocks**: one `Line { kind: CodeBlock, … }` per source line.
  Search can hit text inside code.
- **Tables**: the existing `markdown.rs` table renderer is lifted into
  `layout.rs`. Each rendered table row becomes a `Line { kind: Table }`.
  Tables do not wrap; over-wide tables truncate with `…`.
- **Image placeholders (non-heading)**: stay as `[image: alt text]` text
  spans — same behavior as cat today.
- **Blank lines**: `LineKind::Blank` so search can skip them.

## Cat Path Rewrite

`markdown.rs` today writes to stdout as it walks the pulldown-cmark
event stream. Under the new design:

1. `layout.rs` owns the event walk and produces a `RenderedDoc`.
2. `cat.rs` turns `RenderedDoc` into ANSI bytes on stdout.
3. The orchestration in `main.rs` stays the same for cat mode
   (termios save/restore around the emit).

This is a real refactor of cat output. Byte-level output may shift
(whitespace, ANSI reset timing). Regression protection:

- Freeze the current cat output for every file in `fixtures/` into
  `fixtures/expected/*.ansi` **before** the refactor lands.
- `make test` runs a snapshot comparison against that frozen baseline.
- Diffs are reviewed intent-first — byte-identical is not required,
  but visible behavior must match.

## Runtime State

```rust
pub struct App {
    docs: Vec<DocEntry>,
    cursor: usize,           // active DocEntry index
    history: Vec<usize>,     // back stack of cursor values
    forward: Vec<usize>,     // forward stack
    mode: Mode,
    term_size: (u16, u16),
    next_image_id: u32,
}

enum Mode {
    Normal,
    Search { query: String, direction: Direction },
    Toc,
    LinkSelect,              // overlay shown after `f`
}

pub struct DocEntry {
    source_path: PathBuf,
    doc: RenderedDoc,
    viewport: Viewport,
    wrap_cache: WrapCache,   // keyed by terminal cols
    search: Option<SearchState>,
    placed_images: HashMap<u32, (u16, u16)>,  // id → current (col, row)
}

pub struct Viewport {
    top_visual_line: usize,
    height: u16,
}
```

**Per-doc state** (`viewport`, `search`, `wrap_cache`, `placed_images`)
is intentional. When the user follows a link from A to B and later
presses `o` (back), A reopens at its previous scroll position with its
search still highlighted, mirroring browser back behavior. Memory cost
is a few tens of KB per doc — negligible.

## Event Loop

```text
loop {
    event = poll(16ms) | tick;
    action = input::map(app.mode, event);
    dirty = apply(&mut app, action);

    if dirty {
        terminal.draw(|f| render_text(f, &app))?;  // ratatui writes text cells
        kitty::sync_images(&mut app)?;             // we diff + place/delete
        writer.flush()?;
    }
}
```

Event polling with a 16ms budget coalesces bursts of held-key repeats
into a single redraw per frame.

### Layered Rendering

ratatui owns the text layer; we own the image layer. Coordination:

1. In `terminal.draw`, every image row is filled with a custom
   "ImageReserve" widget whose `render` is a no-op — ratatui's diff
   engine will *not* touch those cells, so images underneath survive.
2. After ratatui flushes, `kitty::sync_images` walks the visible region
   once, computes the desired `{id → (col, row)}` map, diffs against
   `placed_images`, and emits:
   - `a=d, d=i, i=ID` for ids that left the viewport,
   - `a=p, i=ID, x, y` for ids that entered,
   - `delete + place` for ids whose position changed (Kitty does not
     treat repeated `a=p` of the same id as "move" — it stacks).
3. `placed_images` is updated.

## Key Bindings

| Key | Mode | Action |
|---|---|---|
| `j` / `↓` | Normal | Down 1 line |
| `k` / `↑` | Normal | Up 1 line |
| `d` / `Ctrl-d` | Normal | Down half screen |
| `u` / `Ctrl-u` | Normal | Up half screen |
| `f` / `Ctrl-f` / `Space` / `PageDown` | Normal | Down full screen |
| `b` / `Ctrl-b` / `PageUp` | Normal | Up full screen |
| `g g` | Normal | Jump to top |
| `G` | Normal | Jump to bottom |
| `]]` | Normal | Next heading |
| `[[` | Normal | Previous heading |
| `t` | Normal | Toggle ToC panel |
| `/` | Normal → SearchForward | Open search prompt |
| `?` | Normal → SearchBackward | Reverse search prompt |
| `Enter` | Search | Commit query, jump to first match |
| `Esc` | Search / Toc / LinkSelect | Back to Normal |
| `n` | Normal | Next match |
| `N` | Normal | Previous match |
| `Enter` | Normal | Open link: 0 links visible → nop; 1 → open; >1 → enter LinkSelect |
| digit | LinkSelect | Open numbered link |
| `o` | Normal | Back (previous doc) |
| `i` | Normal | Forward |
| `q` / `Ctrl-c` | Any | Quit |

`o`/`i` repurpose vim's `Ctrl-o`/`Ctrl-i` jump semantics as bare keys —
termdown has no insert mode to conflict with.

### Link Opening

Links follow "open first if unambiguous, otherwise select":

- Viewport contains 0 visible links → Enter is a no-op.
- Viewport contains 1 link → Enter opens it via `open`/`xdg-open`.
- Viewport contains multiple links → Enter enters LinkSelect mode.
  Each visible link gets a bracketed digit overlay (`[1]foo`, `[2]bar`);
  pressing a digit opens that link. Esc exits LinkSelect.

## Search

```rust
pub struct SearchState {
    query: String,
    direction: Direction,
    matches: Vec<MatchPos>,
    current: Option<usize>,
}

pub struct MatchPos {
    line_index: usize,
    byte_range: Range<usize>,
}
```

### Matching (v1)

- Literal substring, smart case (case-insensitive unless the query has
  at least one uppercase letter; same rule as vim with `smartcase`).
- Searches `Span::Text.content`, `Span::Link.content`, and
  `HeadingEntry.text`. Skips `HeadingImage` (image-rendered heading
  text is searchable via the corresponding `HeadingEntry`).
- Full scan once on commit — O(N) over the document. 10k-line docs
  complete in single-digit ms.

### Navigation

- `n` advances `current`; wrap-around at the end shows
  `search hit BOTTOM, continuing at TOP` in the status line.
- `N` reverses.
- Jumping centers the match at ~1/3 from the viewport top (vim default),
  not the exact center — reads better.

### Highlight

Drawing each visible line, if the line's `line_index` has matches, the
corresponding byte ranges have their `Style.bg` overwritten:

- Non-current matches: theme-provided "match" background.
- Current match: theme-provided "current match" background (more vivid).

Colors slot into `style.rs::Colors`, getting auto-resolved for light vs
dark theme like the rest of termdown's palette.

### Edge Cases

- Empty query (press `/` then Enter): no-op.
- Zero matches: status line shows `Pattern not found: <query>`, stay
  in Normal mode, don't clear prior `SearchState`.
- Re-running `/` replaces the query (no nested search).
- If the user `n`s to a match and then scrolls away with `j`,
  `current` is preserved — subsequent `n` continues from `current`,
  not from the current viewport position (vim behavior).

## Kitty Image Lifecycle

Three primitive operations added to `render.rs`:

```rust
fn transmit(id: u32, png: &[u8]);       // a=T, i=ID, f=100, q=2, chunked
fn place(id: u32, col: u16, row: u16);  // a=p, i=ID, x=COL, y=ROW, q=2
fn delete_placement(id: u32);            // a=d, d=i, i=ID, q=2
```

### Timeline

1. **Load doc.** `layout.rs` produces `RenderedDoc` with PNGs in memory.
2. **Register images.** For each `HeadingImage` in the new `DocEntry`,
   call `transmit(id, png)` once. The terminal caches the data.
3. **Event loop.** On each dirty frame, `kitty::sync_images` diffs the
   desired placement set against `placed_images` and emits
   delete/place commands (no PNG data).
4. **Resize.** `sync_images` runs with new cell coordinates — terminal
   scales the image to the new cell size; no re-transmission needed.
5. **Exit.** Send `a=d, d=A` to delete all placements *and free the
   stored image data* this process created, then restore termios.
   (`d=a` would delete placements but leave data cached in the
   terminal — wasteful across repeated opens.)

### Why delete + place instead of re-place

Kitty does not treat a second `a=p` of the same id as "move" — it
stacks a second placement. To move an image, the old placement must be
deleted first. Cost per frame: `O(visible_images)` delete + place
command pairs, each a few dozen bytes. Negligible compared to PNG
re-transmission.

### Exit Cleanup

`a=d, d=A` clears all placements and frees image data this client has
made. Trade-off: if the user has two `termdown --tui` processes sharing
one terminal (e.g. tmux panes), exiting one wipes the other's images.
Not worth the id-range partitioning in v1; if it becomes a real
complaint, switch to id-scoped deletion.

## Testing Strategy

| Layer | Test kind | Coverage |
|---|---|---|
| `layout.rs` | Unit, text snapshot | pulldown-cmark event → `Vec<Line>` correctness per Markdown element |
| `cat.rs` | Snapshot | `RenderedDoc` → ANSI bytes. Frozen `fixtures/expected/*.ansi` baseline before refactor |
| `viewport.rs` | Unit | Wrap on CJK, long URLs (no break), scroll bounds, height changes |
| `tui/search.rs` | Unit | Substring, smart case, n/N wrap, byte-range correctness |
| `tui/kitty.rs` | Unit (mock writer) | Diff algorithm + protocol byte format (`\x1b_G...\x1b\\`) |
| `tui/mod.rs` event loop | Manual | Ghostty, iTerm2 real terminals |

`make check` additions:

- `cargo test` picks up the snapshot tests automatically.
- A new `fixtures/expected/` directory under version control.

### Manual Pre-merge Checklist

Run against both Ghostty and iTerm2:

- Short (< 1 screen), mid (README-size), long (20+ screen) docs.
- Heading-dense docs.
- Mixed-script text, emoji, wide tables, long code blocks.
- Held-`j` for 10 s: no flicker, no lag, no image residue.
- Search hit / miss / wrap / re-center at 1/3.
- Multi-file back/forward, per-doc state preserved.
- Resize mid-session.
- Link open: 0/1/>1 visible cases.
- `q` exit: no image residue on terminal.

## Open Questions (Deferred)

1. Regex search (v2).
2. Code-block syntax highlighting — would add `syntect`; out of scope.
3. Mouse support — deferred to avoid mdfried's selection/scroll trade.
4. TUI reading from stdin — requires pty multiplexing; rejected for v1.
5. Configurable key bindings — hardcoded for v1.
6. Performance SLO: held-`j` on a 100-screen doc should not stutter.
   Instrument only if we miss the target.
7. Font-size changes mid-session (not the same event as resize) —
   v1 requires a reopen.

## Migration Plan

1. Freeze cat output snapshots from current `master`.
2. Introduce `layout.rs` + `cat.rs`; wire `main.rs` to use them for the
   default path. Run snapshot diffs; resolve intent differences.
3. Add `tui/` modules behind the `--tui` flag. Core event loop,
   viewport, input, kitty image sync.
4. Search (v1).
5. Heading nav, ToC panel, status line.
6. Back/forward across multiple docs, link following.
7. Manual pre-merge checklist on both Ghostty and iTerm2.
