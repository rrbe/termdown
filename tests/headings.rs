use base64::Engine;
use image::ImageReader;
use std::io::Cursor;
use std::path::Path;
use std::process::{Command, Stdio};

fn binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_termdown")
}

fn run_termdown(path: &Path) -> Vec<u8> {
    let out = Command::new(binary_path())
        .arg("--theme")
        .arg("dark")
        .arg(path)
        .env("TERM_PROGRAM", "ghostty")
        .env_remove("HOME")
        .env_remove("USERPROFILE")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("termdown should run");
    assert!(
        out.status.success(),
        "termdown failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}

/// Walk `out` looking for Kitty graphics protocol APC frames
/// (`ESC _ G <header> ; <payload> ESC \`). Frames where the header has `m=1`
/// continue the current image; `m=0` (or absent) terminates it. Returns one
/// decoded PNG byte vector per emitted image.
fn extract_kitty_pngs(out: &[u8]) -> Vec<Vec<u8>> {
    let mut images = Vec::new();
    let mut current_b64 = String::new();
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
        let header = std::str::from_utf8(&out[i + 3..sep]).expect("APC header is ASCII");

        let payload_start = if sep < out.len() && out[sep] == b';' {
            sep + 1
        } else {
            sep
        };
        let mut end = payload_start;
        while end + 1 < out.len() && !(out[end] == 0x1b && out[end + 1] == b'\\') {
            end += 1;
        }
        let chunk = std::str::from_utf8(&out[payload_start..end]).expect("APC payload is ASCII");
        current_b64.push_str(chunk);

        let more = header.split(',').any(|kv| kv == "m=1");
        if !more {
            let png = base64::engine::general_purpose::STANDARD
                .decode(&current_b64)
                .expect("base64 payload decodes");
            images.push(png);
            current_b64.clear();
        }

        i = end + 2;
    }
    images
}

#[test]
fn h1_h2_h3_emit_decodable_pngs_with_descending_heights() {
    let fixture = Path::new("fixtures/headings.md");
    let stdout = run_termdown(fixture);
    let pngs = extract_kitty_pngs(&stdout);

    assert_eq!(
        pngs.len(),
        3,
        "expected one Kitty image per heading level (H1, H2, H3); got {}",
        pngs.len()
    );

    let dims: Vec<(u32, u32)> = pngs
        .iter()
        .enumerate()
        .map(|(idx, png)| {
            let img = ImageReader::new(Cursor::new(png))
                .with_guessed_format()
                .unwrap_or_else(|e| panic!("png {idx} guess_format: {e}"))
                .decode()
                .unwrap_or_else(|e| panic!("png {idx} decode: {e}"))
                .to_rgba8();
            assert!(
                img.width() > 0 && img.height() > 0,
                "png {idx} has empty dimensions"
            );
            assert!(
                img.pixels().any(|p| p[3] > 0),
                "png {idx} is fully transparent"
            );
            (img.width(), img.height())
        })
        .collect();

    let (_, h1_h) = dims[0];
    let (_, h2_h) = dims[1];
    let (_, h3_h) = dims[2];

    assert!(
        h1_h > h2_h,
        "H1 height ({h1_h}) should exceed H2 height ({h2_h})"
    );
    assert!(
        h2_h > h3_h,
        "H2 height ({h2_h}) should exceed H3 height ({h3_h})"
    );
}
