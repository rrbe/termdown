//! File Browser state (yazi-style): scans a directory for Markdown files and
//! tracks the cursor + debounce timing that drives the live preview.
//!
//! This is the **filesystem** browser — distinct from the Table of Contents
//! ("Contents") panel, which lists the *headings of one document*. See
//! `CONTEXT.md` for the 目录 terminology collision.

use std::path::{Path, PathBuf};
use std::time::Instant;

pub struct FileBrowser {
    /// Directory being browsed (as given on the CLI; may be relative).
    pub dir: PathBuf,
    /// Markdown files in `dir`, sorted. Single level, no recursion (HALF 1).
    pub entries: Vec<PathBuf>,
    /// Index into `entries` of the highlighted row.
    pub cursor: usize,
    /// `Some(t)` = the cursor moved at `t` and we're waiting for it to settle
    /// before (re)building the preview. `None` = settled/idle.
    pub last_move: Option<Instant>,
    /// Path the current preview was built for; `None` until the first build or
    /// after a commit (so re-entering the browser rebuilds).
    pub preview_path: Option<PathBuf>,
}

impl FileBrowser {
    /// Scan `dir` for `*.md` / `*.markdown` files (single level). Returns an
    /// error only if the directory itself can't be read; an empty result is a
    /// valid (caller-handled) outcome.
    pub fn scan(dir: &Path) -> std::io::Result<FileBrowser> {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file() && is_markdown(p))
            .collect();
        entries.sort();
        Ok(FileBrowser {
            dir: dir.to_path_buf(),
            entries,
            cursor: 0,
            // None so the first loop iteration treats the selection as
            // "settled" and builds the initial preview immediately.
            last_move: None,
            preview_path: None,
        })
    }

    pub fn selected(&self) -> Option<&PathBuf> {
        self.entries.get(self.cursor)
    }

    /// Display name for a row (file name only).
    pub fn name_at(&self, idx: usize) -> String {
        self.entries
            .get(idx)
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default()
    }
}

fn is_markdown(p: &Path) -> bool {
    matches!(
        p.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .as_deref(),
        Some("md") | Some("markdown")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_markdown_matches_extensions() {
        assert!(is_markdown(Path::new("a.md")));
        assert!(is_markdown(Path::new("a.markdown")));
        assert!(is_markdown(Path::new("DIR/A.MD")));
        assert!(!is_markdown(Path::new("a.txt")));
        assert!(!is_markdown(Path::new("README")));
    }
}
