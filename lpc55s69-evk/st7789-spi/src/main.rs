#![no_std]
#![no_main]

extern crate panic_halt;

use lpc55_hal::{
    prelude::*,
    drivers::{pins::Level, SpiMaster},
    typestates::pin::flexcomm::{NoMiso, NoCs},
};

use embedded_graphics::{image::*, prelude::*, pixelcolor::Rgb565};
use st7789::{ST7789, Orientation};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

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
impl DelayUs<u32> for LpcDelay {
    fn delay_us(&mut self, us: u32) {
        cortex_m::asm::delay(10 * us);
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

    let sck = pins.pio1_2.into_spi8_sck_pin(&mut iocon);
    let mosi = pins.pio0_26.into_spi8_mosi_pin(&mut iocon);

    let spi_pins = (sck, mosi, NoMiso, NoCs);
    let spi = SpiMaster::new(spi, spi_pins, 8.mhz(), embedded_hal::spi::MODE_3);

    let mut rst = pins.pio1_9.into_gpio_pin(&mut iocon, &mut gpio).into_output(Level::High);
    let dc = pins.pio1_10.into_gpio_pin(&mut iocon, &mut gpio).into_output_high();
    let mut _cs = pins.pio1_4.into_gpio_pin(&mut iocon, &mut gpio).into_output_low();

    let mut delay = LpcDelay {};
    let di = display_interface_spi::SPIInterfaceNoCS::new(spi, dc);
    let mut display = ST7789::new(di, rst, 240, 240);

    display.init(&mut delay).unwrap();
    display.set_orientation(Orientation::Portrait).unwrap();
    display.clear(Rgb565::BLACK).unwrap();

    let mut rng = SmallRng::seed_from_u64(0);
    let raw_image_data = ImageRawLE::new(include_bytes!("./ferris.raw"), 86, 64);
    let black = Rgb565::BLACK.into_storage();
    let white = Rgb565::WHITE.into_storage();
    let mut img = true;

    loop {
        if img {
            display.clear(Rgb565::BLACK).unwrap();
            let mut xx: u16 = 0;
            let mut yy: u16 = 0;

            loop {
                let ferris = Image::new(&raw_image_data, Point::new(xx as i32, yy as i32));
                ferris.draw(&mut display).unwrap();

                xx += 86;
                if xx + 86 >= 240 {
                    xx = 0;
                    yy += 64;
                }
                if yy + 64 >= 240 {
                    break;
                }
            }
        } else {
            let width: u16 = 240;
            let height: u16 = 240;
            let i_size: u16 = 8; // Number of pixels for each invader
            let interval: u16 = 4;
            let p_size: u16 = 3;

            display.clear(Rgb565::BLACK).unwrap();
            let mut y: u16 = interval;
            while (y + i_size * p_size) < height as u16 {
                let mut x = interval;
                while (x + i_size * p_size) < width as u16 {
                    let mut cy = y;
                    for _j in 0..i_size {
                        let mut bits: u16 = rng.gen();
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
                                        display.set_pixel(cx + j, cy + k, black).unwrap();
                                    } else {
                                        display.set_pixel(cx + j, cy + k, white).unwrap();
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
        }
        delay.delay_ms(500_u32);
        img = !img;
    }
}
