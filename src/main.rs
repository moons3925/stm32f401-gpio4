#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m_rt::entry;
use stm32f4xx_hal::{prelude::*, stm32};

use stm32lib::gpio::Sw;
use stm32lib::gpio::Led;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = Led::new(gpioa.pa5);
    let sw = Sw::new(gpioa.pa6);

    loop {
        if sw.is_pressed() {
            led.turn_on();
        } else {
            led.turn_off();
        }
    }
}
