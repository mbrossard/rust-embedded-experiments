#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_semihosting;

use bme280::BME280;
use core::fmt::Write;
use cortex_m_semihosting::{dbg, hio};

use nrf52840_hal::{
    prelude::*,
    gpio::*,
    pac,
    twim::{self, Twim},
    delay::*,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();
    let pins0 = nrf52840_hal::gpio::p0::Parts::new(p.P0);

    let mut led_0 = pins0.p0_13.into_push_pull_output(Level::High);
    let mut led_1 = pins0.p0_14.into_push_pull_output(Level::High);
    let mut led_2 = pins0.p0_15.into_push_pull_output(Level::High);
    let mut led_3 = pins0.p0_16.into_push_pull_output(Level::High);

    // Connect SDA to P0.27 and SCL to pin P0.26
    let scl = pins0.p0_26.into_floating_input().degrade();
    let sda = pins0.p0_27.into_floating_input().degrade();

    let pins = twim::Pins { scl, sda };

    let i2c = Twim::new(p.TWIM0, pins, twim::Frequency::K400);

    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "I2C Init").ok();

    let delay = Delay::new(cp.SYST);
    let mut bme280 = BME280::new(i2c, 0x76, delay);
    // let mut bme280 = BME280::new(i2c, 0x77, delay);

    writeln!(stdout, "bme280 new()").ok();

    // initialize the sensor
    bme280.init().unwrap();

    writeln!(stdout, "bme280 init()").ok();

    let mut l = 0;
    loop {
        match l {
            0 => {
                led_0.set_low().unwrap();
                led_1.set_high().unwrap();
                led_2.set_high().unwrap();
                led_3.set_high().unwrap();
            }
            1 => {
                led_0.set_high().unwrap();
                led_1.set_low().unwrap();
                led_2.set_high().unwrap();
                led_3.set_high().unwrap();
            }
            2 => {
                led_0.set_high().unwrap();
                led_1.set_high().unwrap();
                led_2.set_high().unwrap();
                led_3.set_low().unwrap();
            }
            _ => {
                led_0.set_high().unwrap();
                led_1.set_high().unwrap();
                led_2.set_low().unwrap();
                led_3.set_high().unwrap();
            }
        }
        l = (l + 1) % 4;

        // measure temperature, pressure, and humidity
        match bme280.measure() {
            Ok(measurements) => {
                writeln!(stdout, "Relative Humidity = {}%", measurements.humidity).ok();
                writeln!(stdout, "Temperature = {} deg C", measurements.temperature).ok();
                writeln!(stdout, "Pressure = {} pascals", measurements.pressure).ok();
            }
            Err(error) => {
                dbg!(error);
            }
        }

        // delay.delay_ms(1000_u32);
        cortex_m::asm::delay(10000000);
    }
}
