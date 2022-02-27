use stm32f4xx_hal::{prelude::*, gpio::*};
use stm32f4xx_hal::gpio::gpioa::PA5;
use stm32f4xx_hal::gpio::gpioa::PA6;

pub struct Sw {
    pin: PA6<Input<Floating>>
}

impl Sw {
    pub fn new(pin: PA6<Input<Floating>>) -> Sw {
        Sw { pin: pin.into_floating_input() }
    }
    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }
    pub fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}

pub struct Led {
    pin: PA5<Output<PushPull>>
}

impl Led {
    pub fn new(pin: PA5<Input<Floating>>) -> Led {
        Led { pin: pin.into_push_pull_output() }
    }
    pub fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }
    pub fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }
}
