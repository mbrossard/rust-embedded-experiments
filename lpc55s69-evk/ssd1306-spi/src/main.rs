#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate lpc55_hal;
extern crate panic_halt;

use lpc55_hal::{
    drivers::{pins::Level, SpiMaster},
    prelude::*,
    typestates::pin::flexcomm::{NoCs, NoMiso},
};

use embedded_graphics::{image::Image, image::ImageRaw, pixelcolor::BinaryColor, prelude::*};
use embedded_hal::blocking::delay::DelayMs;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use ssd1306::{self, prelude::*};

struct LpcDelay {}

impl DelayMs<u8> for LpcDelay {
    fn delay_ms(&mut self, ms: u8) {
        cortex_m::asm::delay(10000 * (ms as u32));
    }
}
impl DelayMs<u32> for LpcDelay {
    fn delay_ms(&mut self, ms: u32) {
        cortex_m::asm::delay(10000 * ms);
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let hal = lpc55_hal::new();

    let mut syscon = hal.syscon;
    let mut gpio = hal.gpio.enabled(&mut syscon);
    let mut iocon = hal.iocon.enabled(&mut syscon);

    let mut anactrl = hal.anactrl;
    let mut pmc = hal.pmc;

    let clocks = lpc55_hal::ClockRequirements::default()
        .configure(&mut anactrl, &mut pmc, &mut syscon)
        .unwrap();

    let token = clocks.support_flexcomm_token().unwrap();

    // SPI8 is the high-speed SPI
    let spi = hal.flexcomm.8.enabled_as_spi(&mut syscon, &token);
    let pins = lpc55_hal::Pins::take().unwrap();

    // Note: for the LEDs, high is off and low is on.

    // Red is pio1_6
    let mut red = pins
        .pio1_6
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output(Level::High);

    // Green is pio1_7
    let mut green = pins
        .pio1_7
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output(Level::High);

    let sck = pins.pio1_2.into_spi8_sck_pin(&mut iocon);
    let mosi = pins.pio0_26.into_spi8_mosi_pin(&mut iocon);
    let spi_pins = (sck, mosi, NoMiso, NoCs);
    let spi = SpiMaster::new(spi, spi_pins, 8.mhz(), embedded_hal::spi::MODE_3);
    let mut rst = pins
        .pio1_9
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output(Level::High);
    let dc = pins
        .pio1_10
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output_high();
    let mut _cs = pins
        .pio1_4
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output_low();

    let mut delay = LpcDelay {};
    let mut display: GraphicsMode<_> = ssd1306::Builder::new()
        .size(DisplaySize::Display128x64)
        .connect_spi(spi, dc)
        .into();
    display.reset(&mut rst, &mut delay).unwrap();
    display.init().unwrap();

    let mut rng = SmallRng::seed_from_u64(0);
    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust-image.data"), 128, 64);
    let im = Image::new(&raw, Point::new(0, 0));

    let mut img = true;
    loop {
        if img {
            green.set_high().unwrap();
            red.set_low().unwrap();
            display.draw_image(&im).ok();
            display.flush().unwrap();
        } else {
            red.set_high().unwrap();
            green.set_low().unwrap();
            let width = 128;
            let height = 64;
            let i_size: u32 = 8; // Number of pixels for each invader
            let interval: u32 = 4;
            let p_size: u32 = 3;

            display.clear();
            let mut y = interval;
            while (y + i_size * p_size) < height as u32 {
                let mut x = interval;
                while (x + i_size * p_size) < width as u32 {
                    let mut cy = y;
                    for _j in 0..i_size {
                        let mut bits: u32 = rng.gen();
                        for i in 0..i_size / 2 {
                            let h = 1 << (i_size - i - 1);
                            let l = 1 << i;
                            if bits & l != 0 {
                                bits |= h;
                            } else {
                                bits &= !h;
                            }
                        }

                        let mut cx = x;
                        for _i in 0..i_size {
                            for k in 0..p_size {
                                for j in 0..p_size {
                                    if bits & 0x1 != 0 {
                                        display.set_pixel(cx + j, cy + k, 0);
                                    } else {
                                        display.set_pixel(cx + j, cy + k, 1);
                                    }
                                }
                            }
                            bits >>= 1;
                            cx += p_size;
                        }
                        cy += p_size;
                    }
                    x += i_size * p_size + interval;
                }
                y += i_size * p_size + interval;
            }
            display.flush().unwrap();
        }
        delay.delay_ms(500_u32);
        img = !img;
    }
}
