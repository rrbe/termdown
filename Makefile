.PHONY: help fmt fmt-check lint test build check all

CARGO ?= cargo

help:
	@echo "Targets:"
	@echo "  fmt        - format all code (cargo fmt --all)"
	@echo "  fmt-check  - verify formatting without changes (CI gate)"
	@echo "  lint       - clippy on all targets, warnings as errors (CI gate)"
	@echo "  test       - cargo test"
	@echo "  build      - cargo build --all-targets"
	@echo "  check      - fmt-check + lint + test (run before pushing)"
	@echo "  all        - fmt + check + build"

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

check: fmt-check lint test

all: fmt check build
