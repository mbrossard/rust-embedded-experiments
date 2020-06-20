#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate feather_m4;

use feather_m4::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = feather_m4::pac::Peripherals::take().unwrap();
    let mut pins = feather_m4::Pins::new(peripherals.PORT);
    let mut led = pins.d13.into_open_drain_output(&mut pins.port);

    loop {
        led.set_high().unwrap();
        cortex_m::asm::delay(5000000);
        led.set_low().unwrap();
        cortex_m::asm::delay(5000000);
    }
}
