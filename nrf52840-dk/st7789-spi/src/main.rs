#![no_std]
#![no_main]

extern crate panic_halt;

use embedded_graphics::{image::*, pixelcolor::Rgb565, prelude::*};
use embedded_hal::blocking::delay::DelayMs;
use nrf52840_hal::{delay::Delay, gpio::Level, pac::Peripherals, spim::*};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use st7789::{Orientation, ST7789};

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let pins1 = nrf52840_hal::gpio::p1::Parts::new(p.P1);

    let spiclk = pins1.p1_15.into_push_pull_output(Level::Low).degrade();
    let spimosi = pins1.p1_13.into_push_pull_output(Level::Low).degrade();
    let rst = pins1.p1_08.into_push_pull_output(Level::Low).degrade();
    let dc = pins1.p1_07.into_push_pull_output(Level::Low).degrade();
    let _cs = pins1.p1_06.into_push_pull_output(Level::Low).degrade();

    let spi_pins = nrf52840_hal::spim::Pins {
        sck: spiclk,
        miso: None,
        mosi: Some(spimosi),
    };
    let spi = Spim::new(p.SPIM0, spi_pins, Frequency::M8, MODE_3, 0);

    let mut delay = Delay::new(cp.SYST);
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
