# blinky

Small "blinky" in Rust developed using the SoC specifications.

See parent directory for build and installation instructions.

The 2 LEDs (1 + 1 RGB) on the board should start blinking once project is installed.

## Quick build and flash instructions

```
$ cargo compile --bin blinky
$ cargo objcopy --bin blinky -- -O ihex blinky.hex
$ nrfutil pkg generate --hw-version 52 --sd-req=0x00  --debug-mode \
    --application blinky.hex blinky.zip
$ nrfutil dfu usb-serial -pkg blinky.zip -p /dev/tty.usbmodem*
```
