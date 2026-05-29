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

    // Isolate config loading from the developer/CI environment so a real
    // ~/.config/termdown/config.toml (or $XDG_CONFIG_HOME) can't leak in.
    // Callers may re-set any of these via `envs`.
    command.env_remove("HOME");
    command.env_remove("USERPROFILE");
    command.env_remove("XDG_CONFIG_HOME");

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
        // `nanos()` alone can collide between two parallel test threads on
        // platforms with coarse-grained clock resolution; a monotonic counter
        // on top guarantees uniqueness within the process.
        static SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let seq = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "termdown-cli-{}-{}-{}.md",
            std::process::id(),
            unique,
            seq
        ));

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

/// A throwaway directory tree, cleaned up on drop. Files are written at
/// caller-chosen relative paths so the same helper can stage either an XDG
/// config root (`termdown/config.toml`) or a fake `$HOME` (`.termdown/...`).
struct TempDir {
    root: PathBuf,
}

impl TempDir {
    fn new() -> Self {
        static SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let seq = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "termdown-cfg-{}-{}-{}",
            std::process::id(),
            unique,
            seq
        ));
        fs::create_dir_all(&root).expect("failed to create temp dir");
        Self { root }
    }

    fn write(&self, rel: &str, contents: &str) {
        let path = self.root.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("failed to create temp subdir");
        }
        fs::write(path, contents).expect("failed to write temp file");
    }

    fn path_str(&self) -> &str {
        self.root.to_str().expect("temp path should be valid UTF-8")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

// Body is a plain paragraph (not a heading) so it renders as text under
// ghostty rather than as a Kitty-graphics image.
const FRONTMATTER_DOC: &str = "---\ntitle: Hello\nauthor: Me\n---\n\nPlain body text.\n";

#[test]
fn help_flag_prints_usage() {
    let output = run_termdown(&["--help"], None, &[], &[]);
    let stdout = stdout_text(&output);

    assert!(output.status.success());
    assert!(stdout.contains("Render Markdown with large-font headings in the terminal"));
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("Config: ~/.config/termdown/config.toml"));
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
    assert_eq!(stdout_text(&output), "hello\n");
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
    assert!(stdout.contains("A  │  B"));
    assert!(stdout.contains("x  │  long"));
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
    assert!(clean.contains("<div>"), "clean output was: {clean:?}");
    assert!(clean.contains("<p>x</p>"), "clean output was: {clean:?}");
    assert!(clean.contains("</div>"), "clean output was: {clean:?}");

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
    assert_eq!(stdout_text(&output), "hello\n");
    let stderr = stderr_text(&output);
    assert!(stderr.contains("termdown: warning: terminal may not support Kitty graphics protocol"));
    assert!(stderr.contains("termdown: headings require Ghostty, Kitty, WezTerm, or iTerm2"));
}

#[test]
fn file_arg_with_piped_stdout_falls_back_to_cat() {
    // Default is TUI when a FILE is given, but only if stdout is a TTY.
    // Tests pipe stdout, so we should get plain cat output, not a TUI error.
    let file = TempMarkdownFile::new("hello from file\n");
    let output = run_termdown(
        &[file.path().to_str().expect("path should be valid UTF-8")],
        None,
        &[("TERM_PROGRAM", "ghostty")],
        &[],
    );
    let stdout = strip_ansi(&stdout_text(&output));

    assert!(output.status.success(), "stderr: {}", stderr_text(&output));
    assert!(stdout.contains("hello from file"), "stdout was: {stdout:?}");
}

#[test]
fn cat_flag_forces_cat_output_with_file() {
    let file = TempMarkdownFile::new("plain content\n");
    let output = run_termdown(
        &[
            "--cat",
            file.path().to_str().expect("path should be valid UTF-8"),
        ],
        None,
        &[("TERM_PROGRAM", "ghostty")],
        &[],
    );
    let stdout = strip_ansi(&stdout_text(&output));

    assert!(output.status.success(), "stderr: {}", stderr_text(&output));
    assert!(stdout.contains("plain content"), "stdout was: {stdout:?}");
}

#[test]
fn xdg_config_home_is_honored_for_config_loading() {
    let doc = TempMarkdownFile::new(FRONTMATTER_DOC);
    let path = doc.path().to_str().expect("path should be valid UTF-8");

    // Baseline: with no config at all, the folded metadata summary is shown.
    let baseline = run_termdown(&["--cat", path], None, &[("TERM_PROGRAM", "ghostty")], &[]);
    let baseline_out = strip_ansi(&stdout_text(&baseline));
    assert!(
        baseline.status.success(),
        "stderr: {}",
        stderr_text(&baseline)
    );
    assert!(
        baseline_out.contains("[metadata ·"),
        "baseline should show the metadata summary, was: {baseline_out:?}"
    );

    // A `config.toml` placed at `$XDG_CONFIG_HOME/termdown/` must take effect:
    // `metadata = false` hides the summary the baseline rendered.
    let cfg = TempDir::new();
    cfg.write("termdown/config.toml", "metadata = false\n");
    let configured = run_termdown(
        &["--cat", path],
        None,
        &[
            ("TERM_PROGRAM", "ghostty"),
            ("XDG_CONFIG_HOME", cfg.path_str()),
        ],
        &[],
    );
    let configured_out = strip_ansi(&stdout_text(&configured));
    assert!(
        configured.status.success(),
        "stderr: {}",
        stderr_text(&configured)
    );
    assert!(
        !configured_out.contains("[metadata"),
        "config should hide the metadata summary, was: {configured_out:?}"
    );
    assert!(
        configured_out.contains("Plain body text"),
        "body should still render, was: {configured_out:?}"
    );
}

#[test]
fn legacy_config_location_triggers_migration_warning() {
    // A fake $HOME that still holds the pre-XDG `~/.termdown/config.toml` but
    // has no config at the new XDG path. termdown should ignore the legacy
    // file and warn the user to move it, instead of failing silently.
    let home = TempDir::new();
    home.write(".termdown/config.toml", "theme = \"dark\"\n");

    let doc = TempMarkdownFile::new("hello\n");
    let path = doc.path().to_str().expect("path should be valid UTF-8");
    let output = run_termdown(
        &["--cat", path],
        None,
        &[("TERM_PROGRAM", "ghostty"), ("HOME", home.path_str())],
        &[],
    );

    assert!(output.status.success(), "stderr: {}", stderr_text(&output));
    let stderr = stderr_text(&output);
    assert!(
        stderr.contains("ignoring legacy config"),
        "expected a migration warning, stderr was: {stderr:?}"
    );
    // Match the legacy dir name only (not a `/`-joined path) so the assertion
    // holds on Windows, where `Path::display` uses `\` separators. The new
    // path is `.config<sep>termdown`, which never contains `.termdown`.
    assert!(
        stderr.contains(".termdown"),
        "warning should name the legacy path, stderr was: {stderr:?}"
    );
}
