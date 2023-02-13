# Environment setup
This document provides guidelines on how to set up the environment for Linux kernel module, including loadable kernel module (LKM), development in Rust: Linux kernel source code, packages, Rust tools.

## 0. Pre-requisites
These are the parameters that were tested on:
```
VMWare Fusion 12.2.3 build-19436697 (or any other hypervisor)
VM: Ubuntu 64-bit 22.04.1
Hard disk: at least 40 GB
Processor cores: the more the better
```

## 1. Install the Linux kernel (release for Rust)
1) Check the latest release of Linux kernel from [Rust-for-Linux repository][https://github.com/Rust-for-Linux/linux/tags] and get the kernel tarball in a directory where you have permissions (`linux-rust-<..>.tar.gz`)
2) Untar the source code:
```
tar xf linux-rust-<..>
```
## 2. Update the system and get necessary packages for compiling the kernel
```
sudo apt-get update && sudo apt-get -y install git fakeroot build-essential ncurses-dev xz-utils libssl-dev bc curl flex cmake bison pahole jfsutils reiserfsprogs xfsprogs btrfs-progs quota nfs-common pkg-config fuse libelf-dev lld llvm
```
## 3. Install Rust
Run these steps or follow [the official guide from Rust][https://www.rust-lang.org/tools/install] and the steps of [Rust-for-Linux quick start][https://docs.kernel.org/rust/quick-start.html#requirements-building].
1) Get `rustup`
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2) Update the current terminal or start a new one to add the Rust toolchain path
```
source "$HOME/.cargo/env"
```
3) Install and set to default the latest supported `rustc` (Rust compiler)
```
// Run from inside linux-rust-<..> folder
cd linux-rust-<..>
rustup override set $(scripts/min-tool-version.sh rustc)
```
4) Add required components
```
rustup component add rust-src
```
5) Install `bindigen` tool (requires `LLVM` - installed in Section 2)
```
// Run from inside linux-rust-<..> folder
cargo install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen
```
Tools such as `rustfmt`, `clippy` and `cargo` are already included if you followed these steps.
