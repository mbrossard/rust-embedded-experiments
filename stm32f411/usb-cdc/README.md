
## Testing Semihosting

To compile, flash and debug with openocd:

```
$ cargo objcopy --bin usb-cdc --release -- -O binary usb-cdc.bin
$ openocd -f stm32f411.cfg -c \
    "init; reset halt; flash write_image erase usb-cdc.bin 0x08000000 bin; arm semihosting enable; reset halt"
```

On a second terminal:

```
$ arm-none-eabi-gdb target/thumbv7m-none-eabi/release/usb-cdc -ex "target remote localhost:3333"
[...]
(gdb) c
Continuing.
```

On a third terminal:

```
$ screen /dev/tty.usbmodemTEST1 115200
```
