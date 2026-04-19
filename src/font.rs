use std::collections::HashMap;
use std::fs;
use std::sync::{Mutex, OnceLock};

use ab_glyph::{Font, FontRef, GlyphId};
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

fn preferred_emoji_families() -> &'static [&'static str] {
    #[cfg(target_os = "macos")]
    {
        &["Apple Color Emoji", "Apple Symbols", "Symbol"]
    }
    #[cfg(target_os = "linux")]
    {
        &[
            "Noto Color Emoji",
            "Noto Emoji",
            "Emoji One Color",
            "Symbola",
            "DejaVu Sans",
        ]
    }
    #[cfg(target_os = "windows")]
    {
        &["Segoe UI Emoji", "Segoe UI Symbol"]
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

pub fn is_emoji_like(ch: char) -> bool {
    let cp = ch as u32;
    matches!(
        cp,
        0x2190..=0x21FF
        | 0x2300..=0x23FF
        | 0x2460..=0x24FF
        | 0x2600..=0x27FF
        | 0x2900..=0x297F
        | 0x2B00..=0x2BFF
        | 0x1F000..=0x1FAFF
    )
}

// ─── Font Set ───────────────────────────────────────────────────────────────

/// Fonts used for image-rendered headings.
pub struct FontSet {
    pub latin: FontRef<'static>,
    pub cjk: FontRef<'static>,
    pub emoji: Option<FontRef<'static>>,
}

pub struct GlyphFont<'a> {
    pub font: &'a FontRef<'static>,
    pub glyph_id: GlyphId,
}

impl FontSet {
    fn has_renderable_glyph(font: &FontRef<'static>, ch: char) -> Option<GlyphId> {
        let glyph_id = font.glyph_id(ch);
        if glyph_id.0 == 0 {
            return None;
        }
        if font.outline(glyph_id).is_some()
            || font.glyph_raster_image2(glyph_id, u16::MAX).is_some()
        {
            return Some(glyph_id);
        }
        None
    }

    fn try_font<'a>(font: &'a FontRef<'static>, ch: char) -> Option<GlyphFont<'a>> {
        Some(GlyphFont {
            font,
            glyph_id: Self::has_renderable_glyph(font, ch)?,
        })
    }

    /// Pick the best available font for a character based on script first,
    /// then fall back across all configured heading fonts.
    pub fn for_char(&self, ch: char) -> GlyphFont<'_> {
        if is_emoji_like(ch) {
            if let Some(font) = self
                .emoji
                .as_ref()
                .and_then(|font| Self::try_font(font, ch))
            {
                return font;
            }
        }

        if is_cjk(ch) {
            if let Some(font) = Self::try_font(&self.cjk, ch) {
                return font;
            }
            if let Some(font) = Self::try_font(&self.latin, ch) {
                return font;
            }
        } else {
            if let Some(font) = Self::try_font(&self.latin, ch) {
                return font;
            }
            if let Some(font) = Self::try_font(&self.cjk, ch) {
                return font;
            }
        }

        if let Some(font) = self
            .emoji
            .as_ref()
            .and_then(|font| Self::try_font(font, ch))
        {
            return font;
        }

        GlyphFont {
            font: &self.latin,
            glyph_id: self.latin.glyph_id(ch),
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

fn resolve_optional_font(
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
    None
}

/// Per-level cache of resolved font sets. `Config` is loaded once at startup
/// and is constant for the process lifetime, so a level → FontSet mapping is
/// safe to memoize. `SystemSource::new()` and the family-resolution walk are
/// the dominant cost in heading rasterization (~30–40ms each on macOS), and
/// without this cache they were re-paid for every H1–H3 in the document.
static FONT_SETS: [OnceLock<Option<FontSet>>; 6] = [
    OnceLock::new(),
    OnceLock::new(),
    OnceLock::new(),
    OnceLock::new(),
    OnceLock::new(),
    OnceLock::new(),
];

/// Resolve a Latin + CJK + optional emoji font set for the given heading level.
pub fn get_fonts(level: u8, config: &Config) -> Option<&'static FontSet> {
    let idx = level.clamp(1, 6).saturating_sub(1) as usize;
    FONT_SETS[idx]
        .get_or_init(|| resolve_font_set(level, config))
        .as_ref()
}

fn resolve_font_set(level: u8, config: &Config) -> Option<FontSet> {
    let source = SystemSource::new();
    let props = Properties {
        style: Style::Normal,
        weight: weight_for_level(level),
        stretch: Stretch::NORMAL,
    };
    let emoji_props = Properties {
        style: Style::Normal,
        weight: Weight::NORMAL,
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

    let emoji = resolve_optional_font(
        &source,
        &emoji_props,
        config.font.heading.emoji.as_deref(),
        preferred_emoji_families(),
    );

    Some(FontSet { latin, cjk, emoji })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emoji_font_prefers_renderable_emoji_glyph() {
        let config = Config::default();
        let fonts = get_fonts(1, &config).expect("fonts should resolve");
        let glyph = fonts.for_char('😀');
        let raster = glyph.font.glyph_raster_image2(glyph.glyph_id, u16::MAX);

        assert!(
            glyph.font.outline(glyph.glyph_id).is_some() || raster.is_some(),
            "selected font should be able to render 😀"
        );

        if let Some(emoji_font) = fonts.emoji.as_ref() {
            assert!(
                std::ptr::eq(glyph.font, emoji_font),
                "emoji-like code points should prefer the emoji font when available"
            );
        }
    }
}
