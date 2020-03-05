# stm32f411-blinky

## Build instructions

- Install Rust (see [Rust installation instructions](https://www.rust-lang.org/tools/install))

- Add Rust cross-compiler for ARMv7-M:
  ```
  $ rustup target add thumbv7m-none-eabi
  ```

- Add [cargo-binutils](https://github.com/rust-embedded/cargo-binutils):
  ```
  $ cargo install cargo-binutils
  $ rustup component add llvm-tools-preview
  ```

- Build (the result is in `target/thumbv7m-none-eabi/release/stm32f411-blinky` which is an ELF file with debug symbols):
  ```
  $ cargo build --release
  ```

- Extract binary file (`blinky.bin`):
  ```
  $ cargo objcopy --bin stm32f411-blinky --release -- -O binary blinky.bin
  ```

## Installation instructions

This step requires the program [dfu-util](http://dfu-util.sourceforge.net/) to be installed.

- Put the device in DFU mode:
  - Connect device to computer.
  - Press and hold `BOOT0`.
  - Press and release `RESET`.
  - Wait 0.5 seconds.
  - Release `BOOT0`.

- Check that the device is accessible:
  ```
  $ dfu-util -l
  ```

- Install binary:
  ```
  $ dfu-util -a 0 -s 0x08000000:leave -D blinky.bin
  ```

The Green LED should start blinking.