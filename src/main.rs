mod config;
mod font;
mod markdown;
mod render;
mod style;

use std::fs;
use std::io::{self, Read};

use terminal_size::{terminal_size, Width};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("termdown {VERSION} - Render Markdown with large-font headings in the terminal");
        println!();
        println!("Usage: {} [OPTIONS] [FILE]", args[0]);
        println!();
        println!("Arguments:");
        println!("  FILE    Markdown file to render (use - or omit for stdin)");
        println!();
        println!("Options:");
        println!("  -h, --help     Show this help message");
        println!("  -V, --version  Show version");
        println!();
        println!("Config: ~/.termdown/config.toml");
        return;
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("termdown {VERSION}");
        return;
    }

    check_terminal_support();

    let config = config::load();

    let md = if args.len() < 2 || args[1] == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| {
            eprintln!("Error reading stdin: {e}");
            std::process::exit(1);
        });
        buf
    } else {
        fs::read_to_string(&args[1]).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {e}", args[1]);
            std::process::exit(1);
        })
    };

    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    // Disable terminal echo so Kitty graphics protocol responses
    // (e.g. iTerm2's OK acknowledgments) don't appear on screen.
    #[cfg(unix)]
    let saved_termios = disable_echo();

    markdown::render(&md, term_width, &config);

    // Drain any pending responses, then restore terminal state.
    #[cfg(unix)]
    {
        render::drain_kitty_responses();
        restore_termios(&saved_termios);
    }
}

fn check_terminal_support() {
    let term_program = std::env::var("TERM_PROGRAM")
        .unwrap_or_default()
        .to_lowercase();
    let term = std::env::var("TERM").unwrap_or_default().to_lowercase();

    let likely_supported = term_program == "ghostty"
        || term_program == "kitty"
        || term_program == "wezterm"
        || term_program == "iterm.app"
        || term.contains("kitty")
        || term.contains("ghostty")
        || std::env::var("KITTY_WINDOW_ID").is_ok();

    if !likely_supported {
        eprintln!("termdown: warning: terminal may not support Kitty graphics protocol");
        eprintln!("termdown: headings require Ghostty, Kitty, WezTerm, or iTerm2");
    }
}

// ─── UNIX Terminal State ────────────────────────────────────────────────────

#[cfg(unix)]
fn disable_echo() -> libc::termios {
    // SAFETY: tcgetattr/tcsetattr are standard POSIX calls on valid fd.
    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut termios);
        let saved = termios;
        termios.c_lflag &= !libc::ECHO;
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &termios);
        saved
    }
}

#[cfg(unix)]
fn restore_termios(saved: &libc::termios) {
    // SAFETY: restoring previously saved termios state.
    unsafe {
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, saved);
    }
}
