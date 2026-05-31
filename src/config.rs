use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub font: FontSection,

    /// Color theme. `None` (key absent) means `Auto`. An unrecognized value is
    /// a hard parse error surfaced by `load`, not a silent fallback to auto.
    pub theme: Option<ThemeChoice>,

    /// Vim-style edge bell: emit a terminal BEL when the user tries to scroll
    /// past the top or bottom of the document. The terminal emulator decides
    /// the visible effect (audible beep, title-bar 🔔, dock bounce, …) — see
    /// e.g. Ghostty's `bell-features`. `None` means default (on). CLI
    /// `--no-bell` overrides to `Some(false)`.
    pub bell: Option<bool>,

    /// Whether to render frontmatter (YAML `---` / TOML `+++` metadata blocks).
    /// Mirrors `bell`: `None` (key absent) and `Some(true)` both render — the
    /// one-line summary in cat / TUI-folded, with the TUI `m` key to expand.
    /// `Some(false)` hides metadata entirely; parsing still runs so the block
    /// never leaks into body content.
    /// See `docs/adr/0001-metadata-block-handling.md`.
    pub metadata: Option<bool>,
}

/// Color theme selection. `Auto` (the default when the key is absent) detects
/// the terminal background via OSC 11; `Dark` / `Light` force a palette.
#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ThemeChoice {
    Auto,
    Dark,
    Light,
}

impl std::str::FromStr for ThemeChoice {
    type Err = ();

    /// Parse a CLI `--theme` value. Mirrors the serde `rename_all = "lowercase"`
    /// mapping so the flag and the config file accept exactly the same names.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "dark" => Ok(Self::Dark),
            "light" => Ok(Self::Light),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct FontSection {
    #[serde(default)]
    pub heading: HeadingFontConfig,
}

#[derive(Deserialize, Default, Clone)]
#[serde(deny_unknown_fields)]
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
        assert_eq!(parsed.theme, Some(ThemeChoice::Auto));
        assert_eq!(parsed.bell, Some(true));
        // `metadata` mirrors `bell`: the example spells out the effective
        // default explicitly as `Some(true)` (a missing key parses as `None`,
        // which is also treated as "show").
        assert_eq!(parsed.metadata, Some(true));
        // Font overrides are commented out, so they must parse as unset.
        assert!(parsed.font.heading.latin.is_none());
        assert!(parsed.font.heading.cjk.is_none());
        assert!(parsed.font.heading.emoji.is_none());
    }

    /// A valid `theme` deserializes to the matching enum; the field is optional.
    #[test]
    fn theme_parses_known_values_and_defaults_to_none() {
        let parsed: Config = toml::from_str("theme = \"dark\"").unwrap();
        assert_eq!(parsed.theme, Some(ThemeChoice::Dark));
        assert_eq!(Config::default().theme, None);
    }

    /// An unrecognized `theme` is a hard error, not a silent fallback to auto —
    /// `load` turns this into a one-line warning instead of dropping it.
    #[test]
    fn invalid_theme_value_is_rejected() {
        assert!(toml::from_str::<Config>("theme = \"drak\"").is_err());
    }

    /// `deny_unknown_fields` catches typo'd keys (e.g. `bel` for `bell`) rather
    /// than silently ignoring them.
    #[test]
    fn unknown_top_level_key_is_rejected() {
        assert!(toml::from_str::<Config>("bel = false").is_err());
    }

    /// The CLI `--theme` parser accepts exactly the config file's value set.
    #[test]
    fn theme_choice_from_str_matches_serde_names() {
        use std::str::FromStr;
        assert_eq!(ThemeChoice::from_str("auto"), Ok(ThemeChoice::Auto));
        assert_eq!(ThemeChoice::from_str("dark"), Ok(ThemeChoice::Dark));
        assert_eq!(ThemeChoice::from_str("light"), Ok(ThemeChoice::Light));
        assert!(ThemeChoice::from_str("Dark").is_err());
        assert!(ThemeChoice::from_str("blue").is_err());
    }
}
