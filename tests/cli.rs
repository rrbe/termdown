use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_termdown")
}

fn run_termdown(
    args: &[&str],
    stdin: Option<&str>,
    envs: &[(&str, &str)],
    removed_envs: &[&str],
) -> Output {
    let mut command = Command::new(binary_path());
    command.args(args);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }

    for (key, value) in envs {
        command.env(key, value);
    }
    for key in removed_envs {
        command.env_remove(key);
    }

    let mut child = command.spawn().expect("failed to spawn termdown");
    if let Some(input) = stdin {
        child
            .stdin
            .as_mut()
            .expect("stdin should be piped")
            .write_all(input.as_bytes())
            .expect("failed to write stdin");
    }

    child.wait_with_output().expect("failed to collect output")
}

fn stdout_text(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout should be valid UTF-8")
}

fn stderr_text(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr should be valid UTF-8")
}

fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    for nc in chars.by_ref() {
                        if nc.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
                Some('_') | Some(']') => {
                    chars.next();
                    while let Some(nc) = chars.next() {
                        if nc == '\x1b' && chars.peek() == Some(&'\\') {
                            chars.next();
                            break;
                        }
                    }
                }
                _ => {
                    chars.next();
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

struct TempMarkdownFile {
    path: PathBuf,
}

impl TempMarkdownFile {
    fn new(contents: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("termdown-cli-{}-{}.md", std::process::id(), unique));

        fs::write(&path, contents).expect("failed to write temp markdown file");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempMarkdownFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[test]
fn help_flag_prints_usage() {
    let output = run_termdown(&["--help"], None, &[], &[]);
    let stdout = stdout_text(&output);

    assert!(output.status.success());
    assert!(stdout.contains("Render Markdown with large-font headings in the terminal"));
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("Config: ~/.termdown/config.toml"));
    assert!(stderr_text(&output).trim().is_empty());
}

#[test]
fn version_flag_prints_version() {
    let output = run_termdown(&["--version"], None, &[], &[]);

    assert!(output.status.success());
    assert_eq!(
        stdout_text(&output),
        format!("termdown {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert!(stderr_text(&output).trim().is_empty());
}

#[test]
fn stdin_rendering_works_without_terminal_warning_when_supported() {
    let output = run_termdown(&["-"], Some("hello\n"), &[("TERM_PROGRAM", "ghostty")], &[]);

    assert!(output.status.success());
    assert_eq!(stdout_text(&output), "    hello\n");
    assert!(stderr_text(&output).trim().is_empty());
}

#[test]
fn file_input_renders_table_output() {
    let file = TempMarkdownFile::new("| A | B |\n| - | - |\n| x | long |\n");
    let output = run_termdown(
        &[file.path().to_str().expect("path should be valid UTF-8")],
        None,
        &[("TERM_PROGRAM", "ghostty")],
        &[],
    );
    let stdout = strip_ansi(&stdout_text(&output));

    assert!(output.status.success());
    assert!(stderr_text(&output).trim().is_empty());
    assert!(stdout.contains("    A  │  B"));
    assert!(stdout.contains("    x  │  long"));
}

#[test]
fn missing_file_returns_read_error() {
    let missing = std::env::temp_dir().join(format!(
        "termdown-missing-{}-{}.md",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos()
    ));
    let output = run_termdown(
        &[missing.to_str().expect("path should be valid UTF-8")],
        None,
        &[("TERM_PROGRAM", "ghostty")],
        &[],
    );

    assert!(!output.status.success());
    assert!(
        stderr_text(&output).contains(&format!("Error reading {}", missing.display())),
        "stderr was: {}",
        stderr_text(&output)
    );
}

#[test]
fn html_inline_tags_map_to_ansi_and_block_renders_dim() {
    let input =
        "hello <b>bold</b> <u>under</u> and <!-- hidden --> tail\n\n<div>\n<p>x</p>\n</div>\n";
    let output = run_termdown(&["-"], Some(input), &[("TERM_PROGRAM", "ghostty")], &[]);
    let raw = stdout_text(&output);
    let clean = strip_ansi(&raw);

    assert!(output.status.success(), "stderr: {}", stderr_text(&output));

    // Comment gone, inline tag text preserved.
    assert!(
        clean.contains("hello bold under and  tail"),
        "clean output was: {clean:?}"
    );
    // Block HTML lines preserved verbatim.
    assert!(clean.contains("    <div>"), "clean output was: {clean:?}");
    assert!(
        clean.contains("    <p>x</p>"),
        "clean output was: {clean:?}"
    );
    assert!(clean.contains("    </div>"), "clean output was: {clean:?}");

    // ANSI codes present in raw output: bold (\x1b[1m) and underline (\x1b[4m).
    assert!(raw.contains("\x1b[1m"), "raw output: {raw:?}");
    assert!(raw.contains("\x1b[4m"), "raw output: {raw:?}");
}

#[test]
fn unsupported_terminal_emits_warning_on_stderr() {
    let output = run_termdown(
        &["-"],
        Some("hello\n"),
        &[],
        &["TERM_PROGRAM", "TERM", "KITTY_WINDOW_ID"],
    );

    assert!(output.status.success());
    assert_eq!(stdout_text(&output), "    hello\n");
    let stderr = stderr_text(&output);
    assert!(stderr.contains("termdown: warning: terminal may not support Kitty graphics protocol"));
    assert!(stderr.contains("termdown: headings require Ghostty, Kitty, WezTerm, or iTerm2"));
}

#[test]
fn tui_without_file_fails_with_error() {
    let output = run_termdown(&["--tui"], None, &[("TERM_PROGRAM", "ghostty")], &[]);
    assert!(!output.status.success());
    assert!(
        stderr_text(&output).contains("--tui requires a FILE"),
        "stderr: {}",
        stderr_text(&output)
    );
}

#[test]
fn tui_with_stdin_sentinel_fails() {
    let output = run_termdown(
        &["--tui", "-"],
        Some("# hi\n"),
        &[("TERM_PROGRAM", "ghostty")],
        &[],
    );
    assert!(!output.status.success());
    assert!(
        stderr_text(&output).contains("--tui requires a FILE"),
        "stderr: {}",
        stderr_text(&output)
    );
}
