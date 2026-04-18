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
}

struct App {
    doc: layout::RenderedDoc,
    viewport: Viewport,
    images: kitty::ImageLifecycle,
    pending_g: bool,
    path: String,
    mode: Mode,
    search: Option<SearchState>,
    should_quit: bool,
}

impl App {
    fn new(doc: layout::RenderedDoc, path: String, height: u16, width: u16) -> Self {
        Self {
            doc,
            viewport: Viewport::new(height, width),
            images: kitty::ImageLifecycle::default(),
            pending_g: false,
            path,
            mode: Mode::Normal,
            search: None,
            should_quit: false,
        }
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
    let mut app = App::new(doc, path, body_height, size.width);

    // Transmit all heading PNGs once; subsequent frames only emit placement commands.
    {
        let mut stdout = io::stdout().lock();
        for img in &app.doc.images {
            app.images.register(&mut stdout, img.id, &img.png)?;
        }
        stdout.flush()?;
    }

    let result = event_loop(&mut terminal, &mut app);

    {
        let mut stdout = io::stdout().lock();
        let _ = app.images.cleanup(&mut stdout);
        let _ = stdout.flush();
    }

    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn event_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        app.viewport.ensure_wrap(&app.doc);
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
            if app.pending_g {
                app.viewport.top = 0;
                app.pending_g = false;
            } else {
                app.pending_g = true;
            }
            return Ok(());
        }
        app.pending_g = false;

        match input::map_normal(*key) {
            input::Action::Quit => {
                app.should_quit = true;
            }
            input::Action::ScrollLines(d) => app.viewport.scroll_by(d),
            input::Action::ScrollHalfPage(s) => {
                let delta = (app.viewport.height as i32 / 2) * s;
                app.viewport.scroll_by(delta);
            }
            input::Action::ScrollPage(s) => {
                let delta = app.viewport.height as i32 * s;
                app.viewport.scroll_by(delta);
            }
            input::Action::JumpStart => app.viewport.top = 0,
            input::Action::JumpEnd => {
                let max_top = app
                    .viewport
                    .total_visual_lines()
                    .saturating_sub(app.viewport.height as usize);
                app.viewport.top = max_top;
            }
            input::Action::NextHeading => {
                app.viewport
                    .jump_to_next_heading(&app.doc, app.viewport.top);
            }
            input::Action::PrevHeading => {
                app.viewport
                    .jump_to_prev_heading(&app.doc, app.viewport.top);
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
            let state = SearchState::new(query, direction, &app.doc);
            app.search = Some(state);
            apply_search_jump(app, reverse);
        }
        _ => {
            input.input(key);
        }
    }
    Ok(())
}

fn apply_search_jump(app: &mut App, reverse: bool) {
    let Some(state) = app.search.as_mut() else {
        return;
    };
    if state.matches.is_empty() {
        state.current = None;
        return;
    }
    let current_logical = app
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
    center_on_logical(&mut app.viewport, line);
}

fn advance_search(app: &mut App, delta: i32) {
    let Some(state) = app.search.as_mut() else {
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
    center_on_logical(&mut app.viewport, line);
}

fn center_on_logical(vp: &mut Viewport, logical: usize) {
    if let Some(vi) = vp.visual_line_for_logical(logical) {
        let third = (vp.height as usize) / 3;
        let new_top = vi.saturating_sub(third);
        let max_top = vp.total_visual_lines().saturating_sub(vp.height as usize);
        vp.top = new_top.min(max_top);
    }
}

fn clipped_spans(line: &layout::Line, byte_start: usize, byte_end: usize) -> Vec<RSpan<'static>> {
    let mut out: Vec<RSpan<'static>> = Vec::new();
    let mut cursor = 0usize;
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

        // Intersect [span_start, span_end) with [byte_start, byte_end).
        let s = span_start.max(byte_start);
        let e = span_end.min(byte_end);
        if s >= e {
            continue;
        }
        let slice_start = s - span_start;
        let slice_end = e - span_start;
        let slice = &content[slice_start..slice_end];
        // Guard: slice may start/end in the middle of a multi-byte char.
        // For v1, wrap always breaks at char boundaries so this should hold,
        // but be defensive: if the indices aren't at char boundaries, skip.
        if !content.is_char_boundary(slice_start) || !content.is_char_boundary(slice_end) {
            continue;
        }
        out.push(RSpan::raw(slice.to_string()));
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

    // Body
    let mut rendered: Vec<RLine> = Vec::new();
    for vl in app.viewport.visible() {
        let logical = &app.doc.lines[vl.logical_index];

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

        let rspans = clipped_spans(logical, vl.byte_start, vl.byte_end);
        rendered.push(RLine::from(rspans));
        for _ in 1..image_rows.max(1) {
            if image_rows == 0 {
                break;
            }
            rendered.push(RLine::from(Vec::<RSpan>::new()));
        }
    }

    let para = Paragraph::new(rendered);
    frame.render_widget(para, chunks[0]);

    // Status bar
    let pct = progress_percent(app);
    let status_text = format!(" {}  {pct}%", app.path);
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
}

fn progress_percent(app: &App) -> u32 {
    let total = app.viewport.total_visual_lines() as f64;
    if total == 0.0 {
        return 100;
    }
    let pos = (app.viewport.top as f64 + app.viewport.height as f64).min(total);
    ((pos / total) * 100.0).round() as u32
}

fn desired_image_placements(app: &App) -> HashMap<u32, (u16, u16)> {
    let col = MARGIN_WIDTH as u16;
    let mut out = HashMap::new();
    let mut visual_row: u16 = 0;
    for vl in app.viewport.visible() {
        let logical = &app.doc.lines[vl.logical_index];
        let is_first_visual_of_logical = vl.byte_start == 0;
        let mut image_rows: u16 = 0;
        if is_first_visual_of_logical {
            for span in &logical.spans {
                if let layout::Span::HeadingImage { id, rows } = span {
                    out.insert(*id, (col, visual_row));
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
