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

    let im = Image1BPP::new(include_bytes!("./rust-image.data"), 128, 64);
    display.draw(im.into_iter());
    display.flush().unwrap();

    loop {}
}
