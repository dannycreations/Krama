format:
	cargo fmt

check: format
	cargo clippy --fix --allow-dirty -- -D warnings

test: check
	cargo test -- --no-capture

krama_run:
	cargo run run

krama_test:
	cargo run test

cargo_machete:
	cargo machete

cargo_tarpaulin:
	cargo tarpaulin --all-targets -- --test-threads 1 --no-capture
