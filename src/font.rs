use std::collections::HashMap;
use std::fs;
use std::sync::{Mutex, OnceLock};

use ab_glyph::FontRef;
use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::{Properties, Stretch, Style, Weight};
use font_kit::source::SystemSource;

use crate::config::Config;

const FALLBACK_FONT: &[u8] = include_bytes!("../fonts/SourceSerif4-SemiBold.ttf");

// ─── Platform Default Fonts ─────────────────────────────────────────────────

fn preferred_latin_families() -> &'static [&'static str] {
    #[cfg(target_os = "macos")]
    {
        &["Avenir", "Avenir Next", "Futura", "Helvetica Neue"]
    }
    #[cfg(target_os = "linux")]
    {
        &["Inter", "Noto Sans", "DejaVu Sans", "Liberation Sans"]
    }
    #[cfg(target_os = "windows")]
    {
        &["Segoe UI", "Arial", "Verdana"]
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        &[]
    }
}

fn preferred_cjk_families() -> &'static [&'static str] {
    #[cfg(target_os = "macos")]
    {
        &[
            "Noto Serif CJK SC",
            "Source Han Serif SC",
            "Songti SC",
            "STSong",
        ]
    }
    #[cfg(target_os = "linux")]
    {
        &[
            "Noto Serif CJK SC",
            "Source Han Serif SC",
            "Noto Serif",
            "DejaVu Serif",
        ]
    }
    #[cfg(target_os = "windows")]
    {
        &["SimSun", "KaiTi", "Microsoft YaHei"]
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        &[]
    }
}

// ─── CJK Detection ─────────────────────────────────────────────────────────

/// Returns `true` if the character should be rendered with the CJK font.
pub fn is_cjk(ch: char) -> bool {
    let cp = ch as u32;
    matches!(
        cp,
        0x2E80..=0x9FFF     // CJK Radicals … Unified Ideographs (includes Hiragana, Katakana, CJK Symbols)
        | 0xAC00..=0xD7AF   // Hangul Syllables
        | 0xF900..=0xFAFF   // CJK Compatibility Ideographs
        | 0xFE30..=0xFE4F   // CJK Compatibility Forms
        | 0xFF00..=0xFFEF   // Halfwidth and Fullwidth Forms
        | 0x20000..=0x2FA1F // CJK Extensions B-F, Supplement
    )
}

// ─── Font Pair ──────────────────────────────────────────────────────────────

/// A pair of fonts: one for Latin/ASCII text, one for CJK text.
pub struct FontPair {
    pub latin: FontRef<'static>,
    pub cjk: FontRef<'static>,
}

impl FontPair {
    /// Pick the appropriate font for a character.
    pub fn for_char(&self, ch: char) -> &FontRef<'static> {
        if is_cjk(ch) {
            &self.cjk
        } else {
            &self.latin
        }
    }
}

// ─── Font Loading ───────────────────────────────────────────────────────────

fn weight_for_level(level: u8) -> Weight {
    match level {
        1 | 2 => Weight::BLACK,
        3 => Weight::EXTRA_BOLD,
        _ => Weight::BOLD,
    }
}

static FONT_DATA_CACHE: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();

fn load_from_handle(handle: &Handle) -> Option<FontRef<'static>> {
    let cache = FONT_DATA_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().ok()?;

    match handle {
        Handle::Path { path, font_index } => {
            let key = path.to_string_lossy().into_owned();
            let data = if let Some(existing) = map.get(&key) {
                *existing
            } else {
                let bytes = fs::read(path).ok()?;
                let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
                map.insert(key, leaked);
                leaked
            };
            FontRef::try_from_slice_and_index(data, *font_index).ok()
        }
        Handle::Memory { bytes, font_index } => {
            let key = format!("mem:{}:{}", bytes.as_ptr() as usize, font_index);
            let data = if let Some(existing) = map.get(&key) {
                *existing
            } else {
                let leaked: &'static [u8] = Box::leak(bytes.to_vec().into_boxed_slice());
                map.insert(key, leaked);
                leaked
            };
            FontRef::try_from_slice_and_index(data, *font_index).ok()
        }
    }
}

fn try_load(source: &SystemSource, family: &str, props: &Properties) -> Option<FontRef<'static>> {
    let handle = source
        .select_best_match(&[FamilyName::Title(family.to_string())], props)
        .ok()?;
    load_from_handle(&handle)
}

/// Try to load a font at the requested weight.
///
/// macOS registers bold variants as separate families (e.g. "Avenir Black"),
/// so requesting "Avenir" with Weight::BLACK may return the regular weight.
/// We work around this by trying explicit variant family names first.
fn try_font(source: &SystemSource, family: &str, props: &Properties) -> Option<FontRef<'static>> {
    // For bold weights, try explicit variant family names first
    let suffixes: &[&str] = if props.weight.0 >= Weight::BLACK.0 {
        &["Black", "Heavy"]
    } else if props.weight.0 >= Weight::EXTRA_BOLD.0 {
        &["ExtraBold", "Heavy", "Bold"]
    } else if props.weight.0 >= Weight::BOLD.0 {
        &["Bold", "Demi Bold"]
    } else {
        &[]
    };

    let normal_props = Properties {
        weight: Weight::NORMAL,
        ..*props
    };

    for suffix in suffixes {
        let variant = format!("{family} {suffix}");
        if let Some(font) = try_load(source, &variant, &normal_props) {
            return Some(font);
        }
    }

    // Fall back to standard weight matching
    try_load(source, family, props)
}

fn resolve_font(
    source: &SystemSource,
    props: &Properties,
    user_choice: Option<&str>,
    platform_defaults: &[&str],
) -> Option<FontRef<'static>> {
    if let Some(family) = user_choice {
        if let Some(font) = try_font(source, family, props) {
            return Some(font);
        }
    }
    for family in platform_defaults {
        if let Some(font) = try_font(source, family, props) {
            return Some(font);
        }
    }
    FontRef::try_from_slice(FALLBACK_FONT).ok()
}

/// Resolve a Latin + CJK font pair for the given heading level.
pub fn get_fonts(level: u8, config: &Config) -> Option<FontPair> {
    let source = SystemSource::new();
    let props = Properties {
        style: Style::Normal,
        weight: weight_for_level(level),
        stretch: Stretch::NORMAL,
    };

    let latin = resolve_font(
        &source,
        &props,
        config.font.heading.latin.as_deref(),
        preferred_latin_families(),
    )?;

    let cjk = resolve_font(
        &source,
        &props,
        config.font.heading.cjk.as_deref(),
        preferred_cjk_families(),
    )?;

    Some(FontPair { latin, cjk })
}
