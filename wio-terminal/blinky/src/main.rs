#![no_std]
#![no_main]

extern crate panic_halt;
extern crate atsamd_hal as hal;

use hal::prelude::*;
use hal::*;

use gpio::{Floating, Input, Port};

define_pins!(
    struct Pins,
    target_device: target_device,

    pin led = a15,
);

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = atsamd_hal::target_device::Peripherals::take().unwrap();
    let mut pins = Pins::new(peripherals.PORT);
    let mut led = pins.led.into_open_drain_output(&mut pins.port);

    loop {
         led.set_high().unwrap();
         cortex_m::asm::delay(5000000);
         led.set_low().unwrap();
         cortex_m::asm::delay(5000000);
    }
}
