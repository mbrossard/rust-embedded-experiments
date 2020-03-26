#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate nrf52840_hal;

use nrf52840_hal::{
    prelude::*,
    spim::*,
    delay::Delay,
    gpio::Level,
    nrf52840_pac::Peripherals,
};
use embedded_graphics::{image::Image1BPP, prelude::*};
use ssd1306::{prelude::*, Builder};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let pins1 = p.P1.split();

    let spiclk = pins1.p1_14.into_push_pull_output(Level::Low).degrade();
    let spimosi = pins1.p1_13.into_push_pull_output(Level::Low).degrade();
    let spimiso = pins1.p1_12.into_floating_input().degrade();
    let _cs = pins1.p1_11.into_push_pull_output(Level::Low).degrade();
    let mut rst = pins1.p1_08.into_push_pull_output(Level::Low).degrade();
    let dc = pins1.p1_07.into_push_pull_output(Level::Low).degrade();

    let spi_pins = nrf52840_hal::spim::Pins { sck: spiclk, miso: Some(spimiso), mosi: Some(spimosi) };
    let spi = Spim::new(p.SPIM0, spi_pins, Frequency::K500, MODE_0, 0);

    let mut delay = Delay::new(cp.SYST);
    let mut display: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();
    display.reset(&mut rst, &mut delay).unwrap();
    display.init().unwrap();

    let mut rng = SmallRng::seed_from_u64(0);
    let im = Image1BPP::new(include_bytes!("./rust-image.data"), 128, 64);

    let mut img = true;
    loop {
        if img {
            display.draw(im.into_iter());
            display.flush().unwrap();
        } else {
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
        cortex_m::asm::delay(50000000);
        img = !img;
    }
}
