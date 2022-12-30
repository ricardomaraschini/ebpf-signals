# Signals

An eBPF based engine to capture and forward POSIX signals being sent. This project
hooks an eBPF program to Kernel and make the signals available through a Unix socket
on the filesystem.

## Prerequisites

Before getting started you will need the Rust stable and nightly toolchains installed
on your system. This is easily achieved with rustup:

```
$ rustup install stable
$ rustup toolchain install nightly --component rust-src
```

Once you have the Rust toolchains installed, you must also install bpf-linker. The
linker depends on LLVM, and it can be built against the version shipped with the rust
toolchain if you are running on a linux `x86_64` system with:

```
$ cargo install bpf-linker
```

*NOTE* If you are using Debian you will also need to have the package `build-essential`
installed on the system.

## Building

To build the debug version (version with debug symbols and all):

```
$ make
```

To build the release (the stripped down version):

```
$ make release
```

## Run

To run the debug version:

```bash
$ sudo RUST_LOG=debug ./target/debug/signals
```

To run the "released" version:

```bash
$ sudo ./target/release/signals
```

## Examples

In one terminal, run the `signals` binary as root:

```
$ sudo ./target/debug/signals
[2022-11-30T19:54:02Z INFO  signals]        / /\        /\ \       /\ \        
[2022-11-30T19:54:02Z INFO  signals]       / /  \       \ \ \     /  \ \       
[2022-11-30T19:54:02Z INFO  signals]      / / /\ \__    /\ \_\   / /\ \_\      
[2022-11-30T19:54:02Z INFO  signals]     / / /\ \___\  / /\/_/  / / /\/_/      
[2022-11-30T19:54:02Z INFO  signals]     \ \ \ \/___/ / / /    / / / _____     
[2022-11-30T19:54:02Z INFO  signals]      \ \ \      / / /    / / / /\_____\   
[2022-11-30T19:54:02Z INFO  signals]  _    \ \ \    / / /    / / /  \/____ /   
[2022-11-30T19:54:02Z INFO  signals] /_/\__/ / /___/ / /__  / / /_____/ / /    
[2022-11-30T19:54:02Z INFO  signals] \ \/___/ //\__\/_/___\/ / /______\/ /     
[2022-11-30T19:54:02Z INFO  signals]  \_____\/ \/_________/\/___________/      
[2022-11-30T19:54:02Z INFO  signals] signal emitter development version 
[2022-11-30T19:54:02Z INFO  signals] unix socket listening on /var/run/signals
[2022-11-30T19:54:02Z INFO  signals] loading and attaching to the ebpf program
[2022-11-30T19:54:02Z INFO  signals] awaiting for new connections
```

Signals are now available under `/var/run/signals` socket, you can receive them
by listening on that socket, as an example let's use `nc` (note that not all
`nc` versions have support for Unix Sockets). In a new terminal run:

```
$ sudo nc.openbsd -U /var/run/signals
{"pid":671,"signr":9}
{"signr":17,"pid":668}
{"signr":17,"pid":679}
{"signr":1,"pid":679}
{"pid":678,"signr":17}
{"signr":17,"pid":672}
{"signr":17,"pid":1}
{"pid":341,"signr":17}
{"pid":668,"signr":28}
{"pid":668,"signr":17}
{"pid":668,"signr":1}
{"pid":667,"signr":17}
{"signr":17,"pid":661}
{"signr":17,"pid":1}
{"signr":17,"pid":341}
{"pid":405,"signr":28}
...
```
