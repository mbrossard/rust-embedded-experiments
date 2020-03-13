#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
// extern crate stm32f4xx_hal;

use embedded_graphics::{image::Image1BPP, prelude::*};
use ssd1306::{prelude::*, Builder as SSD1306Builder};
use stm32f4xx_hal::{i2c::I2c, prelude::*, stm32};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

#[cortex_m_rt::entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let mut rng = SmallRng::seed_from_u64(0);

        // Set up I2C - SCL is PB6 and SDA is PB7; they are set to Alternate Function 4
        // as per the STM32F411xC/E datasheet page 48.
        let gpiob = dp.GPIOB.split();
        let scl = gpiob.pb6.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb7.into_alternate_af4().set_open_drain();
        let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);

        // Set up the display
        let mut disp: GraphicsMode<_> = SSD1306Builder::new().connect_i2c(i2c).into();
        disp.init().unwrap();
        disp.flush().unwrap();

        // Display the rustacean
        let im = Image1BPP::new(include_bytes!("./rust-image.data"), 128, 64);

        let mut img = true;
        loop {
            if img {
                disp.draw(im.into_iter());
                disp.flush().unwrap();
            } else {
                let width = 128;
                let height = 64;
                let i_size: u32 = 8; // Number of pixels for each invader
                let interval: u32 = 4;
                let p_size: u32 = 3;

                disp.clear();
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
                                            disp.set_pixel(cx + j, cy + k, 0);
                                        } else {
                                            disp.set_pixel(cx + j, cy + k, 1);
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
                disp.flush().unwrap();
            }
            cortex_m::asm::delay(50000000);
            img = !img;
        }
    }

    loop {}
}
