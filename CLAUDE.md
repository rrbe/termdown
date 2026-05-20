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

## Git workflow

**Do not commit or push to `master` directly.** All changes land on `master` via PR merges. For any code or docs change:

1. Create a feature branch (e.g. `feat/...`, `fix/...`, `docs/...`).
2. Commit there, push the branch, open a PR.
3. Never run `git commit` while `HEAD` is on `master`, and never run `git push origin master` — even for "small" doc tweaks. Committing to local master (even without pushing) tends to contaminate the merge base of later feature branches.

If you notice you're on `master` with uncommitted changes, stash them, switch to a new branch, and pop the stash before committing.

### Exception: version bumps

Version bumps are the **only** allowed direct-to-`master` commits. They must not ride along inside a feature/fix PR — keep them as standalone commits so the release history stays readable.

When the feature/fix PRs that make up a release have all merged into `master`:

1. `git checkout master && git pull` to land on the merged tip.
2. Edit `Cargo.toml` (and `Cargo.lock` — `cargo build` will refresh it) to the new version.
3. Lock the `CHANGELOG.md` `[Unreleased]` section to `[X.Y.Z] - YYYY-MM-DD`.
4. `git commit -m "chore: bump version to X.Y.Z"` on `master`.
5. `git tag vX.Y.Z` and `git push origin master --follow-tags`.

The `release.yml` workflow triggers on `v*` tag pushes and produces the GitHub Release (cross-platform binaries + checksums). Pushing the tag is therefore both the version stamp **and** the release trigger — no extra step.
