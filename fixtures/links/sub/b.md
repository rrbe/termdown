# Page B (nested)

You reached Page B at `fixtures/links/sub/b.md`. Relative paths here must climb one directory.

## Links

- [Back to index (`../index.md`)](../index.md)
- [Sibling Page A (`../a.md`)](../a.md)
- [External — anthropic.com](https://www.anthropic.com)

## What to verify

1. Arrival here from Page A means the resolver handled `sub/b.md` relative to `a.md`'s parent directory.
2. Pressing `o` should pop history back to Page A, not to index.
3. Pressing Enter on `../index.md` walks back up one level — the status bar should now show the index path.
