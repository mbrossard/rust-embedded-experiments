#![no_std]
#![no_main]

extern crate panic_halt;
extern crate xiao_m0 as hal;

use hal::hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = hal::pac::Peripherals::take().unwrap();
    let pins = hal::Pins::new(peripherals.PORT);
    let mut led0 = pins.led0.into_push_pull_output();
    let mut led1 = pins.led1.into_push_pull_output();
    let mut led2 = pins.led2.into_push_pull_output();

    led0.set_high().unwrap();
    led1.set_high().unwrap();
    led2.set_high().unwrap();

    loop {
        led0.set_low().unwrap();
        cortex_m::asm::delay(500000);
        led0.set_high().unwrap();
        led1.set_low().unwrap();
        cortex_m::asm::delay(500000);
        led1.set_high().unwrap();
        led2.set_low().unwrap();
        cortex_m::asm::delay(500000);
        led2.set_high().unwrap();
    }
}
