##############################################################################
# Set up
##############################################################################
.PHONY: install
install: deps direnv

.PHONY: deps
deps: # Install dependencies
	cargo fetch

.PHONY: direnv
direnv: # Set environment variables
	direnv allow .

##############################################################################
# Development
##############################################################################
# Default to the current day
day ?= $(shell date +%d)

.PHONY: format
format: # Format code
	@cargo fmt

.PHONY: lint
lint: # Static code analysis
	@cargo clippy --all-targets

.PHONY: fix
fix: # Static code analysis
	@cargo clippy --fix --all-targets --allow-dirty

.PHONY: run
run: # Run day XX
	@cargo run --bin day$(day)

.PHONY: test
test: # Run tests in day XX
	@cargo test --bin day$(day)

.PHONY: testall
testall: # Run all tests
	@cargo test

.PHONY: gen
gen: # Generate next day
	@cargo run --bin gen -- $(day)
