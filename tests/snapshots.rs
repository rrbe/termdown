use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

fn binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_termdown")
}

fn render(path: &Path) -> String {
    let out = Command::new(binary_path())
        .arg(path)
        .env("TERM_PROGRAM", "ghostty")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("termdown should run");
    assert!(out.status.success(), "termdown failed on {path:?}");
    String::from_utf8(out.stdout).expect("valid utf-8")
}

fn check_snapshot(fixture: &str) {
    let md = Path::new("fixtures").join(format!("{fixture}.md"));
    let expected_path = Path::new("fixtures/expected").join(format!("{fixture}.ansi"));
    let expected = fs::read_to_string(&expected_path).expect("expected file");
    let actual = render(&md);
    if actual != expected {
        let tmp = std::env::temp_dir().join(format!("termdown-snapshot-{fixture}.ansi"));
        fs::write(&tmp, &actual).ok();
        panic!(
            "snapshot mismatch for {fixture}\n  expected: {}\n  actual written to: {}",
            expected_path.display(),
            tmp.display()
        );
    }
}

#[test]
fn snapshot_emoji_test() {
    check_snapshot("emoji-test");
}
#[test]
fn snapshot_full_syntax_zh() {
    check_snapshot("full-syntax-zh");
}
#[test]
fn snapshot_full_syntax() {
    check_snapshot("full-syntax");
}
#[test]
fn snapshot_tasklist() {
    check_snapshot("tasklist");
}
#[test]
fn snapshot_unsupported_syntax() {
    check_snapshot("unsupported-syntax");
}
