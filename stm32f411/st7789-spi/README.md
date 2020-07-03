# st7789-spi

Ths project is based on ST7789 SPI controller. I tested it with this [1.54" Display Module](https://www.buydisplay.com/1-54-inch-tft-ips-lcd-display-module-135x240-spi-for-arduino-raspberry-pi).

## Connections

### SPI1

|  Board |   Screen    |
|:------:|:-----------:|
|  GND   |  GND        |
|  VDD   |  VDD        |
|  PA7   |  SDA (MOSI) |
|  PA5   |  SCL (SCK)  |
|  PA4   |  RES        |
|  PA3   |  DC         |
|  PA2   |  CS         |

### SPI2

|  Board |   Screen    |
|:------:|:-----------:|
|  GND   |  GND        |
|  VDD   |  VDD        |
|  PB13  |  SCL (SCK)  |
|  PB15  |  SDA (MOSI) |
|  PA8   |  RES        |
|  PA9   |  DC         |
|  PA10  |  CS         |


See parent directory for build and installation instructions.

After installing the project, the screen should alternate between the Rust mascot and randomly generated space invaders.
