# Testing

How termdown's tests are organized and run. Every command goes through the
[`Makefile`](../Makefile) so local runs match CI exactly (see the project's
`CLAUDE.md` for the rule — don't call `cargo test`/`clippy` directly).

## Running

| Command | What it does |
|---|---|
| `make test` | Run the whole suite |
| `make check` | `fmt-check` + `lint` + `test` — the CI gate; run before pushing |
| `make coverage` | Local coverage summary via `cargo-llvm-cov` (on-demand, not a CI gate) |

## Test layout

- **Unit tests** — inline `#[cfg(test)]` modules in `src/*.rs` (config parsing, ANSI/width helpers, font ranges, frontmatter parsing, layout, …).
- **`tests/cli.rs`** — black-box CLI: `--help`/`--version`, stdin/file input, missing-file errors, the unsupported-terminal warning.
- **`tests/snapshots.rs`** — byte-level snapshot of cat-mode stdout for each fixture, with Kitty image payloads collapsed to `<IMG>` (PNG bytes are font/OS-dependent, so only the *position* of an image is compared, not its pixels). Background: [`TERMINAL_PROTOCOLS.md`](TERMINAL_PROTOCOLS.md).
- **`tests/headings.rs`** — parses the Kitty APC frames out of stdout, decodes the heading PNGs, and asserts dimensions / non-blank pixels / H1 > H2 > H3 scaling.

Tests drive the compiled binary through `tests/common/mod.rs::run_termdown`,
which forces a ghostty-like terminal (`TERM_PROGRAM=ghostty`, so Kitty
emission is on), `--theme dark`, and clears `HOME`/`XDG_CONFIG_HOME` so a
developer's own config can't leak in.

### Fixtures

- `fixtures/*.md` and `fixtures/specialized/*.md` — rendering inputs. Their snapshot expectations live alongside in `fixtures/expected/**/*.ansi`.
- `fixtures/links/` — a small `.md` link graph for **manual** QA of TUI link-following; `index.md` documents the steps. Not wired into automated tests.

### Regenerating snapshots

When a rendering change is intentional, the snapshot test fails and writes the
*actual* output to a temp file, printing both paths. Review the diff
intent-first, then accept it by copying the temp file over the expected one,
e.g.:

```sh
cp "$TMPDIR/termdown-snapshot-supported-syntax.ansi" fixtures/expected/supported-syntax.ansi
make test   # confirm green
```

## Performance / stress testing

`fixtures/specialized/large.md` is a deterministic stress fixture — 1 H1 + 500
H2 sections (mixed CN/EN paragraphs, nested lists, 3×3 tables, fenced Rust code)
plus a ~200-row tail table: ~14.7k lines / ~460 KB. It exists only to eyeball
that termdown stays snappy on a large document; it is **not** wired into the
automated suite.

Because it is generated build output with no automated consumer, it is **not
committed** — it's gitignored and produced on demand. The full workflow is
**generate → test → delete**:

```sh
# 1. Generate (~1.7s; deterministic, so a clean re-run is byte-identical)
make large-fixture
#   or directly: ./scripts/gen-large-fixture.sh > fixtures/specialized/large.md

# 2. Test against it manually
cargo build --release
time ./target/release/termdown --cat fixtures/specialized/large.md > /dev/null   # cat throughput
./target/release/termdown fixtures/specialized/large.md                          # TUI: scroll / search / heading-jump feel

# 3. Delete when done (it's large and gitignored anyway)
rm fixtures/specialized/large.md
```

Size/shape is tunable via env vars read by the script: `SECTIONS` (default
500), `H3_EVERY` (50), `TAIL_TABLE_ROWS` (200) — e.g.
`SECTIONS=2000 make large-fixture` for an even larger document. After changing
`scripts/gen-large-fixture.sh`, re-run it to regenerate.
