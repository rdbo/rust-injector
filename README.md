# Rust Injector
Shared library injector for Linux written in Rust

## License
Read the file `LICENSE`

## Usage
Requires root access
```
[ Rust Injector ] by rdbo
====================
usage: ./rust-injector [-n NAME][-f FILENAME][-p PID] SHARED_LIB
```

## Building
The output binary will be located at: `target/release/rust-injector`
```
cargo build --release
```

## Status
- x86_64: Working
- x86_32: Not fully working

## Notes
This is my first Rust project, so don't expect best quality code  
Everything was written from scratch in pure Rust  
Requires the `nix` and `regex` packages

## PoC
Terminal Logs:
```
$ cargo build && sudo ./target/debug/rust-injector -f test ~/Documents/Repos/libtest/libtest.so
[ Rust Injector ] by rdbo
====================
```
...
```
Library handle: 0x55db721be6f0
====================
Injected successfully!
```
Target Process:
```
$ ./test
Waiting...
Waiting...
Waiting...
Injected!
Waiting...
Waiting...
```
