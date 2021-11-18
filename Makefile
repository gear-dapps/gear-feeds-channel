.PHONY: all prepare

all:
	cargo +nightly build --target wasm32-unknown-unknown --release
	wasm-proc --path ./target/wasm32-unknown-unknown/release/gear_feeds_channel.wasm
	ls -la ./target/wasm32-unknown-unknown/release/gear_feeds_channel*.wasm

prepare:
	rustup update
	rustup update nightly
	rustup target add wasm32-unknown-unknown --toolchain nightly
	cargo install --git https://github.com/gear-tech/gear wasm-proc
