//! TUI-side Kitty image lifecycle. Transmits each heading PNG to the
//! terminal exactly once (cached by id), and on each frame diffs the
//! desired placement map against the currently-placed state to emit only
//! the minimum `place`/`delete_placement` commands.

use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

use crate::render;

#[derive(Default)]
pub struct ImageLifecycle {
    /// Ids that have been transmitted to the terminal (with cached data).
    transmitted: HashSet<u32>,
    /// Current placements: id → (col, row).
    placed: HashMap<u32, (u16, u16)>,
}

impl ImageLifecycle {
    /// Transmit `png` under `id` if not already transmitted. Idempotent.
    pub fn register<W: Write>(&mut self, w: &mut W, id: u32, png: &[u8]) -> io::Result<()> {
        if self.transmitted.contains(&id) {
            return Ok(());
        }
        render::transmit(w, id, png)?;
        self.transmitted.insert(id);
        Ok(())
    }

    /// Diff `desired` against `placed`, emit delete/place commands to reconcile.
    ///
    /// For ids whose position changed, Kitty treats a second `place` as
    /// stacking a new placement rather than moving — so a `delete_placement`
    /// is always required first when an id's (col, row) differs.
    pub fn sync<W: Write>(
        &mut self,
        w: &mut W,
        desired: &HashMap<u32, (u16, u16)>,
    ) -> io::Result<()> {
        // 1. Delete placements that are no longer desired (entirely off-screen).
        let to_remove: Vec<u32> = self
            .placed
            .keys()
            .filter(|id| !desired.contains_key(id))
            .copied()
            .collect();
        for id in to_remove {
            render::delete_placement(w, id)?;
            self.placed.remove(&id);
        }

        // 2. Place new ids, or re-place moved ids.
        for (&id, &(col, row)) in desired {
            match self.placed.get(&id) {
                Some(&pos) if pos == (col, row) => {
                    // unchanged
                }
                Some(_) => {
                    render::delete_placement(w, id)?;
                    render::place(w, id, col, row)?;
                    self.placed.insert(id, (col, row));
                }
                None => {
                    render::place(w, id, col, row)?;
                    self.placed.insert(id, (col, row));
                }
            }
        }

        Ok(())
    }

    /// Delete every placement + cached image data this client created.
    /// Clears our tracking. Called on TUI exit.
    pub fn cleanup<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        render::delete_all_for_client(w)?;
        self.placed.clear();
        self.transmitted.clear();
        Ok(())
    }

    /// Delete all current placements (keeping cached image data so they can
    /// be re-placed without re-transmitting). Resets `placed` tracking so
    /// the next `sync` treats every desired image as new and emits a fresh
    /// place command. Used after `terminal.clear()` to force all images to
    /// re-render from scratch while avoiding an expensive PNG retransmit.
    pub fn reset_placements<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        for &id in self.placed.keys() {
            render::delete_placement(w, id)?;
        }
        self.placed.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_is_idempotent() {
        let mut lc = ImageLifecycle::default();
        let mut buf = Vec::new();
        lc.register(&mut buf, 1, b"payload").unwrap();
        let first_len = buf.len();
        lc.register(&mut buf, 1, b"payload").unwrap();
        assert_eq!(
            buf.len(),
            first_len,
            "second register of same id should emit nothing"
        );
    }

    #[test]
    fn sync_enters_new_moves_and_leaves() {
        let mut lc = ImageLifecycle::default();
        let mut buf = Vec::new();

        lc.register(&mut buf, 1, b"png").unwrap();
        buf.clear();

        // Enter at (5, 10).
        let mut desired = HashMap::new();
        desired.insert(1u32, (5u16, 10u16));
        lc.sync(&mut buf, &desired).unwrap();
        let s = String::from_utf8(buf.clone()).unwrap();
        // New format: CUP (1-indexed row+1=11, col+1=6) followed by a=p,i=1.
        assert!(
            s.contains("\x1b[11;6H") && s.contains("a=p,i=1"),
            "expected initial place, got: {s:?}"
        );
        assert!(!s.contains("a=d,d=i,i=1"), "no delete expected: {s:?}");

        // Move to (5, 8) — needs delete + place.
        desired.insert(1, (5, 8));
        buf.clear();
        lc.sync(&mut buf, &desired).unwrap();
        let s = String::from_utf8(buf.clone()).unwrap();
        assert!(
            s.contains("a=d,d=i,i=1"),
            "expected delete before move: {s:?}"
        );
        // New format: CUP (1-indexed row+1=9, col+1=6) followed by a=p,i=1.
        assert!(
            s.contains("\x1b[9;6H") && s.contains("a=p,i=1"),
            "expected re-place at new pos: {s:?}"
        );

        // Stay at (5, 8) — no output.
        buf.clear();
        lc.sync(&mut buf, &desired).unwrap();
        assert!(buf.is_empty(), "unchanged position should emit nothing");

        // Leave — just delete.
        desired.remove(&1);
        buf.clear();
        lc.sync(&mut buf, &desired).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("a=d,d=i,i=1"));
        assert!(!s.contains("a=p,i=1"), "no place expected on leave: {s:?}");
    }

    #[test]
    fn cleanup_emits_delete_all() {
        let mut lc = ImageLifecycle::default();
        let mut buf = Vec::new();
        lc.register(&mut buf, 1, b"png").unwrap();
        let mut desired = HashMap::new();
        desired.insert(1u32, (0, 0));
        lc.sync(&mut buf, &desired).unwrap();
        buf.clear();

        lc.cleanup(&mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("a=d,d=A"));
        assert!(lc.placed.is_empty());
        assert!(lc.transmitted.is_empty());
    }
}
