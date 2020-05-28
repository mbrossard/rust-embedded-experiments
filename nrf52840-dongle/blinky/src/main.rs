#![no_std]
#![no_main]

extern crate panic_halt;

use nrf52840_hal::{prelude::*, gpio::*, pac::Peripherals};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins0 = p0::Parts::new(p.P0);
    let pins1 = p1::Parts::new(p.P1);

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
        led_1g.set_low().unwrap();
        cortex_m::asm::delay(20000000);
        led_2r.set_low().unwrap();
        cortex_m::asm::delay(20000000);
        led_2g.set_low().unwrap();
        cortex_m::asm::delay(20000000);
        led_2b.set_low().unwrap();
        cortex_m::asm::delay(30000000);

        led_2b.set_high().unwrap();
        cortex_m::asm::delay(20000000);
        led_2g.set_high().unwrap();
        cortex_m::asm::delay(20000000);
        led_2r.set_high().unwrap();
        cortex_m::asm::delay(20000000);
        led_1g.set_high().unwrap();
        cortex_m::asm::delay(30000000);
    }
}
