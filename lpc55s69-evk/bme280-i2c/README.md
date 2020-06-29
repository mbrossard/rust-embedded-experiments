# bme280-i2c

This project uses the
[Bosch BME280](https://www.bosch-sensortec.com/products/environmental-sensors/humidity-sensors-bme280/) sensor through
its I2C interface with the help of the [bme280](https://crates.io/crates/bme280) crate.

|  Board | BME280 |
|:------:|:------:|
|  GND   |  GND   |
|  3.3V  |  VDD   |
|  D15   |  SCL   |
|  D14   |  SDA   |

See parent directory for build and installation instructions.

Tested with [Olimex MOD-BME280](https://www.olimex.com/Products/Modules/Sensors/MOD-BME280/open-source-hardware) breakout.

## Testing

This project uses semihosting output. One way to test it is to use `pyocd` and `gdb`.
In one terminal start `pyocd`:

```
$ pyocd gdb -S -O semihost_console_type=std
```

In another terminal start `gdb` and send commands `mon reset` to reset the board and
`c` to resume execution:

```
$ arm-none-eabi-gdb -ex "set confirm off" \
    -ex "add-symbol-file target/thumbv8m.main-none-eabi/release/bme280-i2c" \
    -ex "target remote localhost:3333"
GNU gdb (GNU Tools for Arm Embedded Processors 9-2019-q4-major) 8.3.0.20190709-git
[...]
(gdb) mon reset
(gdb) c
```
