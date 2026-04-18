//! Interactive TUI mode.

mod input;
mod kitty;
mod search;
mod viewport;

use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Duration;

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
use crate::style::MARGIN_WIDTH;
use crate::theme::Theme;
use crate::tui::search::{Direction as SearchDirection, SearchState};

use viewport::Viewport;

enum Mode {
    Normal,
    Search {
        input: Box<TextArea<'static>>,
        reverse: bool,
    },
    LinkSelect {
        links: Vec<(String, String)>, // (label_content, url)
    },
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
}

impl App {
    fn new_with_initial_doc(
        path: String,
        doc: layout::RenderedDoc,
        body_height: u16,
        width: u16,
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
        };
        app.push_new_doc(path, doc);
        app
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
    #[allow(dead_code)] // Consumed by Task 7.3 (follow-link-to-another-md).
    fn push_new_doc(&mut self, path: String, mut doc: layout::RenderedDoc) -> usize {
        let offset = self.next_image_id;
        // layout::build() assigns ids starting at 1; shift each by (offset - 1)
        // so the first image of this doc becomes `offset`.
        let mut id_map: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for img in &mut doc.images {
            let new_id = offset + (img.id - 1);
            id_map.insert(img.id, new_id);
            img.id = new_id;
        }
        if let Some(max) = doc.images.iter().map(|i| i.id).max() {
            self.next_image_id = max + 1;
        }
        // Patch Span::HeadingImage and LineKind::Heading { id } references.
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
        let (width, height) = self.term_size;
        let viewport = Viewport::new(height, width);
        let entry = DocEntry {
            path,
            doc,
            viewport,
            search: None,
            pending_g: false,
            toc_open: false,
        };
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

    if let Err(e) = run_ui(doc, path.to_string()) {
        eprintln!("termdown: tui error: {e}");
        std::process::exit(1);
    }
}

fn run_ui(doc: layout::RenderedDoc, path: String) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let body_height = size.height.saturating_sub(1);
    let mut app = App::new_with_initial_doc(path, doc, body_height, size.width);

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
        {
            let active = app.active_mut();
            active.viewport.ensure_wrap(&active.doc);
        }
        terminal.draw(|frame| draw(frame, app))?;

        {
            let mut stdout = io::stdout().lock();
            let desired = desired_image_placements(app);
            let _ = app.images.sync(&mut stdout, &desired);
            let _ = stdout.flush();
        }

        if event::poll(Duration::from_millis(16))? {
            let ev = event::read()?;
            match &mut app.mode {
                Mode::Normal => handle_normal_key(app, &ev)?,
                Mode::Search { .. } => handle_search_key(app, ev)?,
                Mode::LinkSelect { .. } => handle_link_select_key(app, ev)?,
            }
            if app.should_quit {
                return Ok(());
            }
        }
    }
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
            input::Action::ScrollLines(d) => app.active_mut().viewport.scroll_by(d),
            input::Action::ScrollHalfPage(s) => {
                let active = app.active_mut();
                let delta = (active.viewport.height as i32 / 2) * s;
                active.viewport.scroll_by(delta);
            }
            input::Action::ScrollPage(s) => {
                let active = app.active_mut();
                let delta = active.viewport.height as i32 * s;
                active.viewport.scroll_by(delta);
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
                // Recompute viewport width for the new body area.
                let new_width = if active.toc_open {
                    active.viewport.width.saturating_sub(30)
                } else {
                    active.viewport.width.saturating_add(30)
                };
                active.viewport.width = new_width;
                // ensure_wrap keys on `cache_width == self.width`; changing width
                // invalidates the cache so the next frame re-wraps correctly.
            }
            input::Action::Back => {
                if let Some(prev) = app.history.pop() {
                    app.forward.push(app.cursor);
                    app.cursor = prev;
                    let mut out = io::stdout().lock();
                    let _ = app.register_active_images(&mut out);
                    let _ = out.flush();
                }
            }
            input::Action::Forward => {
                if let Some(next) = app.forward.pop() {
                    app.history.push(app.cursor);
                    app.cursor = next;
                    let mut out = io::stdout().lock();
                    let _ = app.register_active_images(&mut out);
                    let _ = out.flush();
                }
            }
            input::Action::OpenLink => {
                let links = visible_links(app);
                match links.len() {
                    0 => {}
                    1 => {
                        open_url(&links[0].1);
                    }
                    _ => {
                        app.mode = Mode::LinkSelect { links };
                    }
                }
            }
            // Other actions land in later tasks. No-op for now.
            _ => {}
        }
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
            let direction = if reverse {
                SearchDirection::Backward
            } else {
                SearchDirection::Forward
            };
            let state = SearchState::new(query, direction, &app.active().doc);
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
                open_url(&url);
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

fn open_url(url: &str) {
    let cmd = if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "xdg-open"
    };

    if cmd == "cmd" {
        // Windows `cmd /C start "" "https://..."` is the portable way.
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

fn clipped_spans(
    line: &layout::Line,
    byte_start: usize,
    byte_end: usize,
    matches: &[VisibleMatch],
) -> Vec<RSpan<'static>> {
    use ratatui::style::{Color as RColor, Style as RStyle};

    let mut out: Vec<RSpan<'static>> = Vec::new();
    let mut cursor = 0usize;

    // Highlight styles. Task 8/theme follow-up: pull from style::Colors instead.
    let current_style = RStyle::default().bg(RColor::Yellow).fg(RColor::Black);
    let other_style = RStyle::default()
        .bg(RColor::Rgb(80, 80, 0))
        .fg(RColor::White);

    for span in &line.spans {
        let (content, is_image) = match span {
            layout::Span::Text { content, .. } | layout::Span::Link { content, .. } => {
                (content.as_str(), false)
            }
            layout::Span::HeadingImage { .. } => ("", true),
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
            match style {
                Some(st) => out.push(RSpan::styled(slice.to_string(), st)),
                None => out.push(RSpan::raw(slice.to_string())),
            }
            pos = region_end;
        }
    }
    out
}

fn draw(frame: &mut ratatui::Frame, app: &App) {
    use ratatui::layout::{Constraint, Direction, Layout};
    use ratatui::style::{Color as RColor, Style as RStyle};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let active = app.active();

    // Body
    // Precompute the current-match identity for "is this the current one" checks.
    let current_logical: Option<(usize, usize)> = active.search.as_ref().and_then(|s| {
        s.current.map(|i| {
            let m = &s.matches[i];
            (m.line_index, m.byte_range.start)
        })
    });

    let mut rendered: Vec<RLine> = Vec::new();
    for vl in active.viewport.visible() {
        let logical = &active.doc.lines[vl.logical_index];

        // Compute image-height expansion (only on the first visual line for the logical).
        let is_first_visual_of_logical = vl.byte_start == 0;
        let mut image_rows: u16 = 0;
        if is_first_visual_of_logical {
            for span in &logical.spans {
                if let layout::Span::HeadingImage { rows, .. } = span {
                    image_rows = image_rows.max(*rows);
                }
            }
        }

        let matches = visible_matches_for_line(
            active.search.as_ref(),
            vl.logical_index,
            vl.byte_start,
            vl.byte_end,
            current_logical,
        );
        let rspans = clipped_spans(logical, vl.byte_start, vl.byte_end, &matches);
        rendered.push(RLine::from(rspans));
        for _ in 1..image_rows.max(1) {
            if image_rows == 0 {
                break;
            }
            rendered.push(RLine::from(Vec::<RSpan>::new()));
        }
    }

    let body_area = if active.toc_open {
        let split = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                ratatui::layout::Constraint::Length(30),
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
        let toc = ratatui::widgets::List::new(toc_items).block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::RIGHT)
                .title("Contents"),
        );
        frame.render_widget(toc, split[0]);
        split[1]
    } else {
        chunks[0]
    };

    let para = Paragraph::new(rendered);
    frame.render_widget(para, body_area);

    // Status bar
    let pct = progress_percent(app);
    let status_text = format!(" {}  {pct}%", active.path);
    let status =
        Paragraph::new(status_text).style(RStyle::default().bg(RColor::DarkGray).fg(RColor::White));
    frame.render_widget(status, chunks[1]);

    // Status row override — show search prompt if in Search mode.
    if let Mode::Search { input, reverse } = &app.mode {
        let prefix = if *reverse { "?" } else { "/" };
        let typed: String = input.lines().join("");
        let prompt_text = format!("{prefix}{typed}");
        let prompt = Paragraph::new(prompt_text);
        frame.render_widget(prompt, chunks[1]);
    }

    // Status row override — show link-select overlay if in LinkSelect mode.
    if let Mode::LinkSelect { links } = &app.mode {
        let mut label = String::from(" Open link: ");
        for (i, (content, _)) in links.iter().enumerate().take(9) {
            label.push_str(&format!("[{}]{}  ", i + 1, short(content, 20)));
        }
        if links.len() > 9 {
            label.push('…');
        }
        let overlay = Paragraph::new(label).style(
            ratatui::style::Style::default()
                .bg(ratatui::style::Color::DarkGray)
                .fg(ratatui::style::Color::White),
        );
        frame.render_widget(overlay, chunks[1]);
    }
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

fn desired_image_placements(app: &App) -> HashMap<u32, (u16, u16)> {
    let active = app.active();
    let col_offset: u16 = if active.toc_open {
        // ToC panel (30) + margin within body panel (MARGIN_WIDTH).
        30 + MARGIN_WIDTH as u16
    } else {
        MARGIN_WIDTH as u16
    };
    let mut out = HashMap::new();
    let mut visual_row: u16 = 0;
    for vl in active.viewport.visible() {
        let logical = &active.doc.lines[vl.logical_index];
        let is_first_visual_of_logical = vl.byte_start == 0;
        let mut image_rows: u16 = 0;
        if is_first_visual_of_logical {
            for span in &logical.spans {
                if let layout::Span::HeadingImage { id, rows } = span {
                    out.insert(*id, (col_offset, visual_row));
                    image_rows = image_rows.max(*rows);
                }
            }
        }
        if image_rows > 0 {
            visual_row = visual_row.saturating_add(image_rows);
        } else {
            visual_row = visual_row.saturating_add(1);
        }
    }
    out
}
