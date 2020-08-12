#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_semihosting;

use bme280::BME280;
use core::fmt::Write;
use cortex_m_semihosting::{dbg, hio};
use feather_m4::{clock, delay, pac::*, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut p = Peripherals::take().unwrap();
    let mut clocks = clock::GenericClockController::with_external_32kosc(
        p.GCLK,
        &mut p.MCLK,
        &mut p.OSC32KCTRL,
        &mut p.OSCCTRL,
        &mut p.NVMCTRL,
    );

    let delay = delay::Delay::new(cp.SYST, &mut clocks);
    let mut pins = feather_m4::Pins::new(p.PORT);

    let i2c = feather_m4::i2c_master(
        &mut clocks,
        400_000u32.hz(),
        p.SERCOM2,
        &mut p.MCLK,
        pins.sda,
        pins.scl,
        &mut pins.port,
    );
    let mut bme280 = BME280::new(i2c, 0x76, delay);
    // let mut bme280 = BME280::new(i2c, 0x77, delay);

    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "bme280 new()").ok();

    // initialize the sensor
    bme280.init().unwrap();

    writeln!(stdout, "bme280 init()").ok();

    loop {
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
