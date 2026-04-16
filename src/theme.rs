use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Theme::Dark => write!(f, "dark"),
            Theme::Light => write!(f, "light"),
        }
    }
}

/// Detect whether the terminal has a dark or light background via OSC 11.
///
/// Opens `/dev/tty` directly so detection works even when stdin is piped.
/// Returns `Dark` if detection fails for any reason.
#[cfg(unix)]
pub fn detect() -> Theme {
    use std::io::{Read, Write};
    use std::os::unix::io::AsRawFd;

    // Open the controlling terminal directly — works even when stdin is piped.
    let mut tty = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
    {
        Ok(f) => f,
        Err(_) => return Theme::Dark,
    };

    let fd = tty.as_raw_fd();

    // Save terminal state and switch to raw mode for reading the response.
    let mut saved: libc::termios = unsafe { std::mem::zeroed() };
    unsafe {
        libc::tcgetattr(fd, &mut saved);
    }
    let mut raw = saved;
    raw.c_lflag &= !(libc::ICANON | libc::ECHO);
    raw.c_cc[libc::VMIN] = 0;
    raw.c_cc[libc::VTIME] = 1; // 100 ms inter-byte timeout
    unsafe {
        libc::tcsetattr(fd, libc::TCSANOW, &raw);
    }

    // Send OSC 11 query (request background color).
    let _ = tty.write_all(b"\x1b]11;?\x1b\\");
    let _ = tty.flush();

    // Read response — terminals reply with something like:
    //   \x1b]11;rgb:RRRR/GGGG/BBBB\x1b\\
    let mut buf = [0u8; 64];
    let mut response = Vec::new();
    let start = std::time::Instant::now();

    while start.elapsed() < std::time::Duration::from_millis(200) {
        match tty.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                response.extend_from_slice(&buf[..n]);
                // Complete when we see String Terminator or BEL.
                if response.ends_with(b"\x1b\\") || response.contains(&0x07) {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    // Restore terminal state.
    unsafe {
        libc::tcsetattr(fd, libc::TCSANOW, &saved);
    }

    parse_osc11_response(&response)
}

#[cfg(not(unix))]
pub fn detect() -> Theme {
    Theme::Dark
}

/// Parse an OSC 11 response and classify the background as dark or light.
fn parse_osc11_response(data: &[u8]) -> Theme {
    let s = String::from_utf8_lossy(data);

    // Look for "rgb:RRRR/GGGG/BBBB" — each component is 1-4 hex digits.
    let prefix = "rgb:";
    let rgb_start = match s.find(prefix) {
        Some(pos) => pos + prefix.len(),
        None => return Theme::Dark,
    };

    let rest = &s[rgb_start..];
    let rgb_end = rest
        .find(|c: char| c == '\x1b' || c == '\x07')
        .unwrap_or(rest.len());
    let rgb_str = &rest[..rgb_end];

    let parts: Vec<&str> = rgb_str.split('/').collect();
    if parts.len() != 3 {
        return Theme::Dark;
    }

    let norm = |s: &str| -> Option<f64> {
        let val = u16::from_str_radix(s, 16).ok()?;
        let max = match s.len() {
            1 => 0xF,
            2 => 0xFF,
            3 => 0xFFF,
            4 => 0xFFFF,
            _ => return None,
        };
        Some(val as f64 / max as f64)
    };

    let r = match norm(parts[0]) {
        Some(v) => v,
        None => return Theme::Dark,
    };
    let g = match norm(parts[1]) {
        Some(v) => v,
        None => return Theme::Dark,
    };
    let b = match norm(parts[2]) {
        Some(v) => v,
        None => return Theme::Dark,
    };

    // Perceived luminance (ITU-R BT.601).
    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    if luminance > 0.5 {
        Theme::Light
    } else {
        Theme::Dark
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_dark_from_black_background() {
        // rgb:0000/0000/0000 → dark
        let resp = b"\x1b]11;rgb:0000/0000/0000\x1b\\";
        assert_eq!(parse_osc11_response(resp), Theme::Dark);
    }

    #[test]
    fn detects_light_from_white_background() {
        // rgb:ffff/ffff/ffff → light
        let resp = b"\x1b]11;rgb:ffff/ffff/ffff\x1b\\";
        assert_eq!(parse_osc11_response(resp), Theme::Light);
    }

    #[test]
    fn detects_light_from_short_hex() {
        // Some terminals use 2-digit hex: rgb:ff/ff/ff
        let resp = b"\x1b]11;rgb:ff/ff/ff\x07";
        assert_eq!(parse_osc11_response(resp), Theme::Light);
    }

    #[test]
    fn detects_dark_from_typical_dark_theme() {
        // Ghostty dark default ≈ rgb:1c1c/1c1c/1c1c
        let resp = b"\x1b]11;rgb:1c1c/1c1c/1c1c\x1b\\";
        assert_eq!(parse_osc11_response(resp), Theme::Dark);
    }

    #[test]
    fn detects_light_from_solarized_light() {
        // Solarized Light bg ≈ rgb:fdf6/e3e3/0000 — luminance ≈ 0.95
        let resp = b"\x1b]11;rgb:fdf6/e3e3/c7c7\x1b\\";
        assert_eq!(parse_osc11_response(resp), Theme::Light);
    }

    #[test]
    fn falls_back_to_dark_on_empty_response() {
        assert_eq!(parse_osc11_response(b""), Theme::Dark);
    }

    #[test]
    fn falls_back_to_dark_on_garbage() {
        assert_eq!(parse_osc11_response(b"not a real response"), Theme::Dark);
    }
}
