use std::path::Path;
use std::process::{Command, Stdio};

pub fn binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_termdown")
}

/// Run the compiled termdown binary against `path` in a controlled test
/// environment: ghostty-like terminal (so kitty graphics emission is enabled),
/// dark theme, and `HOME`/`USERPROFILE`/`XDG_CONFIG_HOME` cleared so a
/// developer's `~/.config/termdown/config.toml` can't leak into the test.
/// Returns raw stdout
/// bytes; callers decide whether to treat it as UTF-8 or scan for kitty APC
/// payloads.
pub fn run_termdown(path: &Path) -> Vec<u8> {
    let out = Command::new(binary_path())
        .arg("--theme")
        .arg("dark")
        .arg(path)
        .env("TERM_PROGRAM", "ghostty")
        .env_remove("HOME")
        .env_remove("USERPROFILE")
        .env_remove("XDG_CONFIG_HOME")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("termdown should run");
    assert!(
        out.status.success(),
        "termdown failed on {path:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}
