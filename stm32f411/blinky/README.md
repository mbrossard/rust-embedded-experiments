# blinky

Small "blinky" in Rust developed using the SoC specifications.

The resulting firmware has a size of 644 bytes:

```
$ cargo size --bin blinky --release
...
   text	   data	    bss	    dec	    hex	filename
    644	      0	      4	    648	    288	blinky
```

See parent directory for build and installation instructions.

The Blue LED on the board should start blinking once project is installed.

