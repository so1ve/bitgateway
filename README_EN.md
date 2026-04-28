# BITGATEWAY

[简体中文](./README.md) | English

A simple desktop app for logging in to and out of the BIT campus network gateway (10.0.0.55). Built with Dioxus.

## Download

### Prebuilt binaries

Download the latest version from [GitHub Releases](../../releases).

On startup, the app checks the latest GitHub Release. If a newer version is available, it prompts you to open the download page; it will not download or install updates automatically.

### Install with Cargo

If Rust is already installed, you can install the app directly from crates.io:

```sh
cargo install bitgateway
```

On Linux, installing with Cargo still requires the Dioxus Desktop dependencies listed below.

### Build from source

Install Rust nightly. The repository's `rust-toolchain.toml` will automatically select the pinned toolchain:

```sh
rustup toolchain install nightly
```

On Linux, install the Dioxus Desktop dependencies first:

```sh
sudo apt-get update
sudo apt-get install -y \
  pkg-config \
  libglib2.0-dev \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  libxdo-dev \
  libfuse2 \
  librsvg2-dev \
  patchelf
```

Clone the repository and run the app:

```sh
git clone https://github.com/so1ve/bitgateway.git
cd bitgateway
cargo run -p bitgateway
```

To produce a desktop bundle, it's recommended to install `cargo-binstall` and use it to install the Dioxus CLI. Dioxus 0.7 desktop bundling needs an explicit platform and package type; otherwise `--out-dir` may only create an empty directory:

```sh
cargo binstall dioxus-cli@0.7.6 --locked # or use cargo install
cd crates/bitgateway

# Windows: create a portable exe
cargo build --release -p bitgateway --locked
# Output: target/release/bitgateway.exe

# Windows: create an NSIS installer
dx bundle --release --platform windows --package-types nsis --locked

# macOS: create a DMG and .app.tar.gz
dx bundle --release --platform macos \
  --package-types macos \
  --package-types dmg \
  --package-types updater \
  --locked

# Linux: create AppImage, deb, and rpm packages
dx bundle --release --platform linux \
  --package-types appimage \
  --package-types deb \
  --package-types rpm \
  --locked
```

`Dioxus.toml` is configured to place release bundles in the repository-level `dist/` directory. Dioxus' `updater` package type creates the macOS `.app.tar.gz` archive from the `.app` bundle. The Windows installer does not embed the WebView2 offline runtime, keeping the installer small; systems without WebView2 Runtime need to install it first.

For platform-specific details, see the [official Dioxus documentation](https://dioxuslabs.com/).

### Recommended IDE setup

- [VS Code](https://code.visualstudio.com/) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Acknowledgements

`bitgateway-client` is implemented with reference to [bitsrun-rs](https://github.com/spencerwooo/bitsrun-rs).

## License

[MIT](./LICENSE). Made with ❤️ by [Ray](https://github.com/so1ve)
