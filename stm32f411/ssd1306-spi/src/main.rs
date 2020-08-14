#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;

use embedded_graphics::{image::Image, image::ImageRaw, pixelcolor::BinaryColor, prelude::*};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use ssd1306::{prelude::*, Builder};
use stm32f4xx_hal::{delay::Delay, prelude::*, spi::*, stm32, time::Hertz};

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
    let mut delay = Delay::new(cp.SYST, clocks);

    let gpioa = dp.GPIOA.split();
    #[cfg(feature = "spi2")]
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    // Set up the LED connected to pin PC13.
    let mut led = gpioc.pc13.into_push_pull_output();

    // let mode0 = Mode { polarity: Polarity::IdleLow, phase: Phase::CaptureOnFirstTransition };
    let mode3 = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };
    // let freq500k: Hertz = stm32f4xx_hal::time::KiloHertz(500).into();
    let freq8m: Hertz = stm32f4xx_hal::time::MegaHertz(8).into();

    #[cfg(feature = "spi1")]
    let mut display: GraphicsMode<_> = {
        let sck = gpioa.pa5.into_alternate_af5();
        let miso = gpioa.pa6.into_alternate_af5();
        let mosi = gpioa.pa7.into_alternate_af5();
        let spi = Spi::spi1(dp.SPI1, (sck, miso, mosi), mode3, freq8m, clocks);
        let dc = gpioa.pa3.into_push_pull_output();
        let mut cs = gpioa.pa2.into_push_pull_output();
        cs.set_low().unwrap();

        let interface = display_interface_spi::SPIInterfaceNoCS::new(spi, dc);
        Builder::new()
            .size(DisplaySize128x64)
            .connect(interface)
            .into()
    };
    #[cfg(feature = "spi1")]
    let mut rst = gpioa.pa4.into_push_pull_output();

    #[cfg(feature = "spi2")]
    let mut display: GraphicsMode<_> = {
        let sck = gpiob.pb13.into_alternate_af5();
        let miso = gpiob.pb14.into_alternate_af5();
        let mosi = gpiob.pb15.into_alternate_af5();
        let spi = Spi::spi2(dp.SPI2, (sck, miso, mosi), mode3, freq8m, clocks);
        let dc = gpioa.pa9.into_push_pull_output();
        let mut cs = gpioa.pa10.into_push_pull_output();
        cs.set_low().unwrap();

        let interface = display_interface_spi::SPIInterfaceNoCS::new(spi, dc);
        Builder::new()
            .size(DisplaySize128x64)
            .connect(interface)
            .into()
    };
    #[cfg(feature = "spi2")]
    let mut rst = gpioa.pa8.into_push_pull_output();

    display.reset(&mut rst, &mut delay).unwrap();
    display.init().unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust-image.data"), 128, 64);
    let im = Image::new(&raw, Point::new(0, 0));

    let mut img = true;
    loop {
        if img {
            led.set_low().ok();
            display.draw_image(&im).ok();
            display.flush().unwrap();
        } else {
            led.set_high().ok();
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
