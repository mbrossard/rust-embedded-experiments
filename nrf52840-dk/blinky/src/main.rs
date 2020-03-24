#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate nrf52840_hal;

use nrf52840_hal::{
    prelude::*,
    gpio::Level,
    nrf52840_pac::Peripherals,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins0 = p.P0.split();

    let mut led_1 = pins0.p0_13.into_push_pull_output(Level::High);
    let mut led_2 = pins0.p0_14.into_push_pull_output(Level::High);
    let mut led_3 = pins0.p0_15.into_push_pull_output(Level::High);
    let mut led_4 = pins0.p0_16.into_push_pull_output(Level::High);

    // let mut timer = p.TIMER0.constrain();

    loop {
        led_1.set_low();
        cortex_m::asm::delay(20000000);
        led_2.set_low();
        cortex_m::asm::delay(20000000);
        led_4.set_low();
        cortex_m::asm::delay(20000000);
        led_3.set_low();
        cortex_m::asm::delay(30000000);

        led_3.set_high();
        cortex_m::asm::delay(20000000);
        led_4.set_high();
        cortex_m::asm::delay(20000000);
        led_2.set_high();
        cortex_m::asm::delay(20000000);
        led_1.set_high();
        cortex_m::asm::delay(30000000);
    }
}
