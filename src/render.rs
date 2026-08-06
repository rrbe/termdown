use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use ab_glyph::{point, Font, GlyphId, GlyphImageFormat, PxScale, ScaleFont};
use base64::Engine;
use image::codecs::png::PngEncoder;
use image::ImageFormat;
use image::{ImageEncoder, Rgba, RgbaImage};

use crate::config::Config;
use crate::font::{self, FontSet};
use crate::style;
use crate::theme::Theme;

// ─── Text Metrics ───────────────────────────────────────────────────────────

/// Width, height, and ascent of the measured text.
struct TextMetrics {
    width: f32,
    height: f32,
    ascent: f32,
}

fn is_ignorable_format_char(ch: char) -> bool {
    let cp = ch as u32;
    cp == 0x200D || matches!(cp, 0xFE00..=0xFE0F | 0xE0100..=0xE01EF)
}

fn measure_text(fonts: &FontSet, scale: PxScale, text: &str, tracking: f32) -> TextMetrics {
    let latin_s = fonts.latin.as_scaled(scale);
    let cjk_s = fonts.cjk.as_scaled(scale);
    let emoji_s = fonts.emoji.as_ref().map(|font| font.as_scaled(scale));

    // Use the tallest ascent and deepest descent across all loaded fonts.
    let ascent = emoji_s
        .as_ref()
        .map(|font| latin_s.ascent().max(cjk_s.ascent()).max(font.ascent()))
        .unwrap_or_else(|| latin_s.ascent().max(cjk_s.ascent()));
    let descent = emoji_s
        .as_ref()
        .map(|font| latin_s.descent().min(cjk_s.descent()).min(font.descent()))
        .unwrap_or_else(|| latin_s.descent().min(cjk_s.descent())); // descent is negative

    let chars: Vec<char> = text.chars().collect();
    let mut width = 0.0f32;

    for (i, &ch) in chars.iter().enumerate() {
        if is_ignorable_format_char(ch) {
            continue;
        }

        let glyph_font = fonts.for_char(ch);
        let scaled = glyph_font.font.as_scaled(scale);
        width += scaled.h_advance(glyph_font.glyph_id);
        if tracking > 0.0 && i + 1 < chars.len() {
            width += tracking;
        }
    }

    TextMetrics {
        width,
        height: ascent - descent,
        ascent,
    }
}

// ─── Glyph Drawing ─────────────────────────────────────────────────────────

fn draw_text(
    img: &mut RgbaImage,
    fonts: &FontSet,
    scale: PxScale,
    baseline: (f32, f32),
    text: &str,
    color: [u8; 4],
    tracking: f32,
) {
    let mut x = baseline.0;
    let y = baseline.1;

    for ch in text.chars() {
        if is_ignorable_format_char(ch) {
            continue;
        }

        let glyph_font = fonts.for_char(ch);
        let font = glyph_font.font;
        let scaled = font.as_scaled(scale);
        let glyph_id = glyph_font.glyph_id;
        let glyph = glyph_id.with_scale_and_position(scale, point(x, y));
        let prefer_raster = font::is_emoji_like(ch)
            && font
                .glyph_raster_image2(glyph_id, pixels_per_em(font, scale))
                .is_some();

        if prefer_raster {
            draw_raster_glyph(img, font, glyph_id, scale, (x, y), color);
        } else if let Some(outlined) = font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            outlined.draw(|gx, gy, coverage| {
                let px = bounds.min.x as i32 + gx as i32;
                let py = bounds.min.y as i32 + gy as i32;
                if px >= 0 && py >= 0 && (px as u32) < img.width() && (py as u32) < img.height() {
                    let alpha = (coverage * color[3] as f32) as u8;
                    if alpha == 0 {
                        return;
                    }
                    let pixel = img.get_pixel_mut(px as u32, py as u32);
                    let a = alpha as f32 / 255.0;
                    pixel[0] = (color[0] as f32 * a + pixel[0] as f32 * (1.0 - a)) as u8;
                    pixel[1] = (color[1] as f32 * a + pixel[1] as f32 * (1.0 - a)) as u8;
                    pixel[2] = (color[2] as f32 * a + pixel[2] as f32 * (1.0 - a)) as u8;
                    pixel[3] = ((alpha as f32 + pixel[3] as f32 * (1.0 - a)).min(255.0)) as u8;
                }
            });
        } else {
            draw_raster_glyph(img, font, glyph_id, scale, (x, y), color);
        }

        x += scaled.h_advance(glyph_id) + if tracking > 0.0 { tracking } else { 0.0 };
    }
}

fn pixels_per_em(font: &ab_glyph::FontRef<'static>, scale: PxScale) -> u16 {
    let height = font.height_unscaled().max(1.0);
    let units = font.units_per_em().unwrap_or(height).max(1.0);
    ((scale.y * units / height).round()).clamp(1.0, u16::MAX as f32) as u16
}

fn blend_rgba(dst: &mut Rgba<u8>, src: [u8; 4]) {
    let a = src[3] as f32 / 255.0;
    if a <= 0.0 {
        return;
    }
    dst[0] = (src[0] as f32 * a + dst[0] as f32 * (1.0 - a)) as u8;
    dst[1] = (src[1] as f32 * a + dst[1] as f32 * (1.0 - a)) as u8;
    dst[2] = (src[2] as f32 * a + dst[2] as f32 * (1.0 - a)) as u8;
    dst[3] = ((src[3] as f32 + dst[3] as f32 * (1.0 - a)).min(255.0)) as u8;
}

fn blend_premul_bgra(dst: &mut Rgba<u8>, src: [u8; 4]) {
    let a = src[3] as f32 / 255.0;
    if a <= 0.0 {
        return;
    }
    dst[0] = (src[2] as f32 + dst[0] as f32 * (1.0 - a)) as u8;
    dst[1] = (src[1] as f32 + dst[1] as f32 * (1.0 - a)) as u8;
    dst[2] = (src[0] as f32 + dst[2] as f32 * (1.0 - a)) as u8;
    dst[3] = ((src[3] as f32 + dst[3] as f32 * (1.0 - a)).min(255.0)) as u8;
}

fn raster_sample_coverage(
    format: &GlyphImageFormat,
    data: &[u8],
    width: u32,
    x: u32,
    y: u32,
) -> Option<u8> {
    let bits_per_pixel = match format {
        GlyphImageFormat::BitmapMono | GlyphImageFormat::BitmapMonoPacked => 1,
        GlyphImageFormat::BitmapGray2 | GlyphImageFormat::BitmapGray2Packed => 2,
        GlyphImageFormat::BitmapGray4 | GlyphImageFormat::BitmapGray4Packed => 4,
        GlyphImageFormat::BitmapGray8 => 8,
        _ => return None,
    };

    if bits_per_pixel == 8 {
        let index = (y * width + x) as usize;
        return data.get(index).copied();
    }

    let row_bits = match format {
        GlyphImageFormat::BitmapMono
        | GlyphImageFormat::BitmapGray2
        | GlyphImageFormat::BitmapGray4 => (width * bits_per_pixel).div_ceil(8) * 8,
        _ => width * bits_per_pixel,
    };
    let bit_index = y * row_bits + x * bits_per_pixel;
    let byte_index = (bit_index / 8) as usize;
    let shift = 8 - bits_per_pixel - (bit_index % 8);
    let mask = ((1u16 << bits_per_pixel) - 1) as u8;
    let value = (data.get(byte_index)? >> shift) & mask;

    Some(match bits_per_pixel {
        1 => {
            if value == 0 {
                0
            } else {
                255
            }
        }
        2 => value * 85,
        4 => value * 17,
        _ => 0,
    })
}

fn draw_raster_glyph(
    img: &mut RgbaImage,
    font: &ab_glyph::FontRef<'static>,
    glyph_id: GlyphId,
    scale: PxScale,
    baseline: (f32, f32),
    color: [u8; 4],
) {
    let target_ppem = pixels_per_em(font, scale);
    let raster = match font.glyph_raster_image2(glyph_id, target_ppem) {
        Some(raster) => raster,
        None => return,
    };

    let src_w = raster.width as u32;
    let src_h = raster.height as u32;
    if src_w == 0 || src_h == 0 {
        return;
    }

    let scale_factor = target_ppem as f32 / raster.pixels_per_em.max(1) as f32;
    let draw_w = ((src_w as f32 * scale_factor).round()).max(1.0) as u32;
    let draw_h = ((src_h as f32 * scale_factor).round()).max(1.0) as u32;
    let top_x = (baseline.0 + raster.origin.x * scale_factor).round() as i32;
    let top_y = (baseline.1 - font.as_scaled(scale).ascent() + raster.origin.y * scale_factor)
        .round() as i32;

    let png_image = match raster.format {
        GlyphImageFormat::Png => image::load_from_memory_with_format(raster.data, ImageFormat::Png)
            .ok()
            .map(|img| img.to_rgba8()),
        _ => None,
    };

    for dy in 0..draw_h {
        let sy = ((dy as f32) / scale_factor).floor().min((src_h - 1) as f32) as u32;
        let py = top_y + dy as i32;
        if py < 0 || py >= img.height() as i32 {
            continue;
        }

        for dx in 0..draw_w {
            let sx = ((dx as f32) / scale_factor).floor().min((src_w - 1) as f32) as u32;
            let px = top_x + dx as i32;
            if px < 0 || px >= img.width() as i32 {
                continue;
            }

            let dst = img.get_pixel_mut(px as u32, py as u32);

            if let Some(png) = png_image.as_ref() {
                let src = png.get_pixel(sx, sy).0;
                blend_rgba(dst, src);
                continue;
            }

            if matches!(raster.format, GlyphImageFormat::BitmapPremulBgra32) {
                let idx = ((sy * src_w + sx) * 4) as usize;
                if let Some(chunk) = raster.data.get(idx..idx + 4) {
                    blend_premul_bgra(dst, [chunk[0], chunk[1], chunk[2], chunk[3]]);
                }
                continue;
            }

            if let Some(coverage) =
                raster_sample_coverage(&raster.format, raster.data, src_w, sx, sy)
            {
                let alpha = ((coverage as u16 * color[3] as u16) / 255) as u8;
                blend_rgba(dst, [color[0], color[1], color[2], alpha]);
            }
        }
    }
}

// ─── Heading Rendering ──────────────────────────────────────────────────────

/// Process-global cache of rasterized heading PNGs, keyed on
/// `(level, theme, font fingerprint, text)`. Rasterization (glyph drawing + PNG
/// encoding) is the dominant cost of a render, so live-reload re-renders only
/// headings whose text actually changed — everything else is served from here.
/// The configured heading fonts are folded into the key via
/// [`font_fingerprint`]: a normal CLI run has one constant config, but keying on
/// it keeps the global cache correct if the same text is ever rendered under
/// different font configs in one process (e.g. shared test runs, or future
/// library use). Mirrors the `FONT_DATA_CACHE` pattern in `font.rs`.
/// Rasterized heading: PNG bytes plus pixel `(width, height)`.
type RenderedHeading = (Vec<u8>, u32, u32);
/// `(level, theme, font fingerprint, text)` — see [`HEADING_CACHE`].
type HeadingCacheKey = (u8, Theme, String, String);
static HEADING_CACHE: OnceLock<Mutex<HashMap<HeadingCacheKey, RenderedHeading>>> = OnceLock::new();

/// Soft cap so a long editing session that churns through many distinct
/// heading texts can't grow the cache without bound. Each entry is a few KB.
const HEADING_CACHE_CAP: usize = 1024;

/// Fold the configured heading fonts into a single stable string so the heading
/// cache key distinguishes renders made under different font configs. The unit
/// separator (`\u{1f}`) can't appear in a font name, so the three slots never
/// collide. Resolved platform defaults aren't included: they're constant for
/// the process, so an all-unset config is correctly shared.
fn font_fingerprint(config: &Config) -> String {
    let h = &config.font.heading;
    format!(
        "{}\u{1f}{}\u{1f}{}",
        h.latin.as_deref().unwrap_or(""),
        h.cjk.as_deref().unwrap_or(""),
        h.emoji.as_deref().unwrap_or(""),
    )
}

/// Render heading text to a PNG image. Returns `None` if font loading or
/// PNG encoding fails (caller should fall back to ANSI text).
///
/// Returns the PNG bytes and the image's pixel dimensions `(width, height)`.
/// The TUI needs the pixel height to compute how many terminal rows the image
/// occupies (`ceil(height / cell_pixel_height)`); cat mode ignores it.
///
/// Results are memoized in [`HEADING_CACHE`]; a cache hit skips all
/// rasterization. The lock is held only for the brief get / insert, never
/// across the expensive rasterization, so distinct headings still rasterize in
/// parallel under `layout::build`'s rayon `par_iter`.
pub fn render_heading(
    text: &str,
    level: u8,
    config: &Config,
    theme: Theme,
) -> Option<(Vec<u8>, u32, u32)> {
    let cache = HEADING_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let key: HeadingCacheKey = (level, theme, font_fingerprint(config), text.to_owned());
    if let Ok(map) = cache.lock() {
        if let Some(hit) = map.get(&key) {
            return Some(hit.clone());
        }
    }

    let st = style::heading_style(level, theme);
    let fonts = font::get_fonts(level, config)?;
    let scale = PxScale {
        x: st.size,
        y: st.size,
    };

    let metrics = measure_text(fonts, scale, text, st.tracking);

    let img_w = (metrics.width as u32 + st.pad_x * 2).max(1);
    let img_h = (metrics.height as u32 + st.pad_top + st.pad_bottom).max(1);

    let mut img = RgbaImage::new(img_w, img_h);

    if let Some(bg) = st.bg_color {
        for pixel in img.pixels_mut() {
            *pixel = Rgba(bg);
        }
    }

    let baseline_y = st.pad_top as f32 + metrics.ascent;
    draw_text(
        &mut img,
        fonts,
        scale,
        (st.pad_x as f32, baseline_y),
        text,
        st.color,
        st.tracking,
    );

    let mut buf = Vec::new();
    PngEncoder::new(&mut buf)
        .write_image(img.as_raw(), img_w, img_h, image::ExtendedColorType::Rgba8)
        .ok()?;

    let result = (buf, img_w, img_h);
    if let Ok(mut map) = cache.lock() {
        if map.len() >= HEADING_CACHE_CAP {
            map.clear();
        }
        map.insert(key, result.clone());
    }
    Some(result)
}

// ─── Kitty Graphics Protocol ────────────────────────────────────────────────

/// Discard any pending bytes on stdin without blocking. Cheap; safe to call
/// unconditionally at end of cat-mode.
#[cfg(unix)]
pub fn flush_stdin() {
    // SAFETY: tcflush is a standard POSIX call on a process-owned fd.
    unsafe {
        libc::tcflush(libc::STDIN_FILENO, libc::TCIFLUSH);
    }
}

/// iTerm2 ignores the Kitty `q=2` response-suppression flag and emits `OK`
/// ACKs anyway. Wait briefly so those bytes reach the input buffer, then
/// discard them. Only call under iTerm2 — the 50ms sleep is wasted overhead
/// on terminals (Ghostty/Kitty/WezTerm) that respect `q=2`.
#[cfg(unix)]
pub fn drain_iterm2_acks() {
    std::thread::sleep(std::time::Duration::from_millis(50));
    flush_stdin();
}

/// Encode PNG data as a Kitty graphics protocol escape sequence.
pub fn kitty_display(png: &[u8]) -> String {
    use base64::engine::general_purpose::STANDARD;
    let b64 = STANDARD.encode(png);
    let mut out = String::new();
    let total = b64.len();
    let mut offset = 0;
    let mut first = true;

    while offset < total {
        let end = (offset + 4096).min(total);
        let chunk = &b64[offset..end];
        let m = if end == total { "0" } else { "1" };
        if first {
            first = false;
            out.push_str(&format!("\x1b_Gf=100,a=T,q=2,m={m};{chunk}\x1b\\"));
        } else {
            out.push_str(&format!("\x1b_Gm={m};{chunk}\x1b\\"));
        }
        offset = end;
    }

    out
}

// ─── Kitty Image Lifecycle Primitives ───────────────────────────────────────
//
// Unlike `kitty_display` which always transmits + displays in one go, these
// split the transmit + place + delete operations so the TUI can cache images
// on the terminal and re-position them cheaply on scroll.

use std::io::Write;

/// Transmit PNG data to the terminal and cache it under `id`. The image is
/// not displayed yet; call `place` afterwards.
pub fn transmit<W: Write>(w: &mut W, id: u32, png: &[u8]) -> std::io::Result<()> {
    use base64::engine::general_purpose::STANDARD;
    let b64 = STANDARD.encode(png);
    let total = b64.len();
    let mut offset = 0;
    let mut first = true;
    while offset < total {
        let end = (offset + 4096).min(total);
        let chunk = &b64[offset..end];
        let m = if end == total { "0" } else { "1" };
        if first {
            write!(w, "\x1b_Gf=100,a=t,i={id},q=2,m={m};{chunk}\x1b\\")?;
            first = false;
        } else {
            write!(w, "\x1b_Gm={m};{chunk}\x1b\\")?;
        }
        offset = end;
    }
    // Empty-payload edge case: still send a minimal terminating frame so the
    // terminal knows the transmission ended. For zero-length png this is only
    // reachable from test code; a real heading image is never empty.
    if total == 0 {
        write!(w, "\x1b_Gf=100,a=t,i={id},q=2,m=0;\x1b\\")?;
    }
    Ok(())
}

/// Place previously-transmitted image `id` at the given cell coordinates.
/// Moves the cursor explicitly because Kitty's `a=p` places at the current
/// cursor position; the `x` and `y` APC keys are source-image pixel offsets
/// (for cropping), not terminal cell coordinates.
///
/// The placement id matches the globally-unique image id. Reusing that pair
/// moves the existing placement instead of stacking another copy.
///
/// The `C=1` flag tells Kitty to NOT advance the cursor after placement.
/// Without it, placing a tall image near the bottom of the screen would
/// push the cursor past the last row, causing the terminal to scroll the
/// entire visible region upward — this snowballs across frames and
/// produces the "status bar creeps upward on each scroll" failure.
pub fn place<W: Write>(w: &mut W, id: u32, col: u16, row: u16) -> std::io::Result<()> {
    // CUP (cursor position) is 1-indexed: row+1, col+1.
    write!(
        w,
        "\x1b[{};{}H\x1b_Ga=p,i={id},p={id},C=1,q=2;\x1b\\",
        row + 1,
        col + 1
    )
}

/// Delete a single placement of `id`. Keeps the cached image data so future
/// `place` calls on the same id are cheap.
pub fn delete_placement<W: Write>(w: &mut W, id: u32) -> std::io::Result<()> {
    write!(w, "\x1b_Ga=d,d=i,i={id},p={id},q=2;\x1b\\")
}

/// Delete all placements and image data this client has created. Used at
/// TUI exit to clean up the terminal.
pub fn delete_all_for_client<W: Write>(w: &mut W) -> std::io::Result<()> {
    write!(w, "\x1b_Ga=d,d=A,q=2;\x1b\\")
}

// ─── Shared Image Record ────────────────────────────────────────────────────

/// PNG data + vertical dimensions for a rendered heading image.
/// Stored by id in `RenderedDoc` and transmitted to the terminal
/// once per TUI session (or emitted directly when cat mode writes to a TTY).
///
/// `rows` is the number of terminal cell rows the image occupies. This is
/// a conservative estimate at layout time (based on heading level) and is
/// refined by `tui::mod::refine_image_rows` once the real terminal cell
/// pixel height is known. `px_height` preserves the exact PNG height so
/// the refinement step can compute `ceil(px_height / cell_pixel_height)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingImage {
    pub id: u32,
    pub png: Vec<u8>,
    pub rows: u16,
    pub px_height: u32,
}

#[cfg(test)]
mod kitty_tests {
    use super::*;

    #[test]
    fn transmit_produces_a_eq_t_with_id() {
        let mut buf = Vec::new();
        transmit(&mut buf, 42, b"\x89PNG\r\n").unwrap();
        let s = String::from_utf8(buf).unwrap();
        // Lowercase 'a=t' — transmit without displaying.
        assert!(s.starts_with("\x1b_Gf=100,a=t,i=42,q=2"));
        assert!(s.ends_with("\x1b\\"));
    }

    #[test]
    fn transmit_chunks_large_payload() {
        // 8 KB PNG-ish payload; base64 is ~10.6 KB → 3 chunks of 4 KB.
        let mut buf = Vec::new();
        let payload = vec![0u8; 8_000];
        transmit(&mut buf, 7, &payload).unwrap();
        let s = String::from_utf8(buf).unwrap();
        // Every chunk is an APC-wrapped escape: count \x1b_G occurrences.
        let chunk_count = s.matches("\x1b_G").count();
        assert!(
            chunk_count >= 2,
            "expected chunked transmission, got {chunk_count} escape(s)"
        );
        // Middle chunks use m=1, final uses m=0.
        assert!(s.contains(";") && s.ends_with("\x1b\\"));
    }

    #[test]
    fn display_transmits_and_displays() {
        let s = kitty_display(b"\x89PNG\r\n");
        assert!(s.starts_with("\x1b_Gf=100,a=T,q=2"));
        assert!(s.ends_with("\x1b\\"));
    }

    #[test]
    fn place_produces_cursor_move_then_a_eq_p() {
        let mut buf = Vec::new();
        place(&mut buf, 7, 3, 5).unwrap();
        let s = String::from_utf8(buf).unwrap();
        // Cursor move is 1-indexed (row+1, col+1), then place by image and
        // placement id with C=1 so kitty doesn't advance the cursor.
        assert_eq!(s, "\x1b[6;4H\x1b_Ga=p,i=7,p=7,C=1,q=2;\x1b\\");
    }

    #[test]
    fn delete_placement_preserves_cached_image() {
        let mut buf = Vec::new();
        delete_placement(&mut buf, 9).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(s, "\x1b_Ga=d,d=i,i=9,p=9,q=2;\x1b\\");
    }

    #[test]
    fn delete_all_for_client_sends_d_cap_a() {
        let mut buf = Vec::new();
        delete_all_for_client(&mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(s, "\x1b_Ga=d,d=A,q=2;\x1b\\");
    }
}

#[cfg(test)]
mod heading_cache_tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn render_heading_is_cached_keyed_by_level_theme_font_text() {
        let cfg = Config::default();
        let fp = font_fingerprint(&cfg);
        // A unique string so this test never aliases another's cache entry.
        let text = "Heading Cache Probe — δοκιμή 测试";

        let first = render_heading(text, 1, &cfg, Theme::Dark).expect("heading should render");

        // The (level, theme, font, text) key is now memoized.
        {
            let map = HEADING_CACHE.get().unwrap().lock().unwrap();
            assert!(map.contains_key(&(1u8, Theme::Dark, fp.clone(), text.to_owned())));
        }

        // A second call returns the identical bytes (served from cache).
        let second = render_heading(text, 1, &cfg, Theme::Dark).expect("heading should render");
        assert_eq!(first, second, "cached render must be byte-identical");

        // Theme is part of the key: a different theme is a distinct entry.
        let _light = render_heading(text, 1, &cfg, Theme::Light).expect("heading should render");
        let map = HEADING_CACHE.get().unwrap().lock().unwrap();
        assert!(map.contains_key(&(1u8, Theme::Light, fp, text.to_owned())));
    }

    #[test]
    fn rendered_heading_pngs_are_valid_nonblank_and_scale_by_level() {
        let cfg = Config::default();
        let titles = [
            "🚀 H1 标题 with emoji 中文",
            "✨ H2 标题 with emoji 中文",
            "🧪 H3 标题 with emoji 中文",
        ];
        let images: Vec<_> = titles
            .iter()
            .enumerate()
            .map(|(index, title)| {
                let (png, _, _) = render_heading(title, index as u8 + 1, &cfg, Theme::Dark)
                    .expect("heading should render");
                image::load_from_memory_with_format(&png, ImageFormat::Png)
                    .expect("heading should be a valid PNG")
                    .to_rgba8()
            })
            .collect();

        assert!(images[0].height() > images[1].height());
        assert!(images[1].height() > images[2].height());
        for image in images {
            assert!(image.width() > 100);
            let nonblank = image.pixels().filter(|pixel| pixel[3] > 0).count();
            assert!(nonblank * 20 >= (image.width() * image.height()) as usize);
        }
    }

    #[test]
    fn font_fingerprint_distinguishes_configs() {
        let mut a = Config::default();
        a.font.heading.latin = Some("Inter".to_string());
        let mut b = Config::default();
        b.font.heading.latin = Some("Charter".to_string());
        assert_ne!(font_fingerprint(&a), font_fingerprint(&b));
        // An all-unset config is stable and shared.
        assert_eq!(
            font_fingerprint(&Config::default()),
            font_fingerprint(&Config::default())
        );
    }
}
