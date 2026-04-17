# termdown — Project Instructions

## Build / lint / test must go through the Makefile

This project uses a `Makefile` as the **single source of truth** for build, format, lint, and test commands. CI (`.github/workflows/ci.yml`) invokes the same targets, so anything that passes locally via `make` will pass CI.

**Do not** invoke `cargo fmt`, `cargo clippy`, `cargo build`, or `cargo test` directly when verifying work. Always go through the Makefile so flags stay consistent with CI (e.g. `clippy --all-targets -- -D warnings`).

### Targets

- `make fmt` — auto-format all code
- `make fmt-check` — formatting gate (CI uses this)
- `make lint` — clippy with warnings-as-errors (CI uses this)
- `make test` — run the test suite
- `make build` — `cargo build --all-targets`
- `make check` — `fmt-check` + `lint` + `test` (run this before declaring work done or pushing)
- `make all` — `fmt` + `check` + `build`

### Required workflow before finishing any code change

1. Make your edits.
2. Run `make fmt` to auto-format.
3. Run `make check` and ensure it exits clean.
4. Only then report the task as complete.

If `make check` fails, fix the underlying issue — do not bypass with `#[allow(...)]` or `--no-verify` unless the user explicitly asks for it.

### When you add a new build/lint/test command

Add it as a Makefile target first, then reference the target from CI. Never let CI and local commands diverge.
