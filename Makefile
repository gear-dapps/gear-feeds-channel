.PHONY: all prepare

all:
	cargo build --release
	ls -la ./target/wasm32-unknown-unknown/release/gear_feeds_channel*.wasm

check: all
	@cargo test --workspace --release

prepare:
	rustup toolchain add nightly
	rustup target add wasm32-unknown-unknown --toolchain nightly
