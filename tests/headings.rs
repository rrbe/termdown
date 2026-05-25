mod common;

use base64::Engine;
use image::ImageReader;
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;

use common::run_termdown;

/// One emitted Kitty graphics frame: the parsed APC header fields and the
/// decoded PNG bytes (with any `m=1` chunks already reassembled).
struct Frame {
    header: HashMap<String, String>,
    png: Vec<u8>,
}

/// Walk `out` looking for Kitty graphics protocol APC frames
/// (`ESC _ G <header> ; <payload> ESC \`). Frames where the header has `m=1`
/// continue the current image; `m=0` (or absent) terminates it. Returns one
/// `Frame` per emitted image with the *terminator* frame's parsed header.
///
/// Panics if an APC frame is not terminated by `ESC \` — a regression where
/// termdown truncates output should surface as a clear parse error here,
/// not as a confusing base64-decode failure downstream.
fn extract_kitty_frames(out: &[u8]) -> Vec<Frame> {
    let mut frames = Vec::new();
    let mut current_b64 = String::new();
    let mut start_header: Option<HashMap<String, String>> = None;
    let mut i = 0;
    while i + 2 < out.len() {
        if !(out[i] == 0x1b && out[i + 1] == b'_' && out[i + 2] == b'G') {
            i += 1;
            continue;
        }
        let mut sep = i + 3;
        while sep < out.len() && out[sep] != b';' && out[sep] != 0x1b {
            sep += 1;
        }
        let header_str = std::str::from_utf8(&out[i + 3..sep]).expect("APC header is ASCII");
        let header = parse_header(header_str);
        // The first chunk of an image carries the full header (f=, a=, i=, ...).
        // Continuation chunks only carry `m=1` or `m=0`. Capture the first one
        // and let later chunks just append payload.
        if start_header.is_none() {
            start_header = Some(header.clone());
        }

        let payload_start = if sep < out.len() && out[sep] == b';' {
            sep + 1
        } else {
            sep
        };
        let mut end = payload_start;
        let mut terminated = false;
        while end + 1 < out.len() {
            if out[end] == 0x1b && out[end + 1] == b'\\' {
                terminated = true;
                break;
            }
            end += 1;
        }
        assert!(
            terminated,
            "unterminated kitty APC frame starting at byte offset {i}"
        );
        let chunk = std::str::from_utf8(&out[payload_start..end]).expect("APC payload is ASCII");
        current_b64.push_str(chunk);

        let more = header.get("m").map(|s| s.as_str()) == Some("1");
        if !more {
            let png = base64::engine::general_purpose::STANDARD
                .decode(&current_b64)
                .expect("base64 payload decodes");
            frames.push(Frame {
                header: start_header.take().expect("start header captured"),
                png,
            });
            current_b64.clear();
        }

        i = end + 2;
    }
    frames
}

fn parse_header(s: &str) -> HashMap<String, String> {
    s.split(',')
        .filter_map(|kv| {
            let (k, v) = kv.split_once('=')?;
            Some((k.to_string(), v.to_string()))
        })
        .collect()
}

fn decode_png(png: &[u8], idx: usize) -> image::RgbaImage {
    ImageReader::new(Cursor::new(png))
        .with_guessed_format()
        .unwrap_or_else(|e| panic!("png {idx} guess_format: {e}"))
        .decode()
        .unwrap_or_else(|e| panic!("png {idx} decode: {e}"))
        .to_rgba8()
}

const BASIC_FIXTURE: &str = "fixtures/specialized/headings-basic.md";
const EMOJI_FIXTURE: &str = "fixtures/specialized/headings-emoji.md";

#[test]
fn cat_path_emits_only_display_form_apc_headers() {
    // Cat mode must emit `a=T` (transmit + display) on every frame. `a=t`
    // (transmit-only) is the TUI's cached-placement form and would paint
    // nothing here — a single bad frame would silently break heading
    // rendering, so we check *every* frame, not just the first.
    let stdout = run_termdown(Path::new(BASIC_FIXTURE));
    let frames = extract_kitty_frames(&stdout);
    assert!(
        !frames.is_empty(),
        "expected at least one APC frame in cat output"
    );
    for (idx, frame) in frames.iter().enumerate() {
        let a = frame.header.get("a").map(|s| s.as_str());
        assert_eq!(
            a,
            Some("T"),
            "frame {idx}: cat mode must use display-form a=T, got a={a:?} (full header: {:?})",
            frame.header
        );
    }
}

#[test]
fn every_emitted_png_decodes_and_is_nonblank() {
    let stdout = run_termdown(Path::new(BASIC_FIXTURE));
    let frames = extract_kitty_frames(&stdout);
    assert!(!frames.is_empty(), "expected emitted PNGs");
    for (idx, frame) in frames.iter().enumerate() {
        let img = decode_png(&frame.png, idx);
        assert!(
            img.width() > 0 && img.height() > 0,
            "png {idx} has empty dimensions ({}x{})",
            img.width(),
            img.height()
        );
        assert!(
            img.pixels().any(|p| p[3] > 0),
            "png {idx} is fully transparent"
        );
    }
}

#[test]
fn heading_level_determines_descending_height() {
    let stdout = run_termdown(Path::new(BASIC_FIXTURE));
    let frames = extract_kitty_frames(&stdout);
    assert_eq!(
        frames.len(),
        3,
        "BASIC_FIXTURE has exactly H1+H2+H3 — expected 3 PNGs, got {}",
        frames.len()
    );
    let heights: Vec<u32> = frames
        .iter()
        .enumerate()
        .map(|(idx, f)| decode_png(&f.png, idx).height())
        .collect();
    assert!(
        heights[0] > heights[1],
        "H1 height ({}) should exceed H2 height ({})",
        heights[0],
        heights[1]
    );
    assert!(
        heights[1] > heights[2],
        "H2 height ({}) should exceed H3 height ({})",
        heights[1],
        heights[2]
    );
}

#[test]
fn emoji_headings_emit_decodable_nonblank_pngs() {
    // Regression coverage for emoji font fallback: a broken fallback chain
    // would render emoji glyphs as transparent .notdef and the PNG, while
    // still decodable, would be missing pixels in the emoji region. We
    // assert non-transparency to catch that.
    let stdout = run_termdown(Path::new(EMOJI_FIXTURE));
    let frames = extract_kitty_frames(&stdout);
    assert_eq!(
        frames.len(),
        3,
        "EMOJI_FIXTURE has exactly H1+H2+H3 — expected 3 PNGs, got {}",
        frames.len()
    );
    for (idx, frame) in frames.iter().enumerate() {
        let img = decode_png(&frame.png, idx);
        assert!(
            img.width() > 100,
            "emoji heading {idx} png is suspiciously narrow ({}px) — emoji or text may have been cropped",
            img.width()
        );
        let nonblank = img.pixels().filter(|p| p[3] > 0).count();
        // Require at least ~5% non-transparent pixels. A "mostly empty"
        // image would indicate font fallback failure dropping both text
        // and emoji glyphs.
        let total = (img.width() * img.height()) as usize;
        assert!(
            nonblank * 20 >= total,
            "emoji heading {idx} png is mostly transparent ({nonblank}/{total} non-blank pixels)"
        );
    }
}
