# st7789-spi

Ths project is based on ST7789 SPI controller. I tested it with this [1.54" Display Module](https://www.buydisplay.com/1-54-inch-tft-ips-lcd-display-module-135x240-spi-for-arduino-raspberry-pi).

## Connections

|    Board    | Screen |
|:-----------:|:------:|
|     GND     |  GND   |
|     VDD     |  VDD   |
| P1.15 (D13) |  SCL   |
| P1.13 (D11) |  SDA   |
| P1.08 (D7)  |  RES   |
| P1.07 (D6)  |  DC    |
| P1.06 (D5)  |  CS    |

See parent directory for build and installation instructions.

After installing the project, the screen should alternate between the Rust mascot and randomly generated space invaders.
