# wio-terminal

Rust experiments developed and so far untested on [Feather M4 Express](https://wiki.seeedstudio.com/Wio-Terminal-Getting-Started/) board.

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

## Flashing with Bossa

- Add [cargo-binutils](https://github.com/rust-embedded/cargo-binutils):
  ```
  $ cargo install cargo-binutils
  $ rustup component add llvm-tools-preview
  ```
- Install [bossa](https://github.com/shumatech/BOSSA)

- Compile to binary file:
  ```
  $ cargo objcopy --bin <project> --release -- -O binary <project>.bin
  ```

- Flash image (board needs to be in bootloader mode, replace `<serial>` below with serial port)
   ```
   $ bossac -i -d --port=<serial> -U -e -w -o 0x4000 -v <project>.bin -R
   ```
   