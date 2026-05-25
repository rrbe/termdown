mod common;

use std::fs;
use std::path::Path;

use common::run_termdown;

/// Replace each run of kitty image APC sequences (`ESC _ G ... ESC \`) with a
/// single `<IMG>` marker. Font rasterization produces OS-specific PNG bytes
/// that can't be compared across platforms — we only validate that an image
/// was emitted at a given position, not its pixel content.
fn strip_kitty_images(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    let mut in_image_run = false;
    while i < bytes.len() {
        if i + 2 < bytes.len() && bytes[i] == 0x1b && bytes[i + 1] == b'_' && bytes[i + 2] == b'G' {
            let mut j = i + 3;
            while j + 1 < bytes.len() && !(bytes[j] == 0x1b && bytes[j + 1] == b'\\') {
                j += 1;
            }
            if j + 1 < bytes.len() {
                j += 2;
            }
            if !in_image_run {
                out.extend_from_slice(b"<IMG>");
                in_image_run = true;
            }
            i = j;
            continue;
        }
        in_image_run = false;
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8(out).expect("valid utf-8")
}

fn render(path: &Path) -> String {
    let stdout = run_termdown(path);
    let raw = String::from_utf8(stdout).expect("valid utf-8");
    strip_kitty_images(&raw)
}

fn check_snapshot(fixture: &str) {
    let md = Path::new("fixtures").join(format!("{fixture}.md"));
    let expected_path = Path::new("fixtures/expected").join(format!("{fixture}.ansi"));
    let expected = fs::read_to_string(&expected_path).expect("expected file");
    let actual = render(&md);
    if actual != expected {
        let tmp = std::env::temp_dir().join(format!("termdown-snapshot-{fixture}.ansi"));
        fs::write(&tmp, &actual).expect("failed to write snapshot diff to temp file");
        panic!(
            "snapshot mismatch for {fixture}\n  expected: {}\n  actual written to: {}",
            expected_path.display(),
            tmp.display()
        );
    }
}

#[test]
fn snapshot_supported_syntax() {
    check_snapshot("supported-syntax");
}
#[test]
fn snapshot_unsupported_syntax() {
    check_snapshot("unsupported-syntax");
}
