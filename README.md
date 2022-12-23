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

## Output

```
$ cargo build && sudo RUST_LOG=debug ./target/debug/signals                                                                                                            
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s                                             
[2022-12-23T21:52:04Z DEBUG aya::bpf] [FEAT PROBE] BPF program name support: true                         
[2022-12-23T21:52:04Z DEBUG aya::bpf] [FEAT PROBE] BTF support: true                                      
[2022-12-23T21:52:04Z DEBUG aya::bpf] [FEAT PROBE] BTF func support: true                                 
[2022-12-23T21:52:04Z DEBUG aya::bpf] [FEAT PROBE] BTF global func support: true                          
[2022-12-23T21:52:04Z DEBUG aya::bpf] [FEAT PROBE] BTF var and datasec support: true                      
[2022-12-23T21:52:04Z DEBUG aya::bpf] [FEAT PROBE] BTF float support: false                               
[2022-12-23T21:52:04Z DEBUG aya::bpf] [FEAT PROBE] BTF decl_tag support: false                            
[2022-12-23T21:52:04Z DEBUG aya::bpf] [FEAT PROBE] BTF type_tag support: false                            
[2022-12-23T21:52:04Z DEBUG aya::obj::relocation] relocating program signals function signals             
[2022-12-23T21:52:04Z DEBUG aya::obj::relocation] finished relocating program signals function signals                                                                                                               
[2022-12-23T21:52:04Z DEBUG signals::emitters] spawning task for cpu 0                                    
[2022-12-23T21:52:04Z DEBUG signals::emitters] spawning task for cpu 1                                    
[2022-12-23T21:52:04Z DEBUG signals::emitters] spawning task for cpu 2                                                                                                                                               
[2022-12-23T21:52:04Z DEBUG signals::emitters] task for cpu awaiting for events 0                         
[2022-12-23T21:52:04Z DEBUG signals::emitters] task for cpu awaiting for events 1                         
[2022-12-23T21:52:04Z DEBUG signals::emitters] spawning task for cpu 3                                    
[2022-12-23T21:52:04Z DEBUG signals::emitters] task for cpu awaiting for events 2                         
[2022-12-23T21:52:04Z DEBUG signals::emitters] task for cpu awaiting for events 3                         
[2022-12-23T21:52:04Z DEBUG signals::emitters] spawning task for cpu 4                                    
[2022-12-23T21:52:04Z DEBUG signals::emitters] spawning task for cpu 5                                    
[2022-12-23T21:52:04Z DEBUG signals::emitters] task for cpu awaiting for events 4                         
[2022-12-23T21:52:04Z DEBUG signals::emitters] spawning task for cpu 6                                    
[2022-12-23T21:52:04Z DEBUG signals::emitters] task for cpu awaiting for events 5                         
[2022-12-23T21:52:04Z DEBUG signals::emitters] spawning task for cpu 7                                    
[2022-12-23T21:52:04Z DEBUG signals::emitters] task for cpu awaiting for events 6                                                                                                                                    
[2022-12-23T21:52:04Z INFO  signals] tasks started, awaiting for events                                   
[2022-12-23T21:52:04Z DEBUG signals::emitters] task for cpu awaiting for events 7                         
[2022-12-23T21:52:06Z DEBUG signals] ebpf: enqueued signal 14 for 686                                     
[2022-12-23T21:52:06Z INFO  signals] Signal { signr: 14, pid: 686 }                                                                                                                                                  
[2022-12-23T21:52:08Z DEBUG signals] ebpf: enqueued signal 14 for 686                                     
[2022-12-23T21:52:08Z INFO  signals] Signal { signr: 14, pid: 686 }                                       
[2022-12-23T21:52:10Z DEBUG signals] ebpf: enqueued signal 17 for 371144
```
