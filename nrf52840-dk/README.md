# nrf52840

Rust experiments developed and tested on [nRF52840-DK](https://www.nordicsemi.com/Software-and-Tools/Development-Kits/nRF52840-DK)
The following projects:
- blinky

In the rest of the document replace `<project>` with the project name:

## Environment installation instructions

- Install Rust (see [Rust installation instructions](https://www.rust-lang.org/tools/install))

- Add Rust cross-compiler for ARMv7-M:
  ```
  $ rustup target add thumbv7em-none-eabihf
  ```

- Add [cargo-binutils](https://github.com/rust-embedded/cargo-binutils):
  ```
  $ cargo install cargo-binutils
  $ rustup component add llvm-tools-preview
  ```

- Install [cargo-flash](https://github.com/probe-rs/cargo-flash)
  ```
  $ cargo install cargo-flash
  ```

## Build instructions

- Build the project:
  ```
  $ cargo build --release
  ```

## Installation instructions

### Using `cargo-flash`

- Flash project:
  ```
  $ cargo flash --bin blinky --release --chip nRF52840_xxAA
  ```

### Using pyocd

- Install CMSIS pack for nRF52840_xxAA (only needs to be done once):
  ```
  $ pyocd pack -i nrf52840
  ```

- Flash binary file:
  ```
  $ pyocd flash -b nRF52840_xxAA --format elf target/thumbv7em-none-eabihf/release/<project>
  ```
