//! Interactive TUI mode. Full implementation lands in Task 2.3+.

use crate::config::Config;
use crate::theme::Theme;

#[allow(dead_code)] // TODO: Task 2.3 replaces this stub
pub fn run(path: &str, _config: &Config, _theme: Theme) {
    eprintln!("termdown: --tui not yet implemented (file: {path})");
    std::process::exit(1);
}
