#![no_std]
#![no_main]

extern crate feather_m4;
extern crate panic_halt;

use feather_m4::{clock::*, delay::Delay, pac::*, prelude::*, timer::SpinTimer};
use smart_leds::{hsv::hsv2rgb, hsv::Hsv, SmartLedsWrite};

#[feather_m4::entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = feather_m4::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // Configure neopixel pin (PB3) as output
    let pixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = ws2812_timer_delay::Ws2812::new(SpinTimer::new(9), pixel_pin);

    loop {
        for j in 0..255u8 {
            let colors = [hsv2rgb(Hsv {
                hue: j,
                sat: 255,
                val: 2,
            })];
            neopixel.write(colors.iter().cloned()).unwrap();
            delay.delay_ms(100u8);
        }
    }
}
