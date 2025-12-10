format:
	cargo fmt

check: format
	cargo clippy --fix --allow-dirty -- -D warnings

test: check
	cargo nextest run --config-file nextest.toml --no-capture --no-fail-fast

machete:
	cargo machete --with-metadata

tarpaulin:
	cargo tarpaulin --all-targets -- --test-threads 1 --no-capture
