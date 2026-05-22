use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub font: FontSection,

    /// Theme override: "dark", "light", or "auto" (default).
    pub theme: Option<String>,

    /// Vim-style edge bell: emit a terminal BEL when the user tries to scroll
    /// past the top or bottom of the document. The terminal emulator decides
    /// the visible effect (audible beep, title-bar 🔔, dock bounce, …) — see
    /// e.g. Ghostty's `bell-features`. `None` means default (on). CLI
    /// `--no-bell` overrides to `Some(false)`.
    pub bell: Option<bool>,
}

#[derive(Deserialize, Default, Clone)]
pub struct FontSection {
    #[serde(default)]
    pub heading: HeadingFontConfig,
}

#[derive(Deserialize, Default, Clone)]
pub struct HeadingFontConfig {
    /// Font for Latin / ASCII text (sans-serif recommended, e.g. "Inter").
    pub latin: Option<String>,
    /// Font for CJK text (e.g. "LXGW WenKai").
    pub cjk: Option<String>,
    /// Font for emoji and symbol glyphs in image-rendered headings.
    pub emoji: Option<String>,
}

fn config_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?;
    Some(PathBuf::from(home).join(".termdown"))
}

pub fn load() -> Config {
    let path = match config_dir() {
        Some(dir) => dir.join("config.toml"),
        None => return Config::default(),
    };

    match std::fs::read_to_string(&path) {
        Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Warning: invalid config at {}: {}", path.display(), e);
            Config::default()
        }),
        Err(_) => Config::default(),
    }
}
