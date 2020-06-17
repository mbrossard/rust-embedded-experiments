# lpc55s69-evk

Rust experiments developed and tested on [LPC55S69-EVK](https://www.nxp.com/products/processors-and-microcontrollers/arm-microcontrollers/general-purpose-mcus/lpc5500-cortex-m33/lpcxpresso55s69-development-board:LPC55S69-EVK) board.
The following projects:
- blinky
- ssd1306-spi
- st7789-spi

In the rest of the document replace `<project>` with the project name:

## Environment installation instructions

- Install Rust (see [Rust installation instructions](https://www.rust-lang.org/tools/install))

- Add Rust cross-compiler for ARMv8-M Mainline:
  ```
  $ rustup target add thumbv8m.main-none-eabi
  ```

- Add [cargo-binutils](https://github.com/rust-embedded/cargo-binutils):
  ```
  $ cargo install cargo-binutils
  $ rustup component add llvm-tools-preview
  ```

## Build instructions

- Build the project:
  ```
  $ cargo build --release
  ```

## Installation instructions

### Using pyocd

- Install CMSIS pack for LPC55S69JBD100 (only needs to be done once):
  ```
  $ pyocd pack -i lpc55s69
  ```

- Flash binary file:
  ```
  $ pyocd flash -b LPC55S69JBD100 --format elf target/thumbv8m.main-none-eabi/release/<project>
  ```
