.PHONY: test lint format

test:
	cargo test

lint:
	cargo clippy --all-targets -- -D warnings

format:
	cargo fmt --all
