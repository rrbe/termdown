use unicode_width::UnicodeWidthChar;

use crate::theme::Theme;

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

pub fn heading_style(level: u8, theme: Theme) -> HeadingStyle {
    match (level, theme) {
        // ── Dark theme ──────────────────────────────────────────
        // H1: yellow on purple (#ffff87 on #5f5fff)
        (1, Theme::Dark) => HeadingStyle {
            size: 96.0,
            color: [255, 255, 135, 255],
            bg_color: Some([95, 95, 255, 255]),
            tracking: 2.5,
            pad_x: 20,
            pad_top: 4,
            pad_bottom: 2,
        },
        // H2: deep sky blue (#00afff), transparent bg
        (2, Theme::Dark) => HeadingStyle {
            size: 72.0,
            color: [0, 175, 255, 255],
            bg_color: None,
            tracking: 1.5,
            pad_x: 2,
            pad_top: 2,
            pad_bottom: 1,
        },
        // H3+: deep sky blue (#00afff), transparent bg
        (_, Theme::Dark) => HeadingStyle {
            size: 56.0,
            color: [0, 175, 255, 255],
            bg_color: None,
            tracking: 1.0,
            pad_x: 2,
            pad_top: 1,
            pad_bottom: 1,
        },

        // ── Light theme ─────────────────────────────────────────
        // H1: white on indigo (#ffffff on #4338ca)
        (1, Theme::Light) => HeadingStyle {
            size: 96.0,
            color: [255, 255, 255, 255],
            bg_color: Some([67, 56, 202, 255]),
            tracking: 2.5,
            pad_x: 20,
            pad_top: 4,
            pad_bottom: 2,
        },
        // H2: dark blue (#0050a0), transparent bg
        (2, Theme::Light) => HeadingStyle {
            size: 72.0,
            color: [0, 80, 160, 255],
            bg_color: None,
            tracking: 1.5,
            pad_x: 2,
            pad_top: 2,
            pad_bottom: 1,
        },
        // H3+: dark blue (#0050a0), transparent bg
        (_, Theme::Light) => HeadingStyle {
            size: 56.0,
            color: [0, 80, 160, 255],
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

// ─── Theme-Aware Colors ────────────────────────────────────────────────────

pub struct Colors {
    pub link: &'static str,
    pub code_fg: &'static str,
    pub code_bg: &'static str,
    pub quote_bar: &'static str,
    pub quote_text: &'static str,
    pub url: &'static str,
}

impl Colors {
    pub fn for_theme(theme: Theme) -> Self {
        match theme {
            Theme::Dark => Self {
                link: "\x1b[36m",               // cyan
                code_fg: "\x1b[38;5;213m",      // pink/magenta
                code_bg: "\x1b[48;5;236m",      // dark gray background
                quote_bar: "\x1b[38;5;240m",    // gray
                quote_text: "\x1b[3;38;5;250m", // italic light gray
                url: "\x1b[38;5;245m",          // subdued gray
            },
            Theme::Light => Self {
                link: "\x1b[38;5;26m",          // dark blue
                code_fg: "\x1b[38;5;125m",      // dark magenta
                code_bg: "\x1b[48;5;253m",      // light gray background
                quote_bar: "\x1b[38;5;243m",    // medium gray
                quote_text: "\x1b[3;38;5;242m", // italic medium-dark gray
                url: "\x1b[38;5;241m",          // dark gray
            },
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_ansi_removes_csi_osc_and_apc_sequences() {
        let text = format!(
            "a{BOLD_ON}b{RESET}\x1b]8;;https://example.com\x1b\\link\x1b]8;;\x1b\\c\x1b_hidden\x1b\\d"
        );

        assert_eq!(strip_ansi(&text), "ablinkcd");
    }

    #[test]
    fn display_width_ignores_ansi_and_counts_wide_chars() {
        let text = format!("{BOLD_ON}\u{597D}{RESET}a");

        assert_eq!(display_width(&text), 3);
    }
}
