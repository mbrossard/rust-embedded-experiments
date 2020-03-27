# ssd1306-spi

This project uses an SPI OLED 0.96" screen using a SSD1306 driver chip. The one I used is very similar to [this one](https://www.buydisplay.com/white-spi-i2c-0-96-inch-oled-display-module-breakout-board-for-arduino). A lot of the work is accomplished by the [ssd1306](https://crates.io/crates/ssd1306) crate. Use the following connections:

|  Board | Screen |
|:------:|:------:|
|  GND   |  GND   |
|  VDD   |  VDD   |
| P1.14  | D0/SCK |
| P1.13  | D1/SDA |
| P1.08  |  RES   |
| P1.07  |   DC   |

See parent directory for build and installation instructions.

After installing the project, the screen should alternate between the Rust mascot and randomly generated space invaders.
