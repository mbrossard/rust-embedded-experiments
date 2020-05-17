#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_semihosting;
extern crate cortex_m_semihosting;
#[macro_use(block)]
extern crate nb;

use cortex_m_semihosting::{hprintln};
use stm32f4xx_hal::{i2c::I2c, delay::Delay, prelude::*, stm32};

#[cortex_m_rt::entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Set up the LED connected to pin PC13.
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        // Set up I2C - SCL is PB6 and SDA is PB7; they are set to Alternate Function 4
        // as per the STM32F411xC/E datasheet page 48.
        let gpiob = dp.GPIOB.split();

        let mut delay = Delay::new(cp.SYST, clocks);

        let scl = gpiob.pb6.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb7.into_alternate_af4().set_open_drain();

        let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks);

        // initialize the sensor
        let mut sensor = apds9960::Apds9960::new(i2c);
        sensor.enable().unwrap();
        sensor.enable_proximity().unwrap();
        sensor.enable_gesture();

        let mut on = true;
        loop {
            if on {
                led.set_high().unwrap();
            } else {
                led.set_low().unwrap();
            }

            hprintln!("Proximity: {}", block!(sensor.read_proximity()).unwrap()).ok();

            delay.delay_ms(1000_u32);

            on = !on;
        }
    }

    loop {}
}
