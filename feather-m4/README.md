# feather-m4

Rust experiments developed and so far unsuccessfully tested on [Feather M4 Express](https://learn.adafruit.com/adafruit-feather-m4-express-atsamd51) board.

The following projects:
- blinky

In the rest of the document replace `<project>` with the project name:

## Flashing with UF2 Bootloader

Install `cargo-hf2` (only need to run once)

```
$ cargo install cargo-hf2
```
Build and flash (device need to be in bootloader mode)

```
$ cargo hf2 --release --bin <project>
```
