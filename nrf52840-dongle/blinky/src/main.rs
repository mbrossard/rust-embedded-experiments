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
    let pins1 = p.P1.split();

    // LD1  Green   P0.06
    // LD2  Red     P0.08
    // LD2  Green   P1.09
    // LD2  Blue    P0.12

    let mut led_1g = pins0.p0_06.into_push_pull_output(Level::High);
    let mut led_2r = pins0.p0_08.into_push_pull_output(Level::High);
    let mut led_2g = pins1.p1_09.into_push_pull_output(Level::High);
    let mut led_2b = pins0.p0_12.into_push_pull_output(Level::High);

    // let mut timer = p.TIMER0.constrain();

    loop {
        led_1g.set_low();
        cortex_m::asm::delay(20000000);
        led_2r.set_low();
        cortex_m::asm::delay(20000000);
        led_2g.set_low();
        cortex_m::asm::delay(20000000);
        led_2b.set_low();
        cortex_m::asm::delay(30000000);

        led_2b.set_high();
        cortex_m::asm::delay(20000000);
        led_2g.set_high();
        cortex_m::asm::delay(20000000);
        led_2r.set_high();
        cortex_m::asm::delay(20000000);
        led_1g.set_high();
        cortex_m::asm::delay(30000000);
    }
}
