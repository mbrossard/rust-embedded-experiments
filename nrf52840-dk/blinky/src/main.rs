#![no_std]
#![no_main]

extern crate panic_halt;

use nrf52840_hal::{gpio::Level, pac::Peripherals, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins0 = nrf52840_hal::gpio::p0::Parts::new(p.P0);

    let mut led_1 = pins0.p0_13.into_push_pull_output(Level::High);
    let mut led_2 = pins0.p0_14.into_push_pull_output(Level::High);
    let mut led_3 = pins0.p0_15.into_push_pull_output(Level::High);
    let mut led_4 = pins0.p0_16.into_push_pull_output(Level::High);

    // let mut timer = p.TIMER0.constrain();

    loop {
        led_1.set_low().unwrap();
        cortex_m::asm::delay(20000000);
        led_2.set_low().unwrap();
        cortex_m::asm::delay(20000000);
        led_4.set_low().unwrap();
        cortex_m::asm::delay(20000000);
        led_3.set_low().unwrap();
        cortex_m::asm::delay(30000000);

        led_3.set_high().unwrap();
        cortex_m::asm::delay(20000000);
        led_4.set_high().unwrap();
        cortex_m::asm::delay(20000000);
        led_2.set_high().unwrap();
        cortex_m::asm::delay(20000000);
        led_1.set_high().unwrap();
        cortex_m::asm::delay(30000000);
    }
}
