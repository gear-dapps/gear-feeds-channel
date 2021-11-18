# 📰 Gear Feeds Channel

An example of a feeds channel program.

## Getting Started

### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install --git https://github.com/gear-tech/gear wasm-proc
```

... or ...

```shell
make prepare
```

### ✍️ Edit the program

Open [`src/lib.rs`](src/lib.rs) and address all `TODO`s there.

### 🏗️ Build

```shell
cargo +nightly build --target wasm32-unknown-unknown --release
wasm-proc --path ./target/wasm32-unknown-unknown/release/gear_feeds_channel.wasm
```

... or ...

```shell
make
```

## License

The source code is licensed under [GPL v3.0 license](LICENSE).
