use ab_glyph::{point, Font, PxScale, ScaleFont};
use base64::Engine;
use image::codecs::png::PngEncoder;
use image::{ImageEncoder, Rgba, RgbaImage};

use crate::config::Config;
use crate::font::{self, FontPair};
use crate::style;

// ─── Text Metrics ───────────────────────────────────────────────────────────

/// Width, height, and ascent of the measured text.
struct TextMetrics {
    width: f32,
    height: f32,
    ascent: f32,
}

fn measure_text(fonts: &FontPair, scale: PxScale, text: &str, tracking: f32) -> TextMetrics {
    let latin_s = fonts.latin.as_scaled(scale);
    let cjk_s = fonts.cjk.as_scaled(scale);

    // Use the tallest ascent and deepest descent across both fonts
    let ascent = latin_s.ascent().max(cjk_s.ascent());
    let descent = latin_s.descent().min(cjk_s.descent()); // descent is negative

    let chars: Vec<char> = text.chars().collect();
    let mut width = 0.0f32;

    for (i, &ch) in chars.iter().enumerate() {
        let font = fonts.for_char(ch);
        let scaled = font.as_scaled(scale);
        width += scaled.h_advance(font.glyph_id(ch));
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
    fonts: &FontPair,
    scale: PxScale,
    baseline: (f32, f32),
    text: &str,
    color: [u8; 4],
    tracking: f32,
) {
    let mut x = baseline.0;
    let y = baseline.1;

    for ch in text.chars() {
        let font = fonts.for_char(ch);
        let scaled = font.as_scaled(scale);
        let glyph_id = font.glyph_id(ch);
        let glyph = glyph_id.with_scale_and_position(scale, point(x, y));

        if let Some(outlined) = font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            outlined.draw(|gx, gy, coverage| {
                let px = bounds.min.x as i32 + gx as i32;
                let py = bounds.min.y as i32 + gy as i32;
                if px >= 0
                    && py >= 0
                    && (px as u32) < img.width()
                    && (py as u32) < img.height()
                {
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
        }

        x += scaled.h_advance(glyph_id) + if tracking > 0.0 { tracking } else { 0.0 };
    }
}

// ─── Heading Rendering ──────────────────────────────────────────────────────

/// Render heading text to a PNG image. Returns `None` if font loading or
/// PNG encoding fails (caller should fall back to ANSI text).
pub fn render_heading(text: &str, level: u8, config: &Config) -> Option<Vec<u8>> {
    let st = style::heading_style(level);
    let fonts = font::get_fonts(level, config)?;
    let scale = PxScale {
        x: st.size,
        y: st.size,
    };

    let metrics = measure_text(&fonts, scale, text, st.tracking);

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
        &fonts,
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
    Some(buf)
}

// ─── Kitty Graphics Protocol ────────────────────────────────────────────────

/// Drain any Kitty graphics protocol responses from stdin.
/// Terminals like iTerm2 send back `OK` acknowledgments that leak as visible text.
#[cfg(unix)]
pub fn drain_kitty_responses() {
    std::thread::sleep(std::time::Duration::from_millis(50));
    // SAFETY: tcflush is a standard POSIX call on valid fd.
    unsafe {
        libc::tcflush(libc::STDIN_FILENO, libc::TCIFLUSH);
    }
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
