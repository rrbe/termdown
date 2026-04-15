# Test Plan

## Current Status

The repository currently has two kinds of automated tests:

- Unit tests in source files such as `src/style.rs`, `src/markdown.rs`, and `src/font.rs`
- Black-box CLI tests in `tests/cli.rs`

These existing tests cover:

- ANSI stripping and display-width handling
- Markdown text wrapping and table alignment
- Emoji font selection
- CLI help/version output
- Basic stdin and file input paths
- Missing-file error handling
- Warning output on unsupported terminals

## Important Gap

The project's signature feature is large image-rendered H1-H3 headings via the Kitty graphics protocol.

That feature is **not yet covered by a true integration test**.

The current CLI tests do not yet:

- read a real Markdown fixture and compare the full rendered output against an expected fixture
- verify that H1-H3 actually produce Kitty graphics protocol output
- extract the embedded PNG payload from terminal output
- decode the PNG and assert image properties
- compare heading output against stable golden fixtures

So while the current suite does exercise the CLI, it does **not** yet prove that the main heading-rendering pipeline works end to end.

## What "Real" Integration Testing Should Mean Here

A real integration test for this project should validate the full path:

`Markdown heading -> heading rasterization -> PNG generation -> Kitty protocol output`

At minimum, that means testing that:

1. A Markdown fixture containing H1-H3 headings is rendered by the compiled binary.
2. The output contains Kitty graphics protocol payloads instead of only plain ANSI text.
3. The PNG payload can be extracted and decoded successfully.
4. The decoded image is non-empty and matches expected characteristics.

## Proposed Phases

### Phase 1: Fixture-Based Text Integration Tests

Add fixture-based CLI tests for stable text output only.

Scope:

- Read Markdown from files under `fixtures/`
- Run the binary in a controlled environment
- Strip ANSI when appropriate
- Compare output against checked-in expected text fixtures

Recommended targets:

- paragraphs
- lists
- blockquotes
- tables
- horizontal rules
- H4-H6 headings

This will give true fixture-based regression coverage for the text-rendering parts of the program.

### Phase 2: Heading Rendering Integration Tests

Add integration tests specifically for H1-H3 image-rendered headings.

Scope:

- Use Markdown fixtures containing representative headings
- Capture stdout from the compiled binary
- Detect Kitty graphics protocol escape sequences
- Extract the embedded base64 PNG payload
- Decode the PNG
- Assert basic image properties:
  - width and height are greater than zero
  - the image is not fully transparent
  - different heading levels produce different image sizes

Recommended heading cases:

- ASCII heading
- CJK heading
- mixed Latin/CJK heading
- emoji-containing heading
- long heading

### Phase 3: Stable Golden Validation for Headings

Once heading rendering is made deterministic enough, add stronger regression checks.

Options:

- compare decoded PNG hashes
- compare image dimensions plus sampled pixel statistics
- compare against checked-in golden PNG fixtures

This phase should only be enabled once font selection and rendering are stable across environments.

## Main Technical Risk

Heading rendering currently depends on system font resolution.

That makes full image snapshots potentially unstable across:

- operating systems
- installed fonts
- font versions
- fontconfig/Core Text differences

Because of that, heading snapshot tests may be flaky unless the test environment is made deterministic.

## Recommended Prerequisite for Reliable Heading Tests

Before adding strict heading snapshots, make the heading test environment font-stable.

Possible approaches:

- allow tests to point configuration at repo-owned font files
- add a test-only font loading path that prefers bundled fonts
- introduce a deterministic fixture config used only in tests

Without this, true heading integration tests can still check for protocol and decodable PNG output, but not reliable pixel-perfect snapshots.

## Short-Term Next Step

The next useful step is:

1. convert part of the current CLI coverage to fixture-based expected-output tests for stable text rendering
2. add a first headings integration test that extracts and decodes the Kitty PNG payload, without yet requiring exact image snapshots

That would be the first version of a real end-to-end test for the project's main feature.
