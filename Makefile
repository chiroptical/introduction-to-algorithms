build:
	cargo build

run: build
	cargo run

test: build
	cargo test

format:
	cargo fmt

lint:
	cargo clippy

.PHONY: build run test format lint
