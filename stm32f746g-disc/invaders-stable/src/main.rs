#![no_main]
#![no_std]

use core::panic::PanicInfo;
use cortex_m::{asm, interrupt};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32f7::stm32f7x6::Peripherals;
use stm32f746g_disc::{
    gpio::{GpioPort, InputPin, OutputPin},
    init, lcd,
    lcd::Color,
    lcd::Lcd,
    print, println,
    random::Rng,
};

fn invaders(lcd: &mut Lcd, rng: &mut Rng) {
    let white = Color::rgb(0xff, 0xff, 0xff);
    let black = Color::rgb(0x00, 0x00, 0x00);
    let i_size: u32 = 10; // Number of pixels for each invader
    let interval: u32 = 8;
    let p_size: u32 = 5;

    lcd.set_background_color(white);
    let mut layer_1 = lcd.layer_1().unwrap();

    let mut y = interval;
    while (y + i_size * p_size) < lcd::HEIGHT as u32 {
        let mut x = interval;
        while (x + i_size * p_size) < lcd::WIDTH as u32 {
            let mut cy = y;
            for _j in 0..i_size {
                let mut bits: u32 = rng
                    .poll_and_get()
                    .expect("Failed to generate random number");
                for i in 0..i_size / 2 {
                    let h = 1 << (i_size - i - 1);
                    let l = 1 << i;
                    if bits & l != 0 {
                        bits |= h;
                    } else {
                        bits &= !h;
                    }
                }

                let mut cx = x;
                for _i in 0..i_size {
                    for k in 0..p_size {
                        for j in 0..p_size {
                            if bits & 0x1 != 0 {
                                layer_1.print_point_color_at(
                                    (cx + j) as usize,
                                    (cy + k) as usize,
                                    black,
                                );
                            } else {
                                layer_1.print_point_color_at(
                                    (cx + j) as usize,
                                    (cy + k) as usize,
                                    white,
                                );
                            }
                        }
                    }
                    bits >>= 1;
                    cx += p_size;
                }
                cy += p_size;
            }
            x += i_size * p_size + interval;
        }
        y += i_size * p_size + interval;
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC;
    let mut pwr = peripherals.PWR;
    let mut flash = peripherals.FLASH;
    let mut fmc = peripherals.FMC;
    let mut ltdc = peripherals.LTDC;
    let mut random_gen = peripherals.RNG;

    init::init_system_clock_216mhz(&mut rcc, &mut pwr, &mut flash);
    init::enable_gpio_ports(&mut rcc);

    let gpio_a = GpioPort::new(peripherals.GPIOA);
    let gpio_b = GpioPort::new(peripherals.GPIOB);
    let gpio_c = GpioPort::new(peripherals.GPIOC);
    let gpio_d = GpioPort::new(peripherals.GPIOD);
    let gpio_e = GpioPort::new(peripherals.GPIOE);
    let gpio_f = GpioPort::new(peripherals.GPIOF);
    let gpio_g = GpioPort::new(peripherals.GPIOG);
    let gpio_h = GpioPort::new(peripherals.GPIOH);
    let gpio_i = GpioPort::new(peripherals.GPIOI);
    let gpio_j = GpioPort::new(peripherals.GPIOJ);
    let gpio_k = GpioPort::new(peripherals.GPIOK);
    let mut pins = init::pins(
        gpio_a, gpio_b, gpio_c, gpio_d, gpio_e, gpio_f, gpio_g, gpio_h, gpio_i, gpio_j, gpio_k,
    );

    init::init_sdram(&mut rcc, &mut fmc);
    let mut lcd = init::init_lcd(&mut ltdc, &mut rcc);
    pins.display_enable.set(true);
    pins.backlight.set(true);

    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();

    layer_1.clear();
    layer_2.clear();
    lcd::init_stdout(layer_2);

    println!("Try pressing the blue button one the left side!");
    let mut rng = Rng::init(&mut random_gen, &mut rcc).expect("RNG init failed");

    let mut previous_button_state = pins.button.get();
    loop {
        // poll button state
        let current_button_state = pins.button.get();
        if current_button_state != previous_button_state {
            if current_button_state {
                invaders(&mut lcd, &mut rng);
                pins.led.toggle();
            }

            previous_button_state = current_button_state;
        }
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();

    if lcd::stdout::is_initialized() {
        println!("{}", info);
    }

    // OK to fire a breakpoint here because we know the microcontroller is connected to a debugger
    asm::bkpt();

    loop {}
}
