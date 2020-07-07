# ssd1306-spi

This project uses an SPI OLED 0.96" screen using a SSD1306 driver chip. The one I used is very similar to [this one](https://www.buydisplay.com/white-spi-i2c-0-96-inch-oled-display-module-breakout-board-for-arduino). A lot of the work is accomplished by the [ssd1306](https://crates.io/crates/ssd1306) crate. Use the following connections:

## Connections

|    Board    | Screen |
|:-----------:|:------:|
|     GND     |  GND   |
|     VDD     |  VDD   |
|  P1_2 (D13) | D0/SCK |
| P0_26 (D11) | D1/SDA |
|  P1_9 (D7)  |  RES   |
| P1_10 (D6)  |  DC    |
|  P1_4 (D5)  |  CS    |

See parent directory for build and installation instructions.

After installing the project, the screen should alternate between the Rust mascot and randomly generated space invaders.
