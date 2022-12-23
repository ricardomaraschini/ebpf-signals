# Signals

An eBPF based engine to capture and process signals being sent.


## Prerequisites

1. Install a rust stable toolchain: `rustup install stable`
2. Install a rust nightly toolchain: `rustup install nightly`
3. Install bpf-linker: `cargo install bpf-linker`
4. Install rust-src: `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu`

## Build eBPF

```bash
cargo xtask build-ebpf
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag

## Build Userspace

```bash
cargo build
```

## Run

```bash
sudo RUST_LOG=debug ./target/debug/signals
```
