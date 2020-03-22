# invaders (nightly)

This version uses the [stm32f7-discovery](https://github.com/embed-rs/stm32f7-discovery.git) crate that requires a non-stable version of the Rust toolchain.

## Build instructions

- Install Rust (see [Rust installation instructions](https://www.rust-lang.org/tools/install))

- Add Rust cross-compiler for ARMv7-M (with floating point):
  ```
  $ rustup target add thumbv7em-none-eabihf
  ```

- Add [cargo-binutils](https://github.com/rust-embedded/cargo-binutils):
  ```
  $ cargo install cargo-binutils
  $ rustup component add llvm-tools-preview
  ```

- Build (the result is in `target/thumbv7em-none-eabihf/release/stm32f746g-disc-invaders` which is an ELF file with debug symbols):
  ```
  $ cargo build --release
  ```

- Extract binary file (`invaders.bin`):
  ```
  $ cargo objcopy --bin stm32f746g-disc-invaders --release -- -O binary invaders.bin
  ```

## Installation instructions

Copy `invaders.bin` to STM32F746-NG virtual drive.
