//! Key-event to Action mapping for the TUI. Actions are intent-level
//! operations (`ScrollLines(1)`, `SearchBegin { reverse: false }`, etc.);
//! dispatching them to concrete state mutations happens in `tui::mod`.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Action {
    Quit,
    ScrollLines(i32),
    ScrollHalfPage(i32),
    ScrollPage(i32),
    JumpStart,
    JumpEnd,
    NextHeading,
    PrevHeading,
    ToggleToc,
    SearchBegin { reverse: bool },
    SearchNext,
    SearchPrev,
    OpenLink,
    Back,
    Forward,
    ToggleHelp,
    None,
}

pub fn map_normal(key: KeyEvent) -> Action {
    if key.kind != KeyEventKind::Press {
        return Action::None;
    }
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('c') if ctrl => Action::Quit,

        KeyCode::Char('j') | KeyCode::Down => Action::ScrollLines(1),
        KeyCode::Char('k') | KeyCode::Up => Action::ScrollLines(-1),

        // Note: the bare-char arms below intentionally match regardless of
        // modifiers, so `Ctrl-d/u/f/b` hit the same actions as `d/u/f/b`.
        // This matches the design-doc key table and is exercised by
        // `ctrl_modifier_variants_match_bare_letter`.
        KeyCode::Char('d') => Action::ScrollHalfPage(1),
        KeyCode::Char('u') => Action::ScrollHalfPage(-1),

        KeyCode::Char('f') | KeyCode::Char(' ') | KeyCode::PageDown => Action::ScrollPage(1),
        KeyCode::Char('b') | KeyCode::PageUp => Action::ScrollPage(-1),

        KeyCode::Char('G') => Action::JumpEnd,
        // Note: `g` alone is not a JumpStart — Task 4.1 adds the `gg` two-key sequence.
        KeyCode::Char(']') => Action::NextHeading,
        KeyCode::Char('[') => Action::PrevHeading,

        KeyCode::Char('t') => Action::ToggleToc,

        KeyCode::Char('/') => Action::SearchBegin { reverse: false },
        KeyCode::Char('?') => Action::ToggleHelp,
        KeyCode::Char('n') => Action::SearchNext,
        KeyCode::Char('N') => Action::SearchPrev,

        KeyCode::Enter => Action::OpenLink,
        KeyCode::Char('o') => Action::Back,
        KeyCode::Char('i') => Action::Forward,

        _ => Action::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn press(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::empty())
    }

    fn press_ctrl(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::CONTROL)
    }

    #[test]
    fn q_and_ctrl_c_both_quit() {
        assert!(matches!(
            map_normal(press(KeyCode::Char('q'))),
            Action::Quit
        ));
        assert!(matches!(
            map_normal(press_ctrl(KeyCode::Char('c'))),
            Action::Quit
        ));
    }

    #[test]
    fn scroll_keys_map_to_correct_actions() {
        assert!(matches!(
            map_normal(press(KeyCode::Char('j'))),
            Action::ScrollLines(1)
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('k'))),
            Action::ScrollLines(-1)
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Down)),
            Action::ScrollLines(1)
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Up)),
            Action::ScrollLines(-1)
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('d'))),
            Action::ScrollHalfPage(1)
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('u'))),
            Action::ScrollHalfPage(-1)
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('f'))),
            Action::ScrollPage(1)
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('b'))),
            Action::ScrollPage(-1)
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char(' '))),
            Action::ScrollPage(1)
        ));
        assert!(matches!(
            map_normal(press(KeyCode::PageDown)),
            Action::ScrollPage(1)
        ));
        assert!(matches!(
            map_normal(press(KeyCode::PageUp)),
            Action::ScrollPage(-1)
        ));
    }

    #[test]
    fn search_keys() {
        assert!(matches!(
            map_normal(press(KeyCode::Char('/'))),
            Action::SearchBegin { reverse: false }
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('n'))),
            Action::SearchNext
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('N'))),
            Action::SearchPrev
        ));
    }

    #[test]
    fn question_mark_toggles_help() {
        assert!(matches!(
            map_normal(press(KeyCode::Char('?'))),
            Action::ToggleHelp
        ));
    }

    #[test]
    fn jump_end_is_capital_g_and_lowercase_g_is_noop() {
        assert!(matches!(
            map_normal(press(KeyCode::Char('G'))),
            Action::JumpEnd
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('g'))),
            Action::None
        ));
    }

    #[test]
    fn heading_nav_and_toc() {
        assert!(matches!(
            map_normal(press(KeyCode::Char(']'))),
            Action::NextHeading
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('['))),
            Action::PrevHeading
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('t'))),
            Action::ToggleToc
        ));
    }

    #[test]
    fn ctrl_modifier_variants_match_bare_letter() {
        // Design doc promises Ctrl-d/u/f/b as aliases. The match expression
        // on key.code (without modifier guards) gives us this for free, but
        // a regression test makes the contract explicit so a future refactor
        // that adds `if !ctrl` guards won't silently break vim muscle memory.
        assert!(matches!(
            map_normal(press_ctrl(KeyCode::Char('d'))),
            Action::ScrollHalfPage(1)
        ));
        assert!(matches!(
            map_normal(press_ctrl(KeyCode::Char('u'))),
            Action::ScrollHalfPage(-1)
        ));
        assert!(matches!(
            map_normal(press_ctrl(KeyCode::Char('f'))),
            Action::ScrollPage(1)
        ));
        assert!(matches!(
            map_normal(press_ctrl(KeyCode::Char('b'))),
            Action::ScrollPage(-1)
        ));
    }

    #[test]
    fn link_and_history() {
        assert!(matches!(
            map_normal(press(KeyCode::Enter)),
            Action::OpenLink
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('o'))),
            Action::Back
        ));
        assert!(matches!(
            map_normal(press(KeyCode::Char('i'))),
            Action::Forward
        ));
    }
}
