use unicode_width::UnicodeWidthChar;

// ─── Heading Styles ─────────────────────────────────────────────────────────

pub struct HeadingStyle {
    pub size: f32,
    pub color: [u8; 4],
    pub bg_color: Option<[u8; 4]>,
    pub tracking: f32,
    pub pad_x: u32,
    pub pad_top: u32,
    pub pad_bottom: u32,
}

pub fn heading_style(level: u8) -> HeadingStyle {
    match level {
        // H1: yellow on purple (#ffff87 on #5f5fff)
        1 => HeadingStyle {
            size: 96.0,
            color: [255, 255, 135, 255],
            bg_color: Some([95, 95, 255, 255]),
            tracking: 2.5,
            pad_x: 20,
            pad_top: 4,
            pad_bottom: 2,
        },
        // H2: deep sky blue (#00afff)
        2 => HeadingStyle {
            size: 72.0,
            color: [0, 175, 255, 255],
            bg_color: None,
            tracking: 1.5,
            pad_x: 2,
            pad_top: 2,
            pad_bottom: 1,
        },
        // H3+
        _ => HeadingStyle {
            size: 56.0,
            color: [0, 175, 255, 255],
            bg_color: None,
            tracking: 1.0,
            pad_x: 2,
            pad_top: 1,
            pad_bottom: 1,
        },
    }
}

// ─── Layout ─────────────────────────────────────────────────────────────────

pub const MARGIN: &str = "    ";
pub const MARGIN_WIDTH: usize = 4;

// ─── ANSI Escape Codes ──────────────────────────────────────────────────────

pub const BOLD_ON: &str = "\x1b[1m";
pub const ITALIC_ON: &str = "\x1b[3m";
pub const ITALIC_OFF: &str = "\x1b[23m";
pub const DIM_ON: &str = "\x1b[2m";
pub const STRIKETHROUGH_ON: &str = "\x1b[9m";
pub const STRIKETHROUGH_OFF: &str = "\x1b[29m";
pub const UNDERLINE_ON: &str = "\x1b[4m";
pub const UNDERLINE_OFF: &str = "\x1b[24m";
pub const RESET: &str = "\x1b[0m";

// Note: ANSI SGR 22 turns off BOTH bold and dim simultaneously.
// There is no way to turn off only one. Use RESET to safely end
// sections where bold and dim might overlap.

pub const LINK_COLOR: &str = "\x1b[36m";      // cyan
pub const CODE_BG: &str = "\x1b[48;5;236m";   // dark gray background
pub const CODE_FG: &str = "\x1b[38;5;213m";   // pink/magenta text
pub const QUOTE_BAR: &str = "\x1b[38;5;240m"; // gray
pub const QUOTE_TEXT: &str = "\x1b[3;38;5;250m"; // italic light gray
pub const URL_COLOR: &str = "\x1b[38;5;245m"; // subdued gray for link URLs

// ─── ANSI Utilities ─────────────────────────────────────────────────────────

/// Strip ANSI escape sequences from a string.
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.peek() {
                Some('[') => {
                    // CSI sequence: \x1b[ ... <letter>
                    chars.next();
                    for nc in chars.by_ref() {
                        if nc.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
                Some('_') | Some(']') => {
                    // APC / OSC: consume until ST (\x1b\\)
                    chars.next();
                    while let Some(nc) = chars.next() {
                        if nc == '\x1b' && chars.peek() == Some(&'\\') {
                            chars.next();
                            break;
                        }
                    }
                }
                _ => {
                    // Unknown escape — skip next char
                    chars.next();
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Calculate the display width of a string, ignoring ANSI codes and
/// counting CJK double-width characters as 2.
pub fn display_width(s: &str) -> usize {
    strip_ansi(s)
        .chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
        .sum()
}
