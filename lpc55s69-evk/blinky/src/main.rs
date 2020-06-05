#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;

use lpc55_hal::{drivers::pins::Level, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    let hal = lpc55_hal::new();

    let mut syscon = hal.syscon;
    let mut gpio = hal.gpio.enabled(&mut syscon);
    let mut iocon = hal.iocon.enabled(&mut syscon);

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

    // Blue is pio1_4
    let mut blue = pins
        .pio1_4
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output(Level::High);

    loop {
        red.set_low().unwrap();
        cortex_m::asm::delay(50000000);

        red.set_high().unwrap();
        blue.set_low().unwrap();
        cortex_m::asm::delay(50000000);

        blue.set_high().unwrap();
        green.set_low().unwrap();
        cortex_m::asm::delay(50000000);

        red.set_low().unwrap();
        blue.set_low().unwrap();
        cortex_m::asm::delay(50000000);

        red.set_high().unwrap();
        blue.set_high().unwrap();
        green.set_high().unwrap();
        cortex_m::asm::delay(50000000);
    }
}
