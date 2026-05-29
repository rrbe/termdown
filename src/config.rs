use std::path::{Path, PathBuf};

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

    #[serde(default)]
    pub metadata: MetadataSection,
}

#[derive(Deserialize, Clone)]
pub struct MetadataSection {
    /// Whether to render frontmatter (YAML `---` / TOML `+++` metadata blocks).
    /// `true` (default) shows the one-line summary in cat / TUI-folded, and
    /// allows the TUI `m` key to expand. `false` hides metadata entirely;
    /// parsing still runs so the block never leaks into body content.
    /// See `docs/adr/0001-metadata-block-handling.md`.
    #[serde(default = "default_metadata_show")]
    pub show: bool,
}

fn default_metadata_show() -> bool {
    true
}

impl Default for MetadataSection {
    fn default() -> Self {
        Self {
            show: default_metadata_show(),
        }
    }
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
    // XDG Base Directory spec: prefer an absolute $XDG_CONFIG_HOME, otherwise
    // fall back to ~/.config (USERPROFILE covers Windows, where HOME is often
    // unset). The config file itself lives at `<dir>/termdown/config.toml`.
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        let xdg = PathBuf::from(xdg);
        if xdg.is_absolute() {
            return Some(xdg.join("termdown"));
        }
    }
    let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?;
    Some(PathBuf::from(home).join(".config").join("termdown"))
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
        Err(_) => {
            warn_if_legacy_config(&path);
            Config::default()
        }
    }
}

/// Nudge users upgrading from the pre-XDG layout: if nothing was found at the
/// new path but a config still sits at the old `~/.termdown/config.toml`, warn
/// so the settings aren't silently dropped. Stays silent on a clean install
/// (no legacy file) and matches `uninstall.sh`'s legacy-cleanup awareness.
fn warn_if_legacy_config(new_path: &Path) {
    let home = match std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
        Some(home) => home,
        None => return,
    };
    let legacy = PathBuf::from(home).join(".termdown").join("config.toml");
    if legacy.is_file() {
        eprintln!(
            "Warning: ignoring legacy config at {}; move it to {}",
            legacy.display(),
            new_path.display()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `config.example.toml` is the user-facing source of truth for defaults,
    /// so guard it against bit-rot: it must always parse, and its explicit
    /// values must match what termdown treats as the built-in defaults.
    #[test]
    fn example_config_parses_and_matches_defaults() {
        let example = include_str!("../config.example.toml");
        let parsed: Config =
            toml::from_str(example).expect("config.example.toml must remain valid TOML");

        // The example spells out the *effective* defaults explicitly.
        assert_eq!(parsed.theme.as_deref(), Some("auto"));
        assert_eq!(parsed.bell, Some(true));
        // `metadata.show` is a real struct default — assert the example tracks it.
        assert_eq!(parsed.metadata.show, Config::default().metadata.show);
        assert!(parsed.metadata.show);
        // Font overrides are commented out, so they must parse as unset.
        assert!(parsed.font.heading.latin.is_none());
        assert!(parsed.font.heading.cjk.is_none());
        assert!(parsed.font.heading.emoji.is_none());
    }
}
