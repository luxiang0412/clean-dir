BINARY := clean-dir
VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
DIST_DIR := dist

# Targets buildable via cross (Linux containers)
CROSS_TARGETS := \
	x86_64-unknown-linux-gnu \
	aarch64-unknown-linux-gnu \
	x86_64-pc-windows-gnu

# macOS targets (only buildable on macOS, no cross Docker image available)
MACOS_TARGETS := \
	x86_64-apple-darwin \
	aarch64-apple-darwin

ALL_TARGETS := $(CROSS_TARGETS) $(MACOS_TARGETS)

.PHONY: build release clean fmt lint test check install all all-cross all-macos help

## Development

build:  ## Build debug binary
	cargo build

release:  ## Build optimized release binary
	cargo build --release

run:  ## Run with arguments (usage: make run ARGS="~/projects --dry-run")
	cargo run -- $(ARGS)

test:  ## Run tests
	cargo test

check:  ## Run cargo check
	cargo check

fmt:  ## Format code
	cargo fmt

lint:  ## Run clippy lints
	cargo clippy -- -D warnings

## Installation

install: release  ## Install to ~/.cargo/bin
	cargo install --path .

## Cross-compilation

$(CROSS_TARGETS):  ## Build for a cross target (Linux/Windows, requires cross + Docker)
	cross build --release --target $@
	@mkdir -p $(DIST_DIR)
	@if echo "$@" | grep -q windows; then \
		cp target/$@/release/$(BINARY).exe $(DIST_DIR)/$(BINARY)-$(VERSION)-$@.exe; \
	else \
		cp target/$@/release/$(BINARY) $(DIST_DIR)/$(BINARY)-$(VERSION)-$@; \
	fi
	@echo "Built: $(DIST_DIR)/$(BINARY)-$(VERSION)-$@"

$(MACOS_TARGETS):  ## Build for macOS (only works on macOS host)
	cargo build --release --target $@
	@mkdir -p $(DIST_DIR)
	@cp target/$@/release/$(BINARY) $(DIST_DIR)/$(BINARY)-$(VERSION)-$@
	@echo "Built: $(DIST_DIR)/$(BINARY)-$(VERSION)-$@"

all-cross: $(CROSS_TARGETS)  ## Build Linux + Windows targets (requires cross + Docker)

all-macos: $(MACOS_TARGETS)  ## Build macOS targets (only on macOS host)

all: $(ALL_TARGETS)  ## Build all platforms

## Packaging

dist: all-cross  ## Build cross targets and create archives
	@cd $(DIST_DIR) && \
	for f in $(BINARY)-$(VERSION)-*; do \
		if echo "$$f" | grep -q '.exe$$'; then \
			zip "$${f%.exe}.zip" "$$f"; \
		else \
			chmod +x "$$f"; \
			tar czf "$$f.tar.gz" "$$f"; \
		fi; \
	done
	@echo "Archives created in $(DIST_DIR)/"

## Cleanup

clean:  ## Remove build artifacts
	cargo clean
	rm -rf $(DIST_DIR)

## Help

help:  ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*##' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
