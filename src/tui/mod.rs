//! Interactive TUI mode. Enters alternate screen + raw mode, runs an event
//! loop until the user quits with `q` or Ctrl-C, and restores the terminal
//! on exit. Content-aware scrolling, search, Kitty image lifecycle, and
//! per-doc history arrive in later tasks.

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;

use crate::config::Config;
use crate::layout;
use crate::theme::Theme;

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

fn run_ui(_doc: layout::RenderedDoc) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = event_loop(&mut terminal);

    // Always restore terminal state, regardless of event_loop outcome.
    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn event_loop<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let block = Block::default().borders(Borders::NONE);
            let para = Paragraph::new("termdown TUI — press q to quit").block(block);
            frame.render_widget(para, frame.area());
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}
