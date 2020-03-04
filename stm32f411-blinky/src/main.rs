#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32f4;

use stm32f4::stm32f411;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = stm32f411::Peripherals::take().unwrap();
    let gpioc = &peripherals.GPIOC;
    let rcc = &peripherals.RCC;

    rcc.ahb1enr.write(|w| w.gpiocen().set_bit());
    gpioc.moder.write(|w| w.moder13().bits(0b01));

    loop {
        gpioc.bsrr.write(|w| w.bs13().set_bit());
        cortex_m::asm::delay(5000000);
        gpioc.bsrr.write(|w| w.br13().set_bit());
        cortex_m::asm::delay(5000000);
    }
}
