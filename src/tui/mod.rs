//! Interactive TUI mode.

mod browser;
mod input;
mod kitty;
mod search;
mod viewport;

use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use browser::FileBrowser;

use crossterm::event::{self, Event};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::text::{Line as RLine, Span as RSpan};
use ratatui::widgets::Paragraph;
use ratatui::Terminal;
use tui_textarea::TextArea;

use crate::config::Config;
use crate::layout;
use crate::theme::Theme;
use crate::tui::search::SearchState;

use viewport::Viewport;

/// Width of the Table-of-Contents side panel when it is open.
const TOC_PANEL_WIDTH: u16 = 30;

/// Width of the File Browser side panel (the filesystem file list).
const BROWSER_PANEL_WIDTH: u16 = 32;

/// How long the cursor must sit still in the File Browser before we dispatch a
/// preview build. Fast scrolling never triggers a build; the preview is only
/// requested once the selection settles. See `feat/file-browser`.
const PREVIEW_DEBOUNCE: Duration = Duration::from_millis(120);

/// A request to the preview worker: build the doc at `path`, tagged with the
/// `generation` that was current when it was dispatched.
struct PreviewRequest {
    generation: u64,
    path: PathBuf,
}

/// A finished preview build coming back from the worker. `doc` is `None` if the
/// file couldn't be read. `generation` lets the main thread discard results
/// whose selection has since moved on.
struct PreviewResponse {
    generation: u64,
    path: PathBuf,
    doc: Option<layout::RenderedDoc>,
}

/// Off-thread Markdown builder for the File Browser. Heading-image
/// rasterization (the dominant per-doc cost) runs here so the event loop never
/// blocks on a settle. The worker coalesces queued requests — if several pile
/// up it only builds the most recent — and the main thread additionally drops
/// any result whose `generation` is stale. The worker exits when the request
/// channel closes (i.e. when `App`/`PreviewWorker` is dropped at TUI teardown).
struct PreviewWorker {
    req_tx: mpsc::Sender<PreviewRequest>,
    res_rx: mpsc::Receiver<PreviewResponse>,
}

impl PreviewWorker {
    fn spawn(config: Config, theme: Theme) -> Self {
        let (req_tx, req_rx) = mpsc::channel::<PreviewRequest>();
        let (res_tx, res_rx) = mpsc::channel::<PreviewResponse>();
        thread::spawn(move || {
            while let Ok(mut req) = req_rx.recv() {
                // Coalesce: if the cursor raced ahead and queued newer
                // requests, skip straight to the latest and drop the rest.
                while let Ok(newer) = req_rx.try_recv() {
                    req = newer;
                }
                let doc = std::fs::read_to_string(&req.path)
                    .ok()
                    .map(|src| layout::build(&src, &config, theme));
                if res_tx
                    .send(PreviewResponse {
                        generation: req.generation,
                        path: req.path,
                        doc,
                    })
                    .is_err()
                {
                    break; // main thread gone
                }
            }
        });
        PreviewWorker { req_tx, res_rx }
    }

    fn request(&self, generation: u64, path: PathBuf) {
        let _ = self.req_tx.send(PreviewRequest { generation, path });
    }

    fn try_recv(&self) -> Option<PreviewResponse> {
        self.res_rx.try_recv().ok()
    }
}

enum Mode {
    Normal,
    Search {
        input: Box<TextArea<'static>>,
        reverse: bool,
    },
    LinkSelect {
        links: Vec<(String, String)>, // (label_content, url)
    },
    Help,
}

/// A single loaded document with its own view state. `App` holds a stack of
/// these so the user can follow local `.md` links and navigate back/forward.
struct DocEntry {
    path: String,
    doc: layout::RenderedDoc,
    viewport: Viewport,
    search: Option<SearchState>,
    pending_g: bool,
    toc_open: bool,
    /// Whether the frontmatter metadata block (if any) is shown expanded as an
    /// inline box. Default `false` = folded one-line summary. Toggled by the
    /// `m` key. Has no effect when `config.metadata` is `Some(false)`.
    metadata_expanded: bool,
}

struct App {
    docs: Vec<DocEntry>,
    cursor: usize,
    history: Vec<usize>,
    forward: Vec<usize>,
    mode: Mode,
    images: kitty::ImageLifecycle,
    /// Global monotonically-increasing image id allocator. Ensures ids stay
    /// unique across all docs loaded during the session so kitty placements
    /// don't collide between back/forward navigations.
    next_image_id: u32,
    /// Body area size (width, height), i.e. terminal size minus the status row.
    /// Remembered so `push_new_doc` can build a correctly-sized `Viewport`.
    term_size: (u16, u16),
    should_quit: bool,
    config: crate::config::Config,
    theme: crate::theme::Theme,
    /// Terminal cell pixel height (reported by the OS). Used to compute real
    /// row counts for heading images. 0 means unknown — callers fall back to
    /// conservative per-level estimates.
    cell_px_height: u32,
    /// Next `event_loop` iteration should force a full-screen clear and
    /// redraw (text + kitty placements). Set whenever state changes in a way
    /// that may leave stale terminal cells behind (scroll, toc toggle, doc
    /// switch, resize).
    needs_full_redraw: bool,
    /// File Browser state, present when termdown was pointed at a directory.
    /// `None` for the plain single-file reader. Survives a commit so the
    /// browser can be re-opened later (HALF 2).
    browser: Option<FileBrowser>,
    /// True while the File Browser panel is showing and holds focus (Browse
    /// mode). False = Read mode (the normal full-screen reader).
    browsing: bool,
    /// The ephemeral preview document for the browser's current selection.
    /// Rebuilt on settle; never enters the `docs` history stack until the user
    /// commits it with Enter.
    preview: Option<DocEntry>,
    /// Background Markdown builder for previews. `None` for the single-file
    /// reader.
    preview_worker: Option<PreviewWorker>,
    /// Monotonic "latest browser interaction" id. Bumped on every cursor move
    /// so an in-flight build whose generation no longer matches is discarded.
    preview_gen: u64,
    /// The generation we have already dispatched a build for, so we don't
    /// re-request the same selection every loop iteration while it builds.
    dispatched_gen: u64,
}

impl App {
    fn new_with_initial_doc(
        path: String,
        doc: layout::RenderedDoc,
        body_height: u16,
        width: u16,
        config: crate::config::Config,
        theme: crate::theme::Theme,
    ) -> Self {
        let mut app = App {
            docs: Vec::new(),
            cursor: 0,
            history: Vec::new(),
            forward: Vec::new(),
            mode: Mode::Normal,
            images: kitty::ImageLifecycle::default(),
            next_image_id: 1,
            term_size: (width, body_height),
            should_quit: false,
            config,
            theme,
            cell_px_height: 0,
            needs_full_redraw: true,
            browser: None,
            browsing: false,
            preview: None,
            preview_worker: None,
            preview_gen: 0,
            dispatched_gen: 0,
        };
        app.push_new_doc(path, doc);
        app
    }

    /// Construct an App that opens directly into the File Browser (Browse
    /// mode) with no committed document yet. The first event-loop iteration
    /// builds the preview for the first file.
    fn new_browser(
        browser: FileBrowser,
        body_height: u16,
        width: u16,
        config: crate::config::Config,
        theme: crate::theme::Theme,
    ) -> Self {
        let worker = PreviewWorker::spawn(config.clone(), theme);
        App {
            docs: Vec::new(),
            cursor: 0,
            history: Vec::new(),
            forward: Vec::new(),
            mode: Mode::Normal,
            images: kitty::ImageLifecycle::default(),
            next_image_id: 1,
            term_size: (width, body_height),
            should_quit: false,
            config,
            theme,
            cell_px_height: 0,
            needs_full_redraw: true,
            browser: Some(browser),
            browsing: true,
            preview: None,
            preview_worker: Some(worker),
            // Start at 1 with dispatched_gen 0 so the first selection is
            // eligible to dispatch on the very first iteration.
            preview_gen: 1,
            dispatched_gen: 0,
        }
    }

    fn active(&self) -> &DocEntry {
        &self.docs[self.cursor]
    }

    fn active_mut(&mut self) -> &mut DocEntry {
        &mut self.docs[self.cursor]
    }

    /// Append a new `DocEntry` and return its index. Re-numbers the doc's
    /// image ids (and any `HeadingImage` / `LineKind::Heading { id }` refs
    /// that point at them) from the global allocator so ids never collide
    /// across docs in a single session.
    fn push_new_doc(&mut self, path: String, mut doc: layout::RenderedDoc) -> usize {
        renumber_doc_ids(&mut doc, &mut self.next_image_id);
        let (width, height) = self.term_size;
        let viewport = Viewport::new(height, width);
        let mut entry = DocEntry {
            path,
            doc,
            viewport,
            search: None,
            pending_g: false,
            toc_open: false,
            metadata_expanded: false,
        };
        // Refine image row estimates now that (a) the doc is populated and
        // (b) we may already know the real terminal cell pixel height.
        refine_image_rows(&mut entry.doc, self.cell_px_height);
        self.docs.push(entry);
        self.docs.len() - 1
    }

    /// Transmit the active doc's images to the terminal. Idempotent for repeats
    /// (kitty drops re-registration of the same id silently; `ImageLifecycle`
    /// also tracks already-registered ids).
    fn register_active_images<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        // Clone out the (id, png) pairs first to avoid aliasing &self.docs while
        // we also want &mut self.images.
        let doc_images: Vec<(u32, Vec<u8>)> = self
            .active()
            .doc
            .images
            .iter()
            .map(|i| (i.id, i.png.clone()))
            .collect();
        for (id, png) in &doc_images {
            self.images.register(w, *id, png)?;
        }
        Ok(())
    }

    /// Transmit the current preview doc's images (Browse mode). No-op when no
    /// preview is built yet.
    fn register_preview_images<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        let doc_images: Vec<(u32, Vec<u8>)> = match &self.preview {
            Some(p) => p.doc.images.iter().map(|i| (i.id, i.png.clone())).collect(),
            None => return Ok(()),
        };
        for (id, png) in &doc_images {
            self.images.register(w, *id, png)?;
        }
        Ok(())
    }

    /// Open a link target. If it's a local `.md` file, pushes a new DocEntry
    /// onto the history stack and makes it active. Otherwise, spawns the
    /// platform URL handler.
    fn open_link_target(&mut self, target: &str) {
        if looks_like_local_md(target) {
            // Resolve relative to the active doc's path.
            let base = std::path::Path::new(&self.active().path);
            let base_dir = base.parent().unwrap_or_else(|| std::path::Path::new("."));
            let resolved = base_dir.join(target);
            if resolved.is_file() {
                match std::fs::read_to_string(&resolved) {
                    Ok(src) => {
                        let new_doc = layout::build(&src, &self.config, self.theme);
                        let new_path = resolved.display().to_string();
                        let new_cursor = self.push_new_doc(new_path, new_doc);
                        self.history.push(self.cursor);
                        self.forward.clear();
                        self.cursor = new_cursor;
                        let mut out = std::io::stdout().lock();
                        let _ = self.register_active_images(&mut out);
                        let _ = std::io::Write::flush(&mut out);
                        self.needs_full_redraw = true;
                    }
                    Err(_) => {
                        // Fall back to opening externally.
                        spawn_open(target);
                    }
                }
                return;
            }
        }
        spawn_open(target);
    }
}

pub fn run(path: &str, config: &Config, theme: Theme) {
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("termdown: error reading {path}: {e}");
            std::process::exit(1);
        }
    };
    let doc = layout::build(&source, config, theme);

    if let Err(e) = run_ui(doc, path.to_string(), config.clone(), theme) {
        eprintln!("termdown: tui error: {e}");
        std::process::exit(1);
    }
}

/// Entry point for `termdown <dir>` — open the File Browser on a directory.
pub fn run_browser(dir: &str, config: &Config, theme: Theme) {
    let browser = match FileBrowser::scan(std::path::Path::new(dir)) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("termdown: error reading directory {dir}: {e}");
            std::process::exit(1);
        }
    };
    if browser.entries.is_empty() {
        eprintln!("termdown: no Markdown files found in {dir}");
        std::process::exit(1);
    }
    if let Err(e) = run_browser_ui(browser, config.clone(), theme) {
        eprintln!("termdown: tui error: {e}");
        std::process::exit(1);
    }
}

fn run_browser_ui(browser: FileBrowser, config: Config, theme: Theme) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let body_height = size.height.saturating_sub(1);
    let mut app = App::new_browser(browser, body_height, size.width, config, theme);
    app.cell_px_height = query_cell_px_height();

    let result = event_loop(&mut terminal, &mut app);

    {
        let mut out = io::stdout().lock();
        let _ = app.images.cleanup(&mut out);
        let _ = out.flush();
    }

    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

/// Shift all heading-image ids in `doc` so they start at `*next_image_id`,
/// then advance the allocator past them. Patches both `Span::HeadingImage` and
/// `LineKind::Heading { id }` references. Shared by the history stack
/// (`push_new_doc`) and the ephemeral browser preview so ids never collide
/// across docs in one session.
fn renumber_doc_ids(doc: &mut layout::RenderedDoc, next_image_id: &mut u32) {
    let offset = *next_image_id;
    // layout::build() assigns ids starting at 1; shift each by (offset - 1)
    // so the first image of this doc becomes `offset`.
    let mut id_map: HashMap<u32, u32> = HashMap::new();
    for img in &mut doc.images {
        let new_id = offset + (img.id - 1);
        id_map.insert(img.id, new_id);
        img.id = new_id;
    }
    if let Some(max) = doc.images.iter().map(|i| i.id).max() {
        *next_image_id = max + 1;
    }
    for line in &mut doc.lines {
        for span in &mut line.spans {
            if let layout::Span::HeadingImage { id, .. } = span {
                if let Some(&new) = id_map.get(id) {
                    *id = new;
                }
            }
        }
        if let layout::LineKind::Heading { id: Some(hid), .. } = &mut line.kind {
            if let Some(&new) = id_map.get(hid) {
                *hid = new;
            }
        }
    }
}

/// True when the highlighted file differs from the one the current preview was
/// built for — i.e. a build is pending/in-flight and the pane should show a
/// loading indicator rather than stale content or images.
fn browse_selection_pending(app: &App) -> bool {
    match app.browser.as_ref() {
        Some(b) => {
            let sel = b.selected();
            sel.is_some() && sel != b.preview_path.as_ref()
        }
        None => false,
    }
}

/// Adopt a finished preview build from the worker: renumber ids onto the global
/// allocator, refine image rows to the real cell height, transmit the PNGs, and
/// mark this path as the one currently shown. A read error (`doc == None`)
/// clears the preview but still records the path so the pane shows the
/// "unreadable" message instead of a perpetual spinner.
fn accept_preview(app: &mut App, resp: PreviewResponse) {
    let (term_width, body_height) = app.term_size;
    let body_width = term_width.saturating_sub(BROWSER_PANEL_WIDTH);

    // Reap the outgoing preview's image data first — it's ephemeral (never
    // committed; commit takes `app.preview` via `take()` so we never reach here
    // for a kept doc) and would otherwise stay cached in the terminal.
    let stale_ids: Vec<u32> = app
        .preview
        .as_ref()
        .map(|p| p.doc.images.iter().map(|i| i.id).collect())
        .unwrap_or_default();
    if !stale_ids.is_empty() {
        let mut out = io::stdout().lock();
        let _ = app.images.forget(&mut out, &stale_ids);
        let _ = out.flush();
    }

    match resp.doc {
        Some(mut doc) => {
            renumber_doc_ids(&mut doc, &mut app.next_image_id);
            refine_image_rows(&mut doc, app.cell_px_height);
            app.preview = Some(DocEntry {
                path: resp.path.display().to_string(),
                doc,
                viewport: Viewport::new(body_height, body_width),
                search: None,
                pending_g: false,
                toc_open: false,
                metadata_expanded: false,
            });
            let mut out = io::stdout().lock();
            let _ = app.register_preview_images(&mut out);
            let _ = out.flush();
        }
        None => {
            app.preview = None;
        }
    }

    if let Some(b) = app.browser.as_mut() {
        b.preview_path = Some(resp.path);
    }
}

/// Synchronously build a `DocEntry` for `path` (used on commit when the async
/// preview isn't ready yet — a one-off blocking build on a deliberate Enter is
/// acceptable). Returns `None` if the file can't be read.
fn build_doc_entry_sync(app: &mut App, path: &std::path::Path) -> Option<DocEntry> {
    let (term_width, body_height) = app.term_size;
    let body_width = term_width.saturating_sub(BROWSER_PANEL_WIDTH);
    let src = std::fs::read_to_string(path).ok()?;
    let mut doc = layout::build(&src, &app.config, app.theme);
    renumber_doc_ids(&mut doc, &mut app.next_image_id);
    refine_image_rows(&mut doc, app.cell_px_height);
    Some(DocEntry {
        path: path.display().to_string(),
        doc,
        viewport: Viewport::new(body_height, body_width),
        search: None,
        pending_g: false,
        toc_open: false,
        metadata_expanded: false,
    })
}

fn run_ui(doc: layout::RenderedDoc, path: String, config: Config, theme: Theme) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let body_height = size.height.saturating_sub(1);
    let mut app = App::new_with_initial_doc(path, doc, body_height, size.width, config, theme);
    // Query the real terminal cell pixel height up-front so heading image
    // `rows` estimates are accurate from the first frame onward. If the OS
    // doesn't report it, `cell_px_height` stays 0 and we keep per-level
    // fallbacks from layout.
    app.cell_px_height = query_cell_px_height();
    if app.cell_px_height > 0 {
        // Refine the already-pushed initial doc now that we know the real
        // cell pixel height (push_new_doc ran with `cell_px_height = 0`).
        let h = app.cell_px_height;
        refine_image_rows(&mut app.docs[0].doc, h);
    }

    // Transmit all heading PNGs once; subsequent frames only emit placement commands.
    {
        let mut out = io::stdout().lock();
        app.register_active_images(&mut out)?;
        out.flush()?;
    }

    let result = event_loop(&mut terminal, &mut app);

    {
        let mut out = io::stdout().lock();
        let _ = app.images.cleanup(&mut out);
        let _ = out.flush();
    }

    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn event_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        // Sync viewport dimensions to the current terminal size. Handles
        // initial startup and terminal resizes. Any change invalidates the
        // wrap cache (`ensure_wrap` re-wraps when `self.width != cache_width`).
        let size = terminal.size()?;
        let body_height = size.height.saturating_sub(1);
        app.term_size = (size.width, body_height);
        let show_metadata = app.config.metadata.unwrap_or(true);

        if app.browsing {
            // 1. Adopt any finished builds, discarding ones the cursor has
            //    since moved past (stale generation).
            let mut accepted: Vec<PreviewResponse> = Vec::new();
            if let Some(worker) = app.preview_worker.as_ref() {
                while let Some(resp) = worker.try_recv() {
                    if resp.generation == app.preview_gen {
                        accepted.push(resp);
                    }
                }
            }
            // Only the newest accepted result matters; drop the rest.
            if let Some(resp) = accepted.pop() {
                accept_preview(app, resp);
            }

            // 2. Dispatch a build once the selection settles (debounced) and we
            //    haven't already requested this generation.
            let (sel, settled) = match app.browser.as_ref() {
                Some(b) => (
                    b.selected().cloned(),
                    b.last_move.is_none_or(|t| t.elapsed() >= PREVIEW_DEBOUNCE),
                ),
                None => (None, false),
            };
            if settled && app.dispatched_gen != app.preview_gen && browse_selection_pending(app) {
                // Mark this generation dispatched before borrowing the worker so
                // we never re-request it; one build per settle.
                let gen = app.preview_gen;
                app.dispatched_gen = gen;
                if let (Some(worker), Some(path)) = (app.preview_worker.as_ref(), sel) {
                    worker.request(gen, path);
                }
            }

            // 3. Size the preview viewport (only meaningful when one is shown).
            let body_width = size.width.saturating_sub(BROWSER_PANEL_WIDTH);
            if let Some(p) = app.preview.as_mut() {
                p.viewport.width = body_width;
                p.viewport.height = body_height;
                let expanded = p.metadata_expanded;
                p.viewport.ensure_wrap(&p.doc, show_metadata, expanded);
            }
        } else {
            let body_width = if app.active().toc_open {
                size.width.saturating_sub(TOC_PANEL_WIDTH)
            } else {
                size.width
            };
            let active = app.active_mut();
            if active.viewport.width != body_width || active.viewport.height != body_height {
                active.viewport.width = body_width;
                active.viewport.height = body_height;
                // width change implicitly invalidates wrap via ensure_wrap's
                // cache_width comparison.
            }
            let expanded = active.metadata_expanded;
            active
                .viewport
                .ensure_wrap(&active.doc, show_metadata, expanded);
        }

        // Force a full redraw if state changed in a way that may leave
        // stale cells behind. Clears text cells, deletes all kitty
        // placements (keeping cached image data), and resets our
        // placement tracking so `sync` re-emits every visible image.
        if app.needs_full_redraw {
            terminal.clear()?;
            // Ghostty evicts cached kitty image data on `\x1b[2J`, so a
            // subsequent `a=p,i=X` references an unknown id and the
            // placement is silently dropped — leaving blank space where the
            // heading image should appear. Forget the transmitted set and
            // re-upload every active image after the clear.
            app.images.reset_transmissions();
            let mut out = io::stdout().lock();
            let _ = app.images.reset_placements(&mut out);
            if app.browsing {
                let _ = app.register_preview_images(&mut out);
            } else {
                let _ = app.register_active_images(&mut out);
            }
            let _ = out.flush();
            app.needs_full_redraw = false;
        }

        if app.browsing {
            terminal.draw(|frame| draw_browse(frame, app))?;
        } else {
            terminal.draw(|frame| draw(frame, app))?;
        }

        // While a preview is still loading (selection differs from the shown
        // doc) we place no images — the pane shows a spinner, not stale art.
        let browse_pending = app.browsing && browse_selection_pending(app);

        {
            let mut stdout = io::stdout().lock();
            let desired = if app.browsing {
                match (browse_pending, app.preview.as_ref()) {
                    (false, Some(p)) => {
                        placements_for(p, BROWSER_PANEL_WIDTH, p.viewport.height, None)
                    }
                    _ => HashMap::new(),
                }
            } else {
                desired_image_placements(app)
            };
            let _ = app.images.sync(&mut stdout, &desired);
            let _ = stdout.flush();
        }

        // Poll faster while a build is in flight so the finished preview pops in
        // promptly; idle otherwise to keep CPU low.
        let poll_ms = if browse_pending { 16 } else { 50 };
        if event::poll(Duration::from_millis(poll_ms))? {
            let ev = event::read()?;
            // Resize is the one event crossterm surfaces that must trigger a
            // full redraw regardless of mode.
            if matches!(ev, Event::Resize(_, _)) {
                app.needs_full_redraw = true;
                continue;
            }
            if app.browsing {
                handle_browse_key(app, &ev);
            } else {
                match &mut app.mode {
                    Mode::Normal => handle_normal_key(app, &ev)?,
                    Mode::Search { .. } => handle_search_key(app, ev)?,
                    Mode::LinkSelect { .. } => handle_link_select_key(app, ev)?,
                    Mode::Help => handle_help_key(app, ev)?,
                }
            }
            if app.should_quit {
                return Ok(());
            }
            // Scroll / mode-change / search events rely on ratatui's cell
            // diff + `images.sync()` for correctness — no full clear. Only
            // the handlers that actually need a clear (resize, toc toggle,
            // doc switch) set `needs_full_redraw` themselves. A blanket
            // full-clear here produces visible flicker at key-autorepeat
            // rates (~30 Hz) because each frame emits `\x1b[2J` + re-uploads
            // every heading PNG.
        }
    }
}

/// Key handling while the File Browser holds focus (Browse mode).
fn handle_browse_key(app: &mut App, ev: &Event) {
    let Event::Key(key) = ev else {
        return;
    };
    if key.kind != event::KeyEventKind::Press {
        return;
    }
    let ctrl = key.modifiers.contains(event::KeyModifiers::CONTROL);
    match key.code {
        event::KeyCode::Char('q') => app.should_quit = true,
        event::KeyCode::Char('c') if ctrl => app.should_quit = true,
        event::KeyCode::Char('j') | event::KeyCode::Down => browse_move(app, 1),
        event::KeyCode::Char('k') | event::KeyCode::Up => browse_move(app, -1),
        event::KeyCode::Enter => browse_commit(app),
        event::KeyCode::Esc => {
            // No committed doc to return to (launched straight into the
            // browser) → quit; otherwise drop back to the reader.
            if app.docs.is_empty() {
                app.should_quit = true;
            } else {
                app.browsing = false;
                app.needs_full_redraw = true;
            }
        }
        _ => {}
    }
}

/// Move the browser cursor by `delta`, clamped to the list bounds. Records the
/// move time (for debounce) and bumps the generation so any in-flight build for
/// the previous selection is discarded when it returns.
fn browse_move(app: &mut App, delta: i32) {
    let moved = if let Some(b) = app.browser.as_mut() {
        if b.entries.is_empty() {
            false
        } else {
            let len = b.entries.len() as i32;
            let next = (b.cursor as i32 + delta).clamp(0, len - 1) as usize;
            if next != b.cursor {
                b.cursor = next;
                b.last_move = Some(Instant::now());
                true
            } else {
                false
            }
        }
    } else {
        false
    };
    if moved {
        app.preview_gen += 1;
    }
}

/// Commit the selected file: reuse its preview if the async build already
/// landed, otherwise build it synchronously now (a one-off blocking build on a
/// deliberate Enter is acceptable). Then move it onto the history stack as the
/// active document and leave Browse mode.
fn browse_commit(app: &mut App) {
    let Some(sel) = app.browser.as_ref().and_then(|b| b.selected().cloned()) else {
        return;
    };
    let preview_ready = !browse_selection_pending(app) && app.preview.is_some();
    let entry = if preview_ready {
        app.preview.take().expect("preview_ready implies Some")
    } else {
        match build_doc_entry_sync(app, &sel) {
            Some(e) => e,
            None => return, // unreadable file — stay in the browser
        }
    };
    if !app.docs.is_empty() {
        app.history.push(app.cursor);
    }
    app.forward.clear();
    app.docs.push(entry);
    app.cursor = app.docs.len() - 1;
    app.browsing = false;
    // Forget the preview tracking so re-opening the browser rebuilds it.
    if let Some(b) = app.browser.as_mut() {
        b.preview_path = None;
    }
    app.needs_full_redraw = true;
}

/// Apply a scroll delta and ring the edge bell if the viewport didn't budge.
/// Detection lives here (not in `Viewport`) so the data layer stays free of
/// `App`/`Config`/audio coupling and `gg`/`G`/`]`/`[` — which bypass this
/// helper — silently stay non-belling, matching the chosen scope.
fn perform_scroll(app: &mut App, delta: i32) {
    if delta == 0 {
        return;
    }
    let before = app.active().viewport.top;
    app.active_mut().viewport.scroll_by(delta);
    if app.active().viewport.top == before {
        ring_bell(&app.config);
    }
}

/// Emit a terminal BEL on blocked edge-scroll. No-op when the user has
/// disabled bells via config or `--no-bell`. Writes to stderr (which is
/// unbuffered, so no manual flush) so the byte does not enter the
/// alternate-screen buffer. The visible "🔔 in the title bar" effect is the
/// terminal emulator's own response to BEL (e.g. Ghostty's `bell-features`
/// defaults include `title`), not something termdown paints.
fn ring_bell(config: &Config) {
    if !config.bell.unwrap_or(true) {
        return;
    }
    let _ = io::stderr().write_all(b"\x07");
}

fn handle_normal_key(app: &mut App, ev: &Event) -> io::Result<()> {
    if let Event::Key(key) = ev {
        if key.kind != event::KeyEventKind::Press {
            return Ok(());
        }
        // gg intercept
        if key.code == event::KeyCode::Char('g')
            && !key.modifiers.contains(event::KeyModifiers::CONTROL)
        {
            let active = app.active_mut();
            if active.pending_g {
                active.viewport.top = 0;
                active.pending_g = false;
            } else {
                active.pending_g = true;
            }
            return Ok(());
        }
        app.active_mut().pending_g = false;

        match input::map_normal(*key) {
            input::Action::Quit => {
                app.should_quit = true;
            }
            input::Action::ScrollLines(d) => perform_scroll(app, d),
            input::Action::ScrollHalfPage(s) => {
                let delta = (app.active().viewport.height as i32 / 2) * s;
                perform_scroll(app, delta);
            }
            input::Action::ScrollPage(s) => {
                let delta = app.active().viewport.height as i32 * s;
                perform_scroll(app, delta);
            }
            input::Action::JumpStart => app.active_mut().viewport.top = 0,
            input::Action::JumpEnd => {
                let active = app.active_mut();
                let max_top = active
                    .viewport
                    .total_visual_lines()
                    .saturating_sub(active.viewport.height as usize);
                active.viewport.top = max_top;
            }
            input::Action::NextHeading => {
                let active = app.active_mut();
                let top = active.viewport.top;
                active.viewport.jump_to_next_heading(&active.doc, top);
            }
            input::Action::PrevHeading => {
                let active = app.active_mut();
                let top = active.viewport.top;
                active.viewport.jump_to_prev_heading(&active.doc, top);
            }
            input::Action::SearchBegin { reverse } => {
                let mut ta = TextArea::default();
                ta.set_cursor_line_style(ratatui::style::Style::default());
                app.mode = Mode::Search {
                    input: Box::new(ta),
                    reverse,
                };
            }
            input::Action::SearchNext => advance_search(app, 1),
            input::Action::SearchPrev => advance_search(app, -1),
            input::Action::ToggleToc => {
                let active = app.active_mut();
                active.toc_open = !active.toc_open;
                // viewport.width is re-synced from terminal size at the top
                // of every event_loop iteration, so we don't need to adjust
                // it here — the next iteration picks up the new body width
                // and `ensure_wrap` re-wraps once cache_width drifts. Width
                // change shifts every image's col offset, so force a full
                // clear to avoid stale image pixels on the body side.
                app.needs_full_redraw = true;
            }
            // No-op when metadata display is disabled — toggling would only
            // churn the wrap cache and force a redraw with no visible change.
            input::Action::ToggleMetadata if app.config.metadata.unwrap_or(true) => {
                let active = app.active_mut();
                if active.doc.metadata.is_some() {
                    active.metadata_expanded = !active.metadata_expanded;
                    active.viewport.invalidate_wrap();
                    app.needs_full_redraw = true;
                }
            }
            input::Action::Back => {
                if let Some(prev) = app.history.pop() {
                    app.forward.push(app.cursor);
                    app.cursor = prev;
                    let mut out = io::stdout().lock();
                    let _ = app.register_active_images(&mut out);
                    let _ = out.flush();
                    app.needs_full_redraw = true;
                }
            }
            input::Action::Forward => {
                if let Some(next) = app.forward.pop() {
                    app.history.push(app.cursor);
                    app.cursor = next;
                    let mut out = io::stdout().lock();
                    let _ = app.register_active_images(&mut out);
                    let _ = out.flush();
                    app.needs_full_redraw = true;
                }
            }
            input::Action::OpenLink => {
                let links = visible_links(app);
                match links.len() {
                    0 => {}
                    1 => {
                        let url = links[0].1.clone();
                        app.open_link_target(&url);
                    }
                    _ => {
                        app.mode = Mode::LinkSelect { links };
                    }
                }
            }
            input::Action::OpenHelp => {
                app.mode = Mode::Help;
                app.needs_full_redraw = true;
            }
            // Other actions land in later tasks. No-op for now.
            _ => {}
        }
    }
    Ok(())
}

fn handle_help_key(app: &mut App, ev: Event) -> io::Result<()> {
    let Event::Key(key) = ev else {
        return Ok(());
    };
    if key.kind != event::KeyEventKind::Press {
        return Ok(());
    }
    match key.code {
        event::KeyCode::Char('?') | event::KeyCode::Esc | event::KeyCode::Char('q') => {
            app.mode = Mode::Normal;
            app.needs_full_redraw = true;
        }
        event::KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        _ => {}
    }
    Ok(())
}

fn handle_search_key(app: &mut App, ev: Event) -> io::Result<()> {
    let Mode::Search { input, reverse } = &mut app.mode else {
        return Ok(());
    };
    let reverse = *reverse;
    let Event::Key(key) = ev else {
        return Ok(());
    };
    if key.kind != event::KeyEventKind::Press {
        return Ok(());
    }
    match key.code {
        event::KeyCode::Esc => {
            app.mode = Mode::Normal;
        }
        event::KeyCode::Enter => {
            let query: String = input.lines().join("");
            app.mode = Mode::Normal;
            let state = SearchState::new(&query, &app.active().doc);
            app.active_mut().search = Some(state);
            apply_search_jump(app, reverse);
        }
        _ => {
            input.input(key);
        }
    }
    Ok(())
}

fn handle_link_select_key(app: &mut App, ev: Event) -> io::Result<()> {
    let Mode::LinkSelect { links } = &app.mode else {
        return Ok(());
    };
    // Clone out to avoid aliasing app.mode mutation below.
    let links = links.clone();

    let Event::Key(key) = ev else {
        return Ok(());
    };
    if key.kind != event::KeyEventKind::Press {
        return Ok(());
    }
    match key.code {
        event::KeyCode::Esc => {
            app.mode = Mode::Normal;
        }
        event::KeyCode::Char(c) if c.is_ascii_digit() => {
            let idx = (c as u8 - b'0') as usize;
            if idx > 0 && idx <= links.len() {
                let (_, url) = &links[idx - 1];
                let url = url.clone();
                app.mode = Mode::Normal;
                app.open_link_target(&url);
            }
        }
        _ => {}
    }
    Ok(())
}

/// Collect all `Span::Link` entries whose visual row is in the viewport.
/// Returns `(content, url)` tuples in document order.
/// Deduplicates by logical line index so wrapped lines don't produce
/// duplicate entries.
fn visible_links(app: &App) -> Vec<(String, String)> {
    let active = app.active();
    let mut seen_logical: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut out = Vec::new();
    for vl in active.viewport.visible() {
        if !seen_logical.insert(vl.logical_index) {
            continue;
        }
        let logical = &active.doc.lines[vl.logical_index];
        for span in &logical.spans {
            if let layout::Span::Link { content, url, .. } = span {
                out.push((content.clone(), url.clone()));
            }
        }
    }
    out
}

fn looks_like_local_md(target: &str) -> bool {
    if target.contains("://") {
        return false;
    }
    let lower = target.to_ascii_lowercase();
    lower.ends_with(".md") || lower.ends_with(".markdown")
}

/// Visual style for a link span. Local `.md` links (followed in-TUI) and
/// external links (delegated to the OS opener) render differently so the user
/// can tell them apart before pressing Enter.
fn link_style_for(url: &str, theme: Theme) -> ratatui::style::Style {
    use ratatui::style::{Color, Modifier, Style};
    let fg = match theme {
        Theme::Dark => Color::Cyan,
        Theme::Light => Color::Blue,
    };
    let mut style = Style::default().fg(fg).add_modifier(Modifier::UNDERLINED);
    if !looks_like_local_md(url) {
        style = style.add_modifier(Modifier::ITALIC);
    }
    style
}

fn spawn_open(url: &str) {
    let cmd = if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "xdg-open"
    };
    if cmd == "cmd" {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn();
    } else {
        let _ = std::process::Command::new(cmd).arg(url).spawn();
    }
}

fn apply_search_jump(app: &mut App, reverse: bool) {
    let active = app.active_mut();
    let Some(state) = active.search.as_mut() else {
        return;
    };
    if state.matches.is_empty() {
        state.current = None;
        return;
    }
    let current_logical = active
        .viewport
        .visible()
        .first()
        .map(|vl| vl.logical_index)
        .unwrap_or(0);
    let idx = if !reverse {
        state
            .matches
            .iter()
            .position(|m| m.line_index >= current_logical)
            .unwrap_or(0)
    } else {
        state
            .matches
            .iter()
            .rposition(|m| m.line_index <= current_logical)
            .unwrap_or(state.matches.len() - 1)
    };
    state.current = Some(idx);
    let line = state.matches[idx].line_index;
    center_on_logical(&mut active.viewport, line);
}

fn advance_search(app: &mut App, delta: i32) {
    let active = app.active_mut();
    let Some(state) = active.search.as_mut() else {
        return;
    };
    if state.matches.is_empty() {
        return;
    }
    let len = state.matches.len() as i32;
    let cur = state.current.unwrap_or(0) as i32;
    let next = ((cur + delta) % len + len) % len;
    state.current = Some(next as usize);
    let line = state.matches[next as usize].line_index;
    center_on_logical(&mut active.viewport, line);
}

fn center_on_logical(vp: &mut Viewport, logical: usize) {
    if let Some(vi) = vp.visual_line_for_logical(logical) {
        let third = (vp.height as usize) / 3;
        let new_top = vi.saturating_sub(third);
        let max_top = vp.total_visual_lines().saturating_sub(vp.height as usize);
        vp.top = new_top.min(max_top);
    }
}

struct VisibleMatch {
    start: usize,
    end: usize,
    is_current: bool,
}

/// Collect matches overlapping `[byte_start, byte_end)` on line `logical_index`.
/// Returns ranges in the *logical* line's byte coordinates (same space the
/// VisualLine uses). `current_logical` is `(line_index, byte_range.start)` of
/// the currently-focused match, if any.
fn visible_matches_for_line(
    search: Option<&SearchState>,
    logical_index: usize,
    byte_start: usize,
    byte_end: usize,
    current_logical: Option<(usize, usize)>,
) -> Vec<VisibleMatch> {
    let Some(state) = search else {
        return Vec::new();
    };
    state
        .matches
        .iter()
        .filter(|m| m.line_index == logical_index)
        .filter(|m| m.byte_range.start < byte_end && m.byte_range.end > byte_start)
        .map(|m| VisibleMatch {
            start: m.byte_range.start,
            end: m.byte_range.end,
            is_current: Some((m.line_index, m.byte_range.start)) == current_logical,
        })
        .collect()
}

/// Render one VisualLine row that visualizes the document's frontmatter.
/// The role determines what shows up: folded summary, expanded top/bottom
/// border, or a single field row inside the expanded box.
fn render_metadata_row(
    meta: &crate::frontmatter::MetadataInfo,
    role: viewport::MetadataVisualRow,
    body_cols: usize,
) -> RLine<'static> {
    use ratatui::style::{Modifier, Style as RStyle};

    let dim = RStyle::default().add_modifier(Modifier::DIM);

    match role {
        viewport::MetadataVisualRow::Folded => {
            // Identical construction/truncation as `--cat` — shared so the two
            // folded renderings can never drift apart.
            let text = crate::frontmatter::folded_summary(meta, body_cols);
            RLine::from(RSpan::styled(text, dim))
        }
        viewport::MetadataVisualRow::ExpandedTop => {
            // Width follows the body area, capped so the box doesn't dominate
            // narrow terminals; minimum 12 for a sensible visual.
            let inner_w = expanded_box_width(body_cols);
            let title = " metadata ";
            let mut s = String::from("┌─");
            s.push_str(title);
            let remaining = inner_w.saturating_sub(s.chars().count() + 1).max(1);
            s.push_str(&"─".repeat(remaining));
            s.push('┐');
            RLine::from(RSpan::styled(s, dim))
        }
        viewport::MetadataVisualRow::ExpandedField(idx) => {
            use unicode_width::UnicodeWidthStr;
            let inner_w = expanded_box_width(body_cols);
            // 2 border chars + 1 leading space + 1 trailing space = 4 chrome.
            let field_budget = inner_w.saturating_sub(4);
            let (k_text, v_text) = if meta.has_pairs() {
                let (k, v) = &meta.pairs[idx];
                (k.clone(), v.clone())
            } else {
                ("metadata".to_string(), meta.fallback_oneline.clone())
            };
            // All widths are display columns (not char counts) so CJK / wide
            // characters in keys or values keep the box border aligned.
            // Right-pad the key column to the widest key (capped) so values align.
            let key_col = meta
                .pairs
                .iter()
                .map(|(k, _)| UnicodeWidthStr::width(k.as_str()))
                .max()
                .unwrap_or(0)
                .min(field_budget.saturating_sub(3));
            let key_pad = key_col.saturating_sub(UnicodeWidthStr::width(k_text.as_str()));
            let line_body = format!("{}{}: ", k_text, " ".repeat(key_pad));
            let val_budget =
                field_budget.saturating_sub(UnicodeWidthStr::width(line_body.as_str()));
            let value = truncate_to_cols(&v_text, val_budget);
            let inside = format!("{line_body}{value}");
            let pad = field_budget.saturating_sub(UnicodeWidthStr::width(inside.as_str()));
            let row = format!("│ {inside}{} │", " ".repeat(pad));
            RLine::from(RSpan::styled(row, dim))
        }
        viewport::MetadataVisualRow::ExpandedBottom => {
            let inner_w = expanded_box_width(body_cols);
            let mut s = String::from("└");
            s.push_str(&"─".repeat(inner_w.saturating_sub(2)));
            s.push('┘');
            RLine::from(RSpan::styled(s, dim))
        }
    }
}

fn expanded_box_width(body_cols: usize) -> usize {
    body_cols.clamp(12, 80)
}

fn truncate_to_cols(s: &str, max_cols: usize) -> String {
    use unicode_width::UnicodeWidthChar;
    if max_cols == 0 {
        return String::new();
    }
    let mut width = 0;
    let mut acc = String::new();
    for ch in s.chars() {
        let cw = UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + cw > max_cols.saturating_sub(1) {
            // Reserve 1 col for the ellipsis.
            acc.push('…');
            return acc;
        }
        acc.push(ch);
        width += cw;
    }
    acc
}

fn clipped_spans(
    line: &layout::Line,
    byte_start: usize,
    byte_end: usize,
    matches: &[VisibleMatch],
    theme: Theme,
) -> Vec<RSpan<'static>> {
    use ratatui::style::{Color as RColor, Modifier, Style as RStyle};

    let mut out: Vec<RSpan<'static>> = Vec::new();
    let mut cursor = 0usize;

    // Highlight styles. Task 8/theme follow-up: pull from style::Colors instead.
    let current_style = RStyle::default().bg(RColor::Yellow).fg(RColor::Black);
    let other_style = RStyle::default()
        .bg(RColor::Rgb(80, 80, 0))
        .fg(RColor::White);

    for span in &line.spans {
        let (content, is_image, link_base) = match span {
            layout::Span::Text { content, .. } => (content.as_str(), false, None),
            layout::Span::Link { content, url, .. } => {
                (content.as_str(), false, Some(link_style_for(url, theme)))
            }
            layout::Span::HeadingImage { .. } => ("", true, None),
        };
        if is_image {
            continue;
        }
        let span_start = cursor;
        let span_end = cursor + content.len();
        cursor = span_end;

        let clip_start = span_start.max(byte_start);
        let clip_end = span_end.min(byte_end);
        if clip_start >= clip_end {
            continue;
        }

        // Walk through the visible slice [clip_start, clip_end) emitting
        // alternating plain/highlighted sub-strings.
        let mut pos = clip_start;
        while pos < clip_end {
            // Find the next match range that contains `pos` or starts after it.
            let next_match = matches
                .iter()
                .filter(|m| m.end > pos && m.start < clip_end)
                .min_by_key(|m| m.start);

            let (region_end, style) = match next_match {
                Some(m) if m.start <= pos => {
                    // Currently inside a match.
                    let region_end = m.end.min(clip_end);
                    let style = if m.is_current {
                        current_style
                    } else {
                        other_style
                    };
                    (region_end, Some(style))
                }
                Some(m) => {
                    // There's a match further ahead; emit plain text up to its start.
                    (m.start.min(clip_end), None)
                }
                None => {
                    // No more matches — emit plain text to clip_end.
                    (clip_end, None)
                }
            };

            let slice_start = pos - span_start;
            let slice_end = region_end - span_start;
            if !content.is_char_boundary(slice_start) || !content.is_char_boundary(slice_end) {
                break; // defensive — shouldn't happen since wrap breaks at char boundaries
            }
            let slice = &content[slice_start..slice_end];
            let resolved = match style {
                Some(st) => {
                    // Search highlight wins on fg/bg, but keep the link
                    // underline so it stays identifiable under the highlight.
                    let st = if link_base.is_some() {
                        st.add_modifier(Modifier::UNDERLINED)
                    } else {
                        st
                    };
                    Some(st)
                }
                None => link_base,
            };
            match resolved {
                Some(st) => out.push(RSpan::styled(slice.to_string(), st)),
                None => out.push(RSpan::raw(slice.to_string())),
            }
            pos = region_end;
        }
    }
    out
}

/// Render one document's visible region into ratatui lines. Shared by the
/// full-screen reader (`draw`) and the File Browser preview (`draw_browse`).
fn render_doc_body(entry: &DocEntry, theme: Theme) -> Vec<RLine<'static>> {
    // Precompute the current-match identity for "is this the current one" checks.
    let current_logical: Option<(usize, usize)> = entry.search.as_ref().and_then(|s| {
        s.current.map(|i| {
            let m = &s.matches[i];
            (m.line_index, m.byte_range.start)
        })
    });

    let mut rendered: Vec<RLine<'static>> = Vec::new();
    for vl in entry.viewport.visible() {
        if let Some(role) = vl.metadata_row {
            let body_cols = entry.viewport.width as usize;
            rendered.push(render_metadata_row(
                entry
                    .doc
                    .metadata
                    .as_ref()
                    .expect("metadata_row visual line requires doc.metadata to be Some"),
                role,
                body_cols,
            ));
            continue;
        }

        // Heading spacer VisualLines reserve the rows below the main heading
        // line so the kitty image's cell footprint matches the viewport row
        // budget. Render as empty — the image paints over them. Checked before
        // indexing `doc.lines` so the metadata block's trailing blank (whose
        // `logical_index` is a sentinel) never dereferences a real line.
        if vl.is_spacer {
            rendered.push(RLine::from(Vec::<RSpan<'static>>::new()));
            continue;
        }

        let logical = &entry.doc.lines[vl.logical_index];

        let matches = visible_matches_for_line(
            entry.search.as_ref(),
            vl.logical_index,
            vl.byte_start,
            vl.byte_end,
            current_logical,
        );
        let rspans = clipped_spans(logical, vl.byte_start, vl.byte_end, &matches, theme);
        rendered.push(RLine::from(rspans));
    }
    rendered
}

/// Render the File Browser: file-list panel on the left, live preview on the
/// right, status row at the bottom.
fn draw_browse(frame: &mut ratatui::Frame, app: &App) {
    use ratatui::layout::{Constraint, Direction, Layout};
    use ratatui::style::{Modifier, Style as RStyle};
    use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(BROWSER_PANEL_WIDTH), Constraint::Min(20)])
        .split(chunks[0]);

    let Some(browser) = app.browser.as_ref() else {
        return;
    };

    // File list panel.
    let items: Vec<ListItem> = (0..browser.entries.len())
        .map(|i| ListItem::new(browser.name_at(i)))
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::RIGHT).title(" Files "))
        .highlight_style(RStyle::default().add_modifier(Modifier::REVERSED));
    let mut state = ListState::default();
    if !browser.entries.is_empty() {
        state.select(Some(browser.cursor));
    }
    frame.render_stateful_widget(list, split[0], &mut state);

    // Preview pane. While the selection is still building (async), show a
    // spinner instead of stale content; otherwise the rendered doc, or an
    // "unreadable" note if the build came back empty.
    let dim = RStyle::default().add_modifier(Modifier::DIM);
    if browse_selection_pending(app) {
        let msg = RLine::from(RSpan::styled("  ⟳ 加载中…", dim));
        frame.render_widget(Paragraph::new(msg), split[1]);
    } else {
        match app.preview.as_ref() {
            Some(entry) => {
                let rendered = render_doc_body(entry, app.theme);
                frame.render_widget(Paragraph::new(rendered), split[1]);
            }
            None => {
                let msg = RLine::from(RSpan::styled("  (无法读取该文件)", dim));
                frame.render_widget(Paragraph::new(msg), split[1]);
            }
        }
    }

    render_browse_status(frame, chunks[1], app);
}

/// Status row for Browse mode: directory + position.
fn render_browse_status(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App) {
    use ratatui::style::Style as RStyle;
    use ratatui::widgets::Paragraph;
    use unicode_width::UnicodeWidthStr;

    let total = area.width as usize;
    if total == 0 {
        return;
    }
    let Some(browser) = app.browser.as_ref() else {
        return;
    };

    let (bg, fg) = status_colors(app.theme);
    let style = RStyle::default().bg(bg).fg(fg);

    let dir = browser.dir.display().to_string();
    let pos = if browser.entries.is_empty() {
        "0/0".to_string()
    } else {
        format!("{}/{}", browser.cursor + 1, browser.entries.len())
    };
    let right = format!(" {pos} ");
    let left = format!(" {dir} ");
    let used = left.width() + right.width();
    let pad = total.saturating_sub(used);
    let line = if used <= total {
        format!("{left}{}{right}", " ".repeat(pad))
    } else {
        // Tight: middle-truncate the dir, keep the position.
        let max_dir = total.saturating_sub(right.width() + 2);
        let t = truncate_middle(&dir, max_dir);
        format!(" {t} {right}")
    };
    frame.render_widget(Paragraph::new(line).style(style), area);
}

fn draw(frame: &mut ratatui::Frame, app: &App) {
    use ratatui::layout::{Constraint, Direction, Layout};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let active = app.active();

    let rendered = render_doc_body(active, app.theme);

    let body_area = if active.toc_open {
        let split = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                ratatui::layout::Constraint::Length(TOC_PANEL_WIDTH),
                ratatui::layout::Constraint::Min(20),
            ])
            .split(chunks[0]);
        let toc_items: Vec<ratatui::widgets::ListItem> = active
            .doc
            .headings
            .iter()
            .map(|h| {
                let indent = "  ".repeat((h.level as usize).saturating_sub(1));
                ratatui::widgets::ListItem::new(format!("{indent}{}", h.text))
            })
            .collect();
        // Highlight the heading the viewport is currently inside: the last
        // heading whose logical line is at or above the top visible line.
        // `render_stateful_widget` auto-scrolls the sidebar so the selected
        // entry stays visible in long docs.
        let selected = active
            .viewport
            .visible()
            .first()
            // Metadata rows carry a sentinel `logical_index`; treat them as the
            // document top (logical 0) so the ToC doesn't select the last heading.
            .map(|vl| {
                if vl.logical_index == viewport::NO_LOGICAL {
                    0
                } else {
                    vl.logical_index
                }
            })
            .and_then(|top| {
                active
                    .doc
                    .headings
                    .iter()
                    .rposition(|h| h.line_index <= top)
            });
        let toc = ratatui::widgets::List::new(toc_items)
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::RIGHT)
                    .title("Contents"),
            )
            .highlight_style(
                ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::REVERSED),
            );
        let mut toc_state = ratatui::widgets::ListState::default();
        toc_state.select(selected);
        frame.render_stateful_widget(toc, split[0], &mut toc_state);
        split[1]
    } else {
        chunks[0]
    };

    let para = Paragraph::new(rendered);
    frame.render_widget(para, body_area);

    if matches!(app.mode, Mode::Help) {
        render_help_popup(frame, chunks[0], app.theme);
    }

    render_status_bar(frame, chunks[1], app);
}

const HELP_SECTIONS: &[(&str, &[(&str, &str)])] = &[
    (
        "Scroll",
        &[
            ("j  k  ↓  ↑", "line down / up"),
            ("d  u", "half page down / up"),
            ("f  b  space  PgDn  PgUp", "full page down / up"),
            ("gg  G", "jump to top / bottom"),
        ],
    ),
    (
        "Headings",
        &[
            ("]  [", "next / previous heading"),
            ("t", "toggle Table of Contents"),
        ],
    ),
    (
        "Search",
        &[("/", "search forward"), ("n  N", "next / previous match")],
    ),
    (
        "Links & history",
        &[
            ("Enter", "follow visible link"),
            ("1–9", "pick numbered link in overlay"),
            ("o  i", "back / forward in history"),
        ],
    ),
    (
        "Other",
        &[
            ("m", "toggle metadata fold (if frontmatter)"),
            ("?", "toggle this help"),
            ("q  Ctrl-C", "quit"),
        ],
    ),
];

/// Intrinsic `(width, height)` of the help popup including its border,
/// derived from `HELP_SECTIONS`. Used by both the renderer and the image
/// placement filter so they agree on the popup footprint.
fn help_popup_intrinsic_size() -> (u16, u16) {
    let key_col: usize = HELP_SECTIONS
        .iter()
        .flat_map(|(_, rows)| rows.iter().map(|(k, _)| k.chars().count()))
        .max()
        .unwrap_or(0);

    let mut line_count: usize = 0;
    let mut max_line_w: usize = 0;
    for (i, (title, rows)) in HELP_SECTIONS.iter().enumerate() {
        if i > 0 {
            line_count += 1; // blank separator
        }
        line_count += 1; // title row
        max_line_w = max_line_w.max(title.chars().count());
        for (k, desc) in *rows {
            line_count += 1;
            // Row layout: "  " + key_col + "   " + desc
            let w = 2 + key_col.max(k.chars().count()) + 3 + desc.chars().count();
            max_line_w = max_line_w.max(w);
        }
    }
    (max_line_w as u16 + 4, line_count as u16 + 2) // +2/+4 for border+padding
}

fn help_popup_rect(body: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let (inner_w, inner_h) = help_popup_intrinsic_size();
    let max_w = (body.width as f32 * 0.9) as u16;
    let max_h = (body.height as f32 * 0.9) as u16;
    let w = inner_w.min(max_w.max(20));
    let h = inner_h.min(max_h.max(6));
    let x = body.x + (body.width.saturating_sub(w)) / 2;
    let y = body.y + (body.height.saturating_sub(h)) / 2;
    ratatui::layout::Rect {
        x,
        y,
        width: w,
        height: h,
    }
}

fn render_help_popup(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, theme: Theme) {
    use ratatui::style::{Modifier, Style as RStyle};
    use ratatui::text::{Line as RLine, Span as RSpan};
    use ratatui::widgets::{Block, Borders, Clear, Paragraph};

    let accent = match theme {
        Theme::Dark => ratatui::style::Color::Cyan,
        Theme::Light => ratatui::style::Color::Blue,
    };
    let section = RStyle::default()
        .fg(accent)
        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
    let key = RStyle::default().add_modifier(Modifier::BOLD);
    let dim = RStyle::default().add_modifier(Modifier::DIM);

    let key_col: usize = HELP_SECTIONS
        .iter()
        .flat_map(|(_, rows)| rows.iter().map(|(k, _)| k.chars().count()))
        .max()
        .unwrap_or(0);

    let mut lines: Vec<RLine<'static>> = Vec::new();
    for (i, (title, rows)) in HELP_SECTIONS.iter().enumerate() {
        if i > 0 {
            lines.push(RLine::from(""));
        }
        lines.push(RLine::from(RSpan::styled(title.to_string(), section)));
        for (k, desc) in *rows {
            let pad = " ".repeat(key_col.saturating_sub(k.chars().count()));
            lines.push(RLine::from(vec![
                RSpan::raw("  "),
                RSpan::styled(format!("{k}{pad}"), key),
                RSpan::raw("   "),
                RSpan::styled(desc.to_string(), dim),
            ]));
        }
    }

    let popup = help_popup_rect(area);
    frame.render_widget(Clear, popup);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Keyboard shortcuts ");
    let para = Paragraph::new(lines).block(block);
    frame.render_widget(para, popup);
}

/// Render the single-row status bar: left region shows the active mode's
/// prompt (search query, link-select overlay, or empty for Normal), right
/// region shows `path  pct%`. When space is tight, `pct%` is dropped first,
/// then the path is middle-truncated so the filename tail stays visible.
fn render_status_bar(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App) {
    use ratatui::style::Style as RStyle;
    use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

    let total = area.width as usize;
    if total == 0 {
        return;
    }

    let active = app.active();
    let pct = progress_percent(app);
    let path = active.path.as_str();

    let (bg, fg) = status_colors(app.theme);
    let style = RStyle::default().bg(bg).fg(fg);

    let pct_s = format!("{pct}%");
    let path_w = path.width();
    let pct_w = pct_s.width();
    let right_full = 1 + path_w + 2 + pct_w + 1; // " <path>  <pct>% "
    let right_path_only = 1 + path_w + 1; // " <path> "

    let left_text: String = match &app.mode {
        Mode::Search { input, reverse } => {
            let prefix = if *reverse { "?" } else { "/" };
            let typed: String = input.lines().join("");
            format!("{prefix}{typed}")
        }
        Mode::LinkSelect { links } => {
            let mut label = String::from(" Open link: ");
            for (i, (content, _)) in links.iter().enumerate().take(9) {
                label.push_str(&format!("[{}]{}  ", i + 1, short(content, 20)));
            }
            if links.len() > 9 {
                label.push('…');
            }
            label
        }
        Mode::Help => String::from(" Help — press ? / Esc / q to close "),
        Mode::Normal => String::new(),
    };
    let left_w = left_text.width();
    // Guarantee at least one blank column between left and right when both
    // are non-empty so the regions stay visually distinct.
    let min_gap = if left_w > 0 { 1 } else { 0 };

    let (right_text, right_w) = if left_w + min_gap + right_full <= total {
        (format!(" {path}  {pct_s} "), right_full)
    } else if left_w + min_gap + right_path_only <= total {
        (format!(" {path} "), right_path_only)
    } else {
        let max_path = total.saturating_sub(left_w + min_gap + 2);
        if max_path == 0 {
            (String::new(), 0)
        } else {
            let t = truncate_middle(path, max_path);
            let w = 1 + t.width() + 1;
            (format!(" {t} "), w)
        }
    };

    let left_budget = total.saturating_sub(right_w);
    let left_fit = if left_w <= left_budget {
        left_text
    } else if left_budget == 0 {
        String::new()
    } else {
        let max = left_budget.saturating_sub(1);
        let mut out = String::new();
        let mut w = 0;
        for c in left_text.chars() {
            let cw = UnicodeWidthChar::width(c).unwrap_or(0);
            if w + cw > max {
                break;
            }
            out.push(c);
            w += cw;
        }
        out.push('…');
        out
    };

    let pad = total.saturating_sub(left_fit.width() + right_w);
    let line = format!("{left_fit}{}{right_text}", " ".repeat(pad));

    frame.render_widget(Paragraph::new(line).style(style), area);
}

/// Theme-aware (bg, fg) pair for the status row.
fn status_colors(theme: Theme) -> (ratatui::style::Color, ratatui::style::Color) {
    use ratatui::style::Color;
    match theme {
        // Soft gray on near-black — readable without feeling inverted.
        Theme::Dark => (Color::Indexed(236), Color::Indexed(252)),
        // Near-black on light gray — same family as style::Colors code_bg (253)
        // so the status row visually anchors to other light-theme surfaces.
        Theme::Light => (Color::Indexed(253), Color::Indexed(237)),
    }
}

/// Middle-truncate `s` to fit `max_cols` display columns, replacing the dropped
/// span with `…`. Biases toward keeping the tail (e.g. filename) over the head.
fn truncate_middle(s: &str, max_cols: usize) -> String {
    use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
    if s.width() <= max_cols {
        return s.to_string();
    }
    if max_cols == 0 {
        return String::new();
    }
    if max_cols == 1 {
        return "…".to_string();
    }
    let budget = max_cols - 1; // reserve 1 col for the ellipsis
    let tail_cols = (budget * 2) / 3;
    let head_cols = budget - tail_cols;

    let chars: Vec<char> = s.chars().collect();
    let mut hi = 0usize;
    let mut hw = 0usize;
    while hi < chars.len() {
        let cw = UnicodeWidthChar::width(chars[hi]).unwrap_or(0);
        if hw + cw > head_cols {
            break;
        }
        hw += cw;
        hi += 1;
    }
    let mut ti = chars.len();
    let mut tw = 0usize;
    while ti > hi {
        let cw = UnicodeWidthChar::width(chars[ti - 1]).unwrap_or(0);
        if tw + cw > tail_cols {
            break;
        }
        tw += cw;
        ti -= 1;
    }
    let head: String = chars[..hi].iter().collect();
    let tail: String = chars[ti..].iter().collect();
    format!("{head}…{tail}")
}

fn short(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max).collect();
        out.push('…');
        out
    }
}

fn progress_percent(app: &App) -> u32 {
    let vp = &app.active().viewport;
    let total = vp.total_visual_lines() as f64;
    if total == 0.0 {
        return 100;
    }
    let pos = (vp.top as f64 + vp.height as f64).min(total);
    ((pos / total) * 100.0).round() as u32
}

/// Query the terminal for its cell pixel height. Returns 0 if the OS or
/// terminal doesn't report it (per crossterm docs: "may not be reliably
/// implemented or default to 0").
fn query_cell_px_height() -> u32 {
    match crossterm::terminal::window_size() {
        Ok(ws) if ws.rows > 0 && ws.height > 0 => (ws.height as u32) / (ws.rows as u32),
        _ => 0,
    }
}

/// Refine image `rows` fields on every `HeadingImage` and every
/// `Span::HeadingImage` in `doc` to match the real terminal cell pixel
/// height. When `cell_px_height` is 0 (unknown), keeps the conservative
/// estimate set by layout.
fn refine_image_rows(doc: &mut layout::RenderedDoc, cell_px_height: u32) {
    if cell_px_height == 0 {
        return;
    }
    // Build id → real_rows map from doc.images' pixel heights.
    let mut real_rows: HashMap<u32, u16> = HashMap::new();
    for img in &mut doc.images {
        let r = img.px_height.div_ceil(cell_px_height).max(1) as u16;
        img.rows = r;
        real_rows.insert(img.id, r);
    }
    // Propagate to each `Span::HeadingImage` so `draw()` and
    // `desired_image_placements()` see the same row count.
    for line in &mut doc.lines {
        for span in &mut line.spans {
            if let layout::Span::HeadingImage { id, rows } = span {
                if let Some(&r) = real_rows.get(id) {
                    *rows = r;
                }
            }
        }
    }
}

fn desired_image_placements(app: &App) -> HashMap<u32, (u16, u16)> {
    let active = app.active();
    // Heading images must start past the ToC panel when it is open.
    let col_offset: u16 = if active.toc_open { TOC_PANEL_WIDTH } else { 0 };
    // When the help popup is open, drop placements whose rows intersect the
    // popup rectangle. Kitty images live on a separate graphics layer, so
    // without this they would show through the popup; dropping them all
    // (earlier behavior) also hid headings above/below the popup.
    let popup_rows: Option<(u16, u16)> = if matches!(app.mode, Mode::Help) {
        let (full_w, body_h) = app.term_size;
        let popup = help_popup_rect(ratatui::layout::Rect {
            x: 0,
            y: 0,
            width: full_w,
            height: body_h,
        });
        Some((popup.y, popup.y.saturating_add(popup.height)))
    } else {
        None
    };
    placements_for(active, col_offset, active.viewport.height, popup_rows)
}

/// Compute the desired `id → (col, row)` heading-image placement map for one
/// document's visible region. `col_offset` shifts images past a left panel
/// (ToC or File Browser); `popup_rows`, if set, suppresses images overlapping
/// the help popup. Shared by the reader and the browser preview.
fn placements_for(
    entry: &DocEntry,
    col_offset: u16,
    body_height: u16,
    popup_rows: Option<(u16, u16)>,
) -> HashMap<u32, (u16, u16)> {
    let mut out = HashMap::new();
    // wrap_all emits one VisualLine per screen row (headings expand into
    // N rows: main + spacers), so visual_row just increments by 1 each
    // iteration and matches the row count used by draw() + the viewport.
    for (visual_row, vl) in entry.viewport.visible().iter().enumerate() {
        // Metadata rows carry a sentinel logical_index and never hold images.
        if vl.metadata_row.is_some() || vl.is_spacer || vl.byte_start != 0 {
            continue;
        }
        let logical = &entry.doc.lines[vl.logical_index];
        let vr = visual_row as u16;
        for span in &logical.spans {
            if let layout::Span::HeadingImage { id, rows } = span {
                // Skip placement if the image's full row budget doesn't fit
                // within the body area. Kitty paints images at natural
                // pixel size, so a heading near the bottom would otherwise
                // bleed into the status bar. The user can scroll another
                // row or two to bring the whole heading on-screen.
                if vr.saturating_add(*rows) > body_height {
                    continue;
                }
                if let Some((py0, py1)) = popup_rows {
                    let img_end = vr.saturating_add(*rows);
                    if vr < py1 && img_end > py0 {
                        continue;
                    }
                }
                out.insert(*id, (col_offset, vr));
            }
        }
    }
    out
}

#[cfg(test)]
mod open_link_tests {
    use super::*;

    #[test]
    fn looks_like_local_md_accepts_relative_md_paths() {
        assert!(looks_like_local_md("other.md"));
        assert!(looks_like_local_md("./docs/other.md"));
        assert!(looks_like_local_md("../a.md"));
        assert!(looks_like_local_md("a.markdown"));
        assert!(looks_like_local_md("a.MD"));
    }

    #[test]
    fn looks_like_local_md_rejects_urls_and_non_md() {
        assert!(!looks_like_local_md("https://example.com/a.md"));
        assert!(!looks_like_local_md("http://a.md"));
        assert!(!looks_like_local_md("file:///a.md"));
        assert!(!looks_like_local_md("other.txt"));
        assert!(!looks_like_local_md(""));
    }
}

#[cfg(test)]
mod help_popup_tests {
    use super::{help_popup_intrinsic_size, help_popup_rect};
    use ratatui::layout::Rect;

    #[test]
    fn intrinsic_size_is_positive_and_bounded() {
        let (w, h) = help_popup_intrinsic_size();
        assert!(w > 4, "width should include border+padding, got {w}");
        assert!(
            h > 2,
            "height should include at least one content row, got {h}"
        );
        // Sanity: the popup is small enough to fit inside a typical 80x25 terminal.
        assert!(w <= 80);
        assert!(h <= 25);
    }

    #[test]
    fn popup_centers_within_body() {
        let body = Rect {
            x: 0,
            y: 0,
            width: 120,
            height: 40,
        };
        let p = help_popup_rect(body);
        // Left and right gutters should be equal within 1 cell (rounding).
        let right_gutter = body.width - (p.x + p.width);
        assert!(p.x.abs_diff(right_gutter) <= 1);
        let bottom_gutter = body.height - (p.y + p.height);
        assert!(p.y.abs_diff(bottom_gutter) <= 1);
    }

    #[test]
    fn popup_caps_at_ninety_percent_of_tiny_body() {
        // Tiny body: popup should cap at 90% and still fit.
        let body = Rect {
            x: 0,
            y: 0,
            width: 30,
            height: 10,
        };
        let p = help_popup_rect(body);
        assert!(p.width <= 27, "width {} should cap at 90% of 30", p.width);
        assert!(p.height <= 9, "height {} should cap at 90% of 10", p.height);
        assert!(p.x + p.width <= body.x + body.width);
        assert!(p.y + p.height <= body.y + body.height);
    }

    #[test]
    fn popup_respects_body_offset() {
        let body = Rect {
            x: 10,
            y: 5,
            width: 80,
            height: 30,
        };
        let p = help_popup_rect(body);
        assert!(p.x >= body.x);
        assert!(p.y >= body.y);
        assert!(p.x + p.width <= body.x + body.width);
        assert!(p.y + p.height <= body.y + body.height);
    }
}

#[cfg(test)]
mod truncate_middle_tests {
    use super::truncate_middle;
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn passthrough_when_within_budget() {
        assert_eq!(truncate_middle("readme.md", 20), "readme.md");
        assert_eq!(truncate_middle("readme.md", 9), "readme.md");
    }

    #[test]
    fn drops_middle_and_keeps_tail_bias() {
        let out = truncate_middle("docs/superpowers/specs/tui-status-bar.md", 20);
        assert!(out.contains('…'));
        assert!(out.width() <= 20);
        // Filename tail stays visible.
        assert!(out.ends_with(".md"));
    }

    #[test]
    fn respects_cjk_widths() {
        // Each CJK char is width 2.
        let s = "文档/测试/读取.md";
        let out = truncate_middle(s, 10);
        assert!(out.width() <= 10);
        assert!(out.contains('…'));
    }

    #[test]
    fn edge_cases() {
        assert_eq!(truncate_middle("abc", 0), "");
        assert_eq!(truncate_middle("abc", 1), "…");
        let out = truncate_middle("abcdef", 2);
        assert_eq!(out.width(), 2);
        assert!(out.contains('…'));
    }
}
