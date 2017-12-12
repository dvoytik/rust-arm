# Rust ARM low level library [![Crates.io](https://img.shields.io/crates/v/arm.svg)](https://crates.io/crates/arm) [![Crates.io](https://img.shields.io/crates/d/arm.svg)](https://crates.io/crates/arm)

Low level abstraction of ARM architecture specific features.

## Quick start

Note: for now this crate was checked only against ARMv7 targets.

Setup cross toolchains:
```sh
rustup install nightly
rustup default nightly
rustup target add armv7-unknown-linux-gnueabihf

cat >>~/.cargo/config <<EOF
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF
```
You need to install `arm-linux-gnueabihf-gcc` separately for cross-linking.

More details about cross-compiling: https://github.com/japaric/rust-cross

Build:
```sh
cargo build --target=armv7-unknown-linux-gnueabihf
```


## TODO

 * basic SMP for AArch64 and AArch32
 * EL1 registers
 * EL2 registers
 * EL3 registers

## Documentation

```sh
cargo doc --open --target=armv7-unknown-linux-gnueabihf
```

## License

The library is distributed under the terms of [MIT license](LICENSE)
