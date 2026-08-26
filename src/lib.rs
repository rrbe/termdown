mod cat;
mod config;
mod font;
mod frontmatter;
mod layout;
mod render;
mod style;
mod theme;
mod tui;

use std::fs;
use std::io::{self, BufWriter, Read, Write};

use crate::config::ThemeChoice;

use crossterm::tty::IsTty;
use terminal_size::{terminal_size, Width};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run() {
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
        println!("  -h, --help                Show this help message");
        println!("  -V, --version             Show version");
        println!("  --theme <auto|dark|light>  Color theme (default: auto-detect)");
        println!("  --cat                     Force non-interactive cat-style output");
        println!("  --render                  Render Markdown from stdin as ANSI for editors");
        println!("  --width <columns>          Output width for --render (required)");
        println!("  --no-bell                 Disable the edge-scroll terminal bell");
        println!("  -w, --watch               Watch the file and live-reload on save (TUI only)");
        println!();
        println!("By default, passing FILE opens it in the interactive TUI.");
        println!("Piped/redirected stdout and stdin input automatically use cat mode.");
        println!();
        println!("Config: ~/.config/termdown/config.toml");
        return;
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("termdown {VERSION}");
        return;
    }

    let cat_flag = args.iter().any(|a| a == "--cat");
    let render_flag = args.iter().any(|a| a == "--render");
    let no_bell_flag = args.iter().any(|a| a == "--no-bell");
    let watch_flag = args.iter().any(|a| a == "--watch" || a == "-w");

    let render_width = parse_render_width(&args, render_flag).unwrap_or_else(|message| {
        eprintln!("termdown: {message}");
        std::process::exit(2);
    });

    let mut config = config::load();
    if no_bell_flag {
        config.bell = Some(false);
    }
    if watch_flag {
        config.watch = Some(true);
    }

    // Parse --theme flag (takes precedence over config). An unrecognized value
    // warns and falls through to config/auto instead of being silently
    // swallowed, matching how the config file reports an invalid theme.
    let cli_theme = args.windows(2).find(|w| w[0] == "--theme").and_then(|w| {
        match w[1].parse::<ThemeChoice>() {
            Ok(choice) => Some(choice),
            Err(()) => {
                eprintln!(
                    "Warning: ignoring invalid --theme value {:?}; expected auto, dark, or light",
                    w[1]
                );
                None
            }
        }
    });

    // Resolve theme: CLI flag > config file > auto-detect.
    let theme = resolve_theme(cli_theme, config.theme);

    // Collect file arg, skipping flags that consume a value.
    let file_arg = {
        let mut i = 1;
        let mut found = None;
        while i < args.len() {
            if args[i] == "--theme" || args[i] == "--width" {
                i += 2; // skip flag + value
            } else if !args[i].starts_with('-') || args[i] == "-" {
                found = Some(args[i].clone());
                break;
            } else {
                i += 1;
            }
        }
        found
    };

    if render_flag {
        if cat_flag {
            cli_error("--render cannot be used with --cat");
        }
        if watch_flag {
            cli_error("--render cannot be used with --watch");
        }
        if file_arg.is_some() {
            cli_error("--render reads Markdown from stdin and does not accept FILE");
        }
    } else {
        check_terminal_support();
    }

    // TUI is the default when we have a real file path and stdout is a
    // terminal; --cat, piping/redirecting, or stdin input all fall through
    // to cat mode so scripts like `termdown foo.md | less` keep working.
    let want_tui =
        !cat_flag && matches!(file_arg.as_deref(), Some(p) if p != "-") && io::stdout().is_tty();

    // Watching only makes sense for the interactive TUI: cat mode renders once
    // and stdin has no path to re-read. Warn only when the user explicitly
    // asked for it via the flag (a config `watch = true` stays silent).
    if watch_flag && !want_tui {
        eprintln!("termdown: --watch only applies to the interactive TUI; ignoring");
    }

    if want_tui {
        let path = file_arg.expect("want_tui implies a file path");
        let watch = config.watch.unwrap_or(false);
        tui::run(&path, &config, theme, watch);
        return;
    }

    let md = match file_arg.as_deref() {
        None | Some("-") => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| {
                eprintln!("Error reading stdin: {e}");
                std::process::exit(1);
            });
            buf
        }
        Some(path) => fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("Error reading {path}: {e}");
            std::process::exit(1);
        }),
    };

    let term_width = render_width.unwrap_or_else(|| {
        terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(80)
    });

    let colors = crate::style::Colors::for_theme(theme);

    if render_flag {
        let doc = layout::build_with_options(
            &md,
            &config,
            theme,
            layout::BuildOptions {
                heading_images: false,
            },
        );
        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());
        cat::write_ansi(&mut out, &doc, term_width, &colors, &config, false)
            .and_then(|()| out.flush())
            .unwrap_or_else(|e| {
                eprintln!("termdown: failed to write rendered output: {e}");
                std::process::exit(1);
            });
        return;
    }

    // iTerm2 ignores the Kitty `q=2` response-suppression flag and sends OK
    // ACKs anyway; disable ECHO there so they don't leak to the screen. Other
    // terminals (Ghostty, Kitty, WezTerm) respect `q=2` — leave ECHO alone so
    // we don't trip Ghostty's Secure Keyboard Entry auto-enable heuristic,
    // which treats `~ECHO` as a password prompt.
    #[cfg(unix)]
    let saved_termios = is_iterm2().then(disable_echo);

    let doc = layout::build(&md, &config, theme);
    cat::print(&doc, term_width, &colors, &config);

    #[cfg(unix)]
    {
        if let Some(saved) = saved_termios {
            render::drain_iterm2_acks();
            restore_termios(&saved);
        } else {
            render::flush_stdin();
        }
    }
}

fn parse_render_width(args: &[String], render_flag: bool) -> Result<Option<usize>, String> {
    let mut width = None;
    let mut i = 1;
    while i < args.len() {
        if args[i] != "--width" {
            i += 1;
            continue;
        }
        if !render_flag {
            return Err("--width can only be used with --render".into());
        }
        if width.is_some() {
            return Err("--width may only be specified once".into());
        }
        let value = args
            .get(i + 1)
            .ok_or_else(|| "--width requires a positive integer".to_string())?;
        let parsed = value
            .parse::<usize>()
            .ok()
            .filter(|value| *value > 0)
            .ok_or_else(|| {
                format!("invalid --width value {value:?}; expected a positive integer")
            })?;
        width = Some(parsed);
        i += 2;
    }

    if render_flag && width.is_none() {
        return Err("--render requires --width <columns>".into());
    }
    Ok(width)
}

fn cli_error(message: &str) -> ! {
    eprintln!("termdown: {message}");
    std::process::exit(2);
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

fn is_iterm2() -> bool {
    std::env::var("TERM_PROGRAM")
        .map(|v| v.eq_ignore_ascii_case("iTerm.app"))
        .unwrap_or(false)
}

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

fn resolve_theme(cli: Option<ThemeChoice>, config: Option<ThemeChoice>) -> theme::Theme {
    match cli.or(config).unwrap_or(ThemeChoice::Auto) {
        ThemeChoice::Dark => theme::Theme::Dark,
        ThemeChoice::Light => theme::Theme::Light,
        ThemeChoice::Auto => theme::detect(),
    }
}
