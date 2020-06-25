#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_semihosting;

use bme280::BME280;
use core::fmt::Write;
use cortex_m_semihosting::{dbg, hio};

use embedded_hal::blocking::delay::DelayMs;
use lpc55_hal::{
    drivers::{pins::Level, I2cMaster, Pins},
    prelude::*,
};

struct LpcDelay {}

impl DelayMs<u8> for LpcDelay {
    fn delay_ms(&mut self, ms: u8) {
        cortex_m::asm::delay(10000 * (ms as u32));
    }
}
impl DelayMs<u32> for LpcDelay {
    fn delay_ms(&mut self, ms: u32) {
        cortex_m::asm::delay(10000 * ms);
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let hal = lpc55_hal::new();

    let mut anactrl = hal.anactrl;
    let mut pmc = hal.pmc;
    let mut syscon = hal.syscon;
    let mut iocon = hal.iocon.enabled(&mut syscon);
    let mut gpio = hal.gpio.enabled(&mut syscon);

    let clocks = lpc55_hal::ClockRequirements::default()
        .system_frequency(50.mhz())
        // .support_flexcomm()
        .configure(&mut anactrl, &mut pmc, &mut syscon)
        .unwrap();

    // cortex_m_semihosting::hprintln!("clocks = {:?}", &clocks).ok();

    let token = clocks.support_flexcomm_token().unwrap();

    let i2c = hal.flexcomm.4.enabled_as_i2c(&mut syscon, &token);

    let pins = Pins::take().unwrap();
    let scl = pins.pio1_20.into_i2c4_scl_pin(&mut iocon);
    let sda = pins.pio1_21.into_i2c4_sda_pin(&mut iocon);

    // let i2c = I2cMaster::new(i2c, (scl, sda), 400.khz());
    let i2c = I2cMaster::new(i2c, (scl, sda), 1.mhz());

    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "I2C Init").ok();

    // R = pio1_6
    let mut led = pins
        .pio1_6
        .into_gpio_pin(&mut iocon, &mut gpio)
        // on = low, off = high
        .into_output(Level::High);

    let delay = LpcDelay {};
    let mut bme280 = BME280::new(i2c, 0x76, delay);
    // let mut bme280 = BME280::new(i2c, 0x77, delay);

    writeln!(stdout, "bme280 new()").ok();

    // initialize the sensor
    bme280.init().unwrap();

    writeln!(stdout, "bme280 init()").ok();

    let mut on = true;
    loop {
        if on {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

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

        on = !on;
    }
}
