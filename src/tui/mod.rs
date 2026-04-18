//! Interactive TUI mode.

mod viewport;

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::text::{Line as RLine, Span as RSpan};
use ratatui::widgets::Paragraph;
use ratatui::Terminal;

use crate::config::Config;
use crate::layout;
use crate::theme::Theme;

use viewport::Viewport;

struct App {
    doc: layout::RenderedDoc,
    viewport: Viewport,
}

impl App {
    fn new(doc: layout::RenderedDoc, height: u16, width: u16) -> Self {
        Self {
            doc,
            viewport: Viewport::new(height, width),
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

    if let Err(e) = run_ui(doc) {
        eprintln!("termdown: tui error: {e}");
        std::process::exit(1);
    }
}

fn run_ui(doc: layout::RenderedDoc) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let mut app = App::new(doc, size.height, size.width);

    let result = event_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn event_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        app.viewport.ensure_wrap(&app.doc);
        terminal.draw(|frame| draw(frame, app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c') if ctrl => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => app.viewport.scroll_by(1),
                    KeyCode::Char('k') | KeyCode::Up => app.viewport.scroll_by(-1),
                    _ => {}
                }
            }
        }
    }
}

fn draw(frame: &mut ratatui::Frame, app: &App) {
    let rendered: Vec<RLine> = app
        .viewport
        .visible()
        .iter()
        .map(|vl| {
            let logical = &app.doc.lines[vl.logical_index];
            let mut rspans: Vec<RSpan> = Vec::new();
            for span in &logical.spans {
                match span {
                    layout::Span::Text { content, .. } | layout::Span::Link { content, .. } => {
                        rspans.push(RSpan::raw(content.clone()));
                    }
                    layout::Span::HeadingImage { .. } => {
                        // Placeholder — Task 3.3 replaces with ImageReserve widget.
                        rspans.push(RSpan::raw("[image]"));
                    }
                }
            }
            RLine::from(rspans)
        })
        .collect();

    let para = Paragraph::new(rendered);
    frame.render_widget(para, frame.area());
}
