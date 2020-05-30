#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_semihosting;
extern crate cortex_m_semihosting;
#[macro_use(block)]
extern crate nb;

use cortex_m_semihosting::{hprintln, hio};
use core::fmt::Write;
use stm32f4xx_hal::{i2c::I2c, delay::Delay, prelude::*, stm32};
use ads1x1x::{channel::*, Ads1x1x, SlaveAddr};

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

        let mut stdout = hio::hstdout().unwrap();
        writeln!(stdout, "Hello world!").ok();

        // initialize the ADC
        #[cfg(feature = "ads1115")]
        let mut adc = Ads1x1x::new_ads1115(i2c, SlaveAddr::default());
        #[cfg(feature = "ads1015")]
        let mut adc = Ads1x1x::new_ads1015(i2c, SlaveAddr::default());

        writeln!(stdout, "Hello world!").ok();
        let mut on = true;
        loop {
            if on {
                led.set_high().unwrap();
            } else {
                led.set_low().unwrap();
            }

            hprintln!("Measurements: [{}, {}, {}, {}]",
                block!(adc.read(&mut SingleA0)).unwrap(),
                block!(adc.read(&mut SingleA1)).unwrap(),
                block!(adc.read(&mut SingleA2)).unwrap(),
                block!(adc.read(&mut SingleA3)).unwrap()).ok();

            delay.delay_ms(1000_u32);

            on = !on;
        }
    }

    loop {}
}
