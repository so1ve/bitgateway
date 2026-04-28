# BITGATEWAY

简体中文 | [English](./README_EN.md)

一个简单的 BIT 校园网（10.0.0.55）登录/登出桌面应用。使用 Dioxus 构建。

## 下载

### 预编译二进制文件

从 [GitHub Releases](../../releases) 下载最新版本。

### 使用 Cargo 安装

如果已经安装 Rust，也可以直接从 crates.io 安装：

```sh
cargo install bitgateway
```

Linux 上通过 Cargo 安装时仍需要先安装下方的 Dioxus Desktop 依赖。

### 从源码编译

准备 Rust nightly（仓库中的 `rust-toolchain.toml` 会自动选择工具链）：

```sh
rustup toolchain install nightly
```

Linux 需要额外安装 Dioxus Desktop 依赖：

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

克隆仓库并运行：

```sh
git clone https://github.com/so1ve/bitgateway.git
cd bitgateway
cargo run -p bitgateway
```

如果需要生成桌面安装包，建议先安装 `cargo-binstall`，再用它安装 Dioxus CLI。Dioxus 0.7 的桌面打包需要显式指定平台和安装包格式，否则 `--out-dir` 可能只创建空目录：

```sh
cargo binstall dioxus-cli@0.7.6 --locked # 或者用 cargo install
cd crates/bitgateway

# Windows：生成 portable exe
cargo build --release -p bitgateway --locked
# 产物：target/release/bitgateway.exe

# Windows：生成 NSIS 安装包
dx bundle --release --platform windows --package-types nsis --locked

# macOS：生成 DMG 和 .app.tar.gz
dx bundle --release --platform macos \
  --package-types macos \
  --package-types dmg \
  --package-types updater \
  --locked

# Linux：生成 AppImage、deb 和 rpm
dx bundle --release --platform linux \
  --package-types appimage \
  --package-types deb \
  --package-types rpm \
  --locked
```

`Dioxus.toml` 已配置输出目录为仓库根目录的 `dist/`。Dioxus 的 `updater` package type 会为 macOS `.app` 生成 `.app.tar.gz`。Windows 安装包不会内置 WebView2 离线运行时，因此体积较小；如果目标系统缺少 WebView2 Runtime，需要先安装它。

更多平台相关说明请参考 [Dioxus 官方文档](https://dioxuslabs.com/)。

### 推荐的 IDE 设置

- [VS Code](https://code.visualstudio.com/) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 致谢

参考 [bitsrun-rs](https://github.com/spencerwooo/bitsrun-rs) 实现了 `bitgateway-client`。

## License

[MIT](./LICENSE). Made with ❤️ by [Ray](https://github.com/so1ve)
