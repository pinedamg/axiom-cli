.PHONY: all build test clean install-hooks help

# Default target
all: build

## Build: Compiles the Axiom CLI in debug mode
build:
	cargo build

## Test: Runs all unit and integration tests
test:
	cargo test

## Clean: Removes the target directory
clean:
	cargo clean

## Install Hooks: Sets up the git pre-commit hook for security (Secret Hunter)
install-hooks:
	@echo "Installing git hooks..."
	@ln -sf ../../scripts/pre-commit.sh .git/hooks/pre-commit
	@chmod +x scripts/pre-commit.sh
	@echo "Hooks installed successfully!"

## Help: Shows this help message
help:
	@echo "Axiom CLI Management Commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'
