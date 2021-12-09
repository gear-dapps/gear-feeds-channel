<p align="center">
  <a href="https://gitpod.io/#https://github.com/gear-tech/gear-feeds-channel">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="GEAR">
  </a>
</p>

# 📰 Gear Feeds Channel

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]

[build_badge]: https://github.com/gear-tech/gear-feeds-channel/workflows/Build/badge.svg
[build_href]: https://github.com/gear-tech/gear-feeds-channel/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-GPL%203.0-success
[lic_href]: https://github.com/gear-tech/gear-feeds-channel/blob/master/LICENSE

An example of a feeds channel program.

## Prebuilt Binaries

➡️ https://github.com/gear-tech/gear-feeds-channel/releases/tag/build

- Output WASM: [gear_feeds_channel.wasm](https://github.com/gear-tech/gear-feeds-channel/releases/download/build/gear_feeds_channel.wasm)
- Optimized WASM: [gear_feeds_channel.opt.wasm](https://github.com/gear-tech/gear-feeds-channel/releases/download/build/gear_feeds_channel.opt.wasm)
- Meta WASM: [gear_feeds_channel.meta.wasm](https://github.com/gear-tech/gear-feeds-channel/releases/download/build/gear_feeds_channel.meta.wasm)

## Building Locally

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

## Using

### 📦 Install Polkadot.js Extension

Download and install Polkadot.js browser extension: https://polkadot.js.org/extension/

### 👛 Create Account

Create a new account using Polkadot.js extension. **Don't forget to save the mnemonic seed phrase and password in a safe place.**

### ✉️ Upload the Program

- Go to https://idea.gear-tech.io/
- Connect to your account using the **Connect** button. Allow website access to your wallet in Polkadot.js extension.
- Top up yout test account using the **Get test account** button. This button can be pressed several times.
- Upload the program (`.opt.wasm`) and metadata (`.meta.wasm`) giving some meaninful name to the program and setting the gas limit to `100'000'000`. Sign the transaction using Polkadot.js extension.
- Find the program in **Recently uploaded programs** section and copy its address.

### 📒 Register your Feeds Channel in Feeds Router

- Find the Feeds Router program in the **All programs** section and open the message sending form.
- Paste your program address (copied at the previous step) as the `address` field value in the **Payload** text area.
- Set the **Gas limit** to `500'000'000` and click **Send request**. Sign the transaction using Polkadot.js extension.

### ✨ Enjoy your Program

- Go to https://workshop.gear-tech.io/
- Connect to your account using the **Connect** button. Allow website access to your wallet in Polkadot'js extension.
- Find your newly created channel.
- You can post a message, subscribe to other channels, and view their posts.

## License

The source code is licensed under [GPL v3.0 license](LICENSE).
