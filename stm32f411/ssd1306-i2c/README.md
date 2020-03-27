# ssd1306-i2c

This project uses an I2C OLED 0.96" screen using a SSD1306 driver chip. The one I used is very similar to [this one](https://www.buydisplay.com/i2c-white-0-96-inch-oled-display-module-128x64-arduino-raspberry-pi). A lot of the work is accomplished by the [ssd1306](https://crates.io/crates/ssd1306) crate. Use the following connections:

|  Board | Screen |
|:------:|:------:|
|  G     |  GND   |
|  3.3   |  VDD   |
|  B6    |  SCL   |
|  B7    |  SDA   |

See parent directory for build and installation instructions.

After installing the project, the screen should alternate between the Rust mascot and randomly generated space invaders.
