#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32f4xx_hal;

use stm32f4xx_hal as hal;
use hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dev_peripherals = hal::stm32::Peripherals::take().unwrap();
    let core_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the LED connected to pin PC13.
    let gpioc = dev_peripherals.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    // Set up the system clock at 100MHz.
    let rcc = dev_peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(100.mhz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(core_peripherals.SYST, clocks);

    loop {
        // On for 1s, off for 1s.
        led.set_high().unwrap();
        delay.delay_ms(1000_u32);
        led.set_low().unwrap();
        delay.delay_ms(1000_u32);
    }
}
