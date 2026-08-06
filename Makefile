.PHONY: help fmt fmt-check lint test build build-release check all coverage coverage-html coverage-lcov large-fixture

CARGO ?= cargo

help:
	@echo "Targets:"
	@echo "  fmt        - format all code (cargo fmt --all)"
	@echo "  fmt-check  - verify formatting without changes (CI gate)"
	@echo "  lint       - clippy on all targets, warnings as errors (CI gate)"
	@echo "  test       - cargo test"
	@echo "  build      - cargo build --all-targets"
	@echo "  build-release - cargo build --release"
	@echo "  check      - fmt-check + lint + test (run before pushing)"
	@echo "  all        - fmt + check + build"
	@echo "  coverage      - test coverage summary in the terminal (cargo-llvm-cov)"
	@echo "  coverage-html - generate an HTML coverage report under target/llvm-cov/html"
	@echo "  coverage-lcov - emit lcov.info for external tooling"
	@echo "  large-fixture - (re)generate the gitignored stress fixture for manual perf runs"

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all --check

lint:
	$(CARGO) clippy --all-targets -- -D warnings

test:
	$(CARGO) test

build:
	$(CARGO) build --all-targets

build-release:
	$(CARGO) build --release

check: fmt-check lint test

all: fmt check build

coverage:
	$(CARGO) llvm-cov --all-targets

coverage-html:
	$(CARGO) llvm-cov --all-targets --html

coverage-lcov:
	$(CARGO) llvm-cov --all-targets --lcov --output-path lcov.info

large-fixture:
	./scripts/gen-large-fixture.sh > fixtures/specialized/large.md
	@echo "Generated fixtures/specialized/large.md (gitignored). Remove it when done perf-testing: rm fixtures/specialized/large.md"
