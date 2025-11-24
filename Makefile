format:
	cargo fmt

check: format
	cargo clippy --fix --allow-dirty -- -D warnings

test: check
	cargo test -- --no-capture

spec_run:
	cargo run run

spec_test:
	cargo run test
