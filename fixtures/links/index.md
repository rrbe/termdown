# Link Navigation Test — Index

This is the **entry point** for testing local `.md` link navigation in TUI mode.

## Expected keybindings

- `Enter` — follow link under/near cursor. If multiple links are visible, a numeric overlay appears; press `1`–`9` to pick.
- `o` — Back (pops history).
- `i` — Forward (re-applies a previously popped entry).

## Links to try

### Single link on its own screen

Scroll so only this paragraph is visible, then press Enter:

[Go to page A](a.md)

### Multiple links — triggers the numeric overlay

When both are visible, Enter should open the numeric picker:

1. [Page A (relative)](./a.md)
2. [Page B (nested)](sub/b.md)
3. [External link — should open in browser, not in TUI](https://example.com)
4. [Broken local link — should fall back to spawn_open](does-not-exist.md)

### Back / forward flow

Suggested script:

1. From **index** → Enter on `a.md` → arrives at Page A.
2. On Page A → Enter on `sub/b.md` → arrives at Page B.
3. Press `o` → back to Page A.
4. Press `o` → back to index.
5. Press `i` → forward to Page A.
6. Press `i` → forward to Page B.

History stack should preserve scroll position per entry.
