<div align="center">

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]

[build_badge]: https://github.com/gear-tech/gear-feeds-channel/workflows/Build/badge.svg
[build_href]: https://github.com/gear-tech/gear-feeds-channel/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-GPL%203.0-success
[lic_href]: https://github.com/gear-tech/gear-feeds-channel/blob/master/LICENSE

</div>

<p align="center">
  <a href="https://gitpod.io/#https://github.com/gear-tech/gear-feeds-channel">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="GEAR">
  </a>
</p>

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
