# BITGATEWAY

[简体中文](./README.md) | English

A simple desktop app for logging in to and out of the BIT campus network gateway (10.0.0.55). Built with Dioxus.

## Download

### Prebuilt binaries

Download the latest version from [GitHub Releases](../../releases).

### Build from source

Install Rust nightly. The repository's `rust-toolchain.toml` will automatically select the pinned toolchain:

```sh
rustup toolchain install nightly
```

On Linux, install the Dioxus Desktop dependencies first:

```sh
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  libxdo-dev \
  librsvg2-dev \
  patchelf
```

Clone the repository and run the app:

```sh
git clone https://github.com/so1ve/bitgateway.git
cd bitgateway
cargo run -p bitgateway
```

To produce a desktop bundle, install the Dioxus CLI and run:

```sh
cargo install dioxus-cli --version 0.7.6 --locked
cd crates/bitgateway
dx bundle --release --platform desktop
```

For platform-specific details, see the [official Dioxus documentation](https://dioxuslabs.com/).

### Recommended IDE setup

- [VS Code](https://code.visualstudio.com/) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Acknowledgements

`bitgateway-client` is implemented with reference to [bitsrun-rs](https://github.com/spencerwooo/bitsrun-rs).

## License

[MIT](./LICENSE). Made with ❤️ by [Ray](https://github.com/so1ve)
