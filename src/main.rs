#![no_std]
#![no_main]

#[allow(unused)]
use panic_halt;

use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use hal::prelude::*; // need for the GpioExt trait (-> .split)

#[rt::entry]
fn main() -> ! {
    if let Some(peripherals) = hal::stm32::Peripherals::take() {
        //let rcc= peripherals.RCC.sysclk();
        let gpioc = peripherals.GPIOC.split();
        let gpiob = peripherals.GPIOB.split(); // + sets RCC->AHB1ENR GPIOB bit
        // .into_push_pull_output performs three steps
        // 1) set PUPDR: 00 -> no pull-up, no pull-down
        // 2) set OTYPER: 0 -> output push-pull
        // 3) set MODER: 01 -> general purpose output mode
        let mut led = gpiob.pb0.into_push_pull_output();
        let button = gpioc.pc13; // pins are input by default

        loop {
            // .is_high reads IDR
            if button.is_high().unwrap(){
                // .set_low uses BSRR
                led.set_high().unwrap();
            } else {
                led.set_low().unwrap();
                
            }
        }
    }
    loop {}
}

