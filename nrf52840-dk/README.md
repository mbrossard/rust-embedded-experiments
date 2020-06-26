# nrf52840-dk

Rust experiments developed and tested on [nRF52840-DK](https://www.nordicsemi.com/Software-and-Tools/Development-Kits/nRF52840-DK)

The following projects:
- blinky
- bme280-i2c
- ssd1306-spi
- st7789-spi


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

### Using DAPLink

The DAPLink firmware is the best option for use with pyocd and cargo-flash. 


- The first step is [Updating the DAPLink bootloader](https://os.mbed.com/blog/entry/DAPLink-bootloader-update/) which
  requires connecting the device with the reset button pushed and downloading the new
  [bootloader binary](https://os.mbed.com/media/uploads/c1728p9/0244_sam3u2c_bootloader_update_0x5000.bin). On Mac OS X:
  ```
  $ sudo mount -u -w -o sync /Volumes/BOOTLOADER
  $ cp -X  ~/Downloads/0244_sam3u2c_bootloader_update_0x5000.bin /Volumes/BOOTLOADER
  ```

- The second step is [Installing DAPLink firmware](https://armmbed.github.io/DAPLink/?board=Nordic-nRF52840-DK). I am using
  [this version](https://armmbed.github.io/DAPLink//firmware/0253_sam3u2c_mkit_dk_dongle_nrf5x_0x5000.bin) with success using
  the commands (on Mac OS X):
  ```
  $ sudo mount -u -w -o sync /Volumes/MAINTENANCE
  $ cp -X  ~/Downloads/0253_sam3u2c_mkit_dk_dongle_nrf5x_0x5000.bin /Volumes/MAINTENANCE
  ```

