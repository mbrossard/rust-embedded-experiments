# stm32f411

Rust experiments developed and tested on [WeAct V1.3 STM32F411CEU6](https://github.com/mcauser/WEACT_F411CEU6)
board with [STM32F411CE](https://www.st.com/en/microcontrollers-microprocessors/stm32f411ce.html).

The following projects:
- blinky
- hal-blinky
- ssd1306-i2c

In the rest of the document replace `<project>` with the project name:

## Environment installation instructions

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

- Install [cargo-flash](https://github.com/probe-rs/cargo-flash)
  ```
  $ cargo install cargo-flash
  ```

## Build instructions

- Build the project:
  ```
  $ cargo build --release
  ```

- (Optional) Inspect size of the project:
  ```
  $ cargo size --bin <project> --release
      Finished release [optimized + debuginfo] target(s) in 0.02s
     text	   data	    bss	    dec	    hex	filename
      644	      0	      4	    648	    288	<project>
  ```

- Extract binary file:
  ```
  $ cargo objcopy --bin <project> --release -- -O binary <project>.bin
  ```

## Installation instructions

### Using `cargo-flash`

Using cargo-flash requires connecting a debugger probe to the SWD connector of the board. This procedure was tested with STLink-V2 probe but should work with all probes supported by `cargo-flash`.

- Flash project:
  ```
  $ cargo flash --release --bin <project> --chip STM32F411CE
  ```

### Using pyocd

Using cargo-flash requires connecting a debugger probe to the SWD connector of the board. This procedure was tested with STLink-V2 probe but should work with all probes supported by pyocd.

- Install CMSIS pack for STM32F411CE (only needs to be done once):
  ```
  $ pyocd pack -i stm32f411ce
  ```

- Flash binary file:
  ```
  $ pyocd flash --target stm32f411ce <project>.bin
  ```

### Using `dfu-util`

This step requires the program [dfu-util](http://dfu-util.sourceforge.net/) to be installed. Only a USB cable is required for this method, but works less reliably than the other two methods.

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

- Install binary file:
  ```
  $ dfu-util -a 0 -s 0x08000000:leave -D <project>.bin
  ```
