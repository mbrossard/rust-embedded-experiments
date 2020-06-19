#![no_std]
#![no_main]

extern crate panic_halt;
// extern crate panic_semihosting;

use embedded_graphics::{image::*, pixelcolor::Rgb565, prelude::*};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use st7789::{Orientation, ST7789};
use stm32f4xx_hal::{delay::Delay, prelude::*, spi::*, stm32, time::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    let gpioa = dp.GPIOA.split();
    #[cfg(feature = "spi2")]
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    // Set up the LED connected to pin PC13.
    let mut led = gpioc.pc13.into_push_pull_output();

    // let mode0 = Mode { polarity: Polarity::IdleLow, phase: Phase::CaptureOnFirstTransition };
    let mode3 = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };
    // let freq500k: Hertz = KiloHertz(500).into();
    let freq8m: Hertz = MegaHertz(8).into();

    #[cfg(feature = "spi1")]
    let mut display = {
        let sck = gpioa.pa5.into_alternate_af5();
        let miso = gpioa.pa6.into_alternate_af5();
        let mosi = gpioa.pa7.into_alternate_af5();
        let spi = Spi::spi1(dp.SPI1, (sck, miso, mosi), mode3, freq8m, clocks);
        let rst = gpioa.pa4.into_push_pull_output();
        let dc = gpioa.pa3.into_push_pull_output();
        let cs = gpioa.pa2.into_push_pull_output();

        let di = display_interface_spi::SPIInterface::new(spi, dc, cs);
        ST7789::new(di, rst, 240, 240)
    };
    #[cfg(feature = "spi2")]
    let mut display = {
        let sck = gpiob.pb13.into_alternate_af5();
        let miso = gpiob.pb14.into_alternate_af5();
        let mosi = gpiob.pb15.into_alternate_af5();
        let spi = Spi::spi2(dp.SPI2, (sck, miso, mosi), mode3, freq8m, clocks);
        let rst = gpioa.pa8.into_push_pull_output();
        let dc = gpioa.pa9.into_push_pull_output();
        let cs = gpioa.pa10.into_push_pull_output();

        let di = display_interface_spi::SPIInterface::new(spi, dc, cs);
        ST7789::new(di, rst, 240, 240)
    };

    let mut delay = Delay::new(cp.SYST, clocks);
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
            led.set_low().unwrap();
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
            led.set_high().unwrap();
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
