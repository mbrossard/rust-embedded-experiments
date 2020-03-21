# hal-blinky

Small "blinky" in Rust developed using the [embedded-hal](https://docs.rs/embedded-hal) absraction.

See parent directory for build and installation instructions.

The Blue LED on the boqrd should start blinking once project is installed.

## `core_delay` feature

Adding `--features core_delay` to the `cargo` instructions disable the use of a much more precise delay function based on the system clock. The benefit is a project size very close to the version of this project not using `embedded-hal`.

- With `core_delay`:
  ```
  % cargo size --bin hal-blinky --release --features core_delay
    text	   data	    bss	    dec	    hex	filename
    664	      0	      4	    668	    29c	hal-blinky
  ```
- Without:
  ```
  % cargo size --bin hal-blinky --release
    text	   data	    bss	    dec	    hex	filename
    2016	      0	      4	   2020	    7e4	hal-blinky
  ```
