# stm32f411-hal-blinky

Small "blinky" in Rust for [STM32F411CE](https://www.st.com/en/microcontrollers-microprocessors/stm32f411ce.html) board
like [WeAct V1.3 STM32F411CEU6](https://github.com/mcauser/WEACT_F411CEU6).

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

- Build (the result is in `target/thumbv7m-none-eabi/release/stm32f411-hal-blinky` which is an ELF file with debug symbols):
  ```
  $ cargo build --release
  ```

  The actual code size is larger than stm32f411-blinky:
  ```
  $ cargo size --bin stm32f411-hal-blinky --release
      Finished release [optimized + debuginfo] target(s) in 2.53s
     text	   data	    bss	    dec	    hex	filename
     2104	      0	      4	   2108	    83c	stm32f411-hal-blinky
  ```

- Extract binary file (`hal-blinky.bin`):
  ```
  $ cargo objcopy --bin stm32f411-hal-blinky --release -- -O binary hal-blinky.bin
  ```

## Installation instructions

### Using `cargo-flash`

- Install [cargo-flash](https://github.com/probe-rs/cargo-flash)
  ```
  $ cargo install cargo-flash
  ```

- Flash image
  ```
  $ cargo flash --release --bin stm32f411-hal-blinky --chip STM32F411CE
  ```

### Using DFU

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
  dfu-util 0.9
  ...

  Found DFU: [0483:df11] ver=2200, devnum=29, cfg=1, intf=0, path="20-1", alt=3, name="@Device Feature/0xFFFF0000/01*004 e", serial="378B34663038"
  Found DFU: [0483:df11] ver=2200, devnum=29, cfg=1, intf=0, path="20-1", alt=2, name="@OTP Memory /0x1FFF7800/01*512 e,01*016 e", serial="378B34663038"
  Found DFU: [0483:df11] ver=2200, devnum=29, cfg=1, intf=0, path="20-1", alt=1, name="@Option Bytes  /0x1FFFC000/01*016 e", serial="378B34663038"
  Found DFU: [0483:df11] ver=2200, devnum=29, cfg=1, intf=0, path="20-1", alt=0, name="@Internal Flash  /0x08000000/04*016Kg,01*064Kg,03*128Kg", serial="378B34663038"
  ```

- Install binary:
  ```
  $ dfu-util -a 0 -s 0x08000000:leave -D hal-blinky.bin
  ...
  Opening DFU capable USB device...
  ID 0483:df11
  Run-time device DFU version 011a
  ...
  Device returned transfer size 2048
  DfuSe interface name: "Internal Flash  "
  Downloading to address = 0x08000000, size = 2116
  Download	[=========================] 100%         2116 bytes
  Download done.
  File downloaded successfully
  Transitioning to dfuMANIFEST state
  ```

### Using pyocd

Tested with STLink-V2 probe but should work with all probes supported by pyocd.

- Install CMSIS pack for STM32F411CE (only needs to be done once):
  ```
  $ pyocd pack -i stm32f411ce
  ```

- Flash binary:
  ```
  $ pyocd flash --target stm32f411ce hal-blinky.bin
  ```

### After installation

The Blue LED should start blinking.
