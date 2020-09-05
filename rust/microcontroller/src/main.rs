//! Blink the Teensy's LED

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt::entry;
use cortex_m::asm::wfi;
use teensy4_bsp as bsp;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let peripherals = bsp::Peripherals::take().unwrap();
    let pins = bsp::t40::pins(peripherals.iomuxc);
    let mut led = bsp::configure_led(pins.p13);
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);

    loop {
        led.set_high().unwrap();
        systick.delay(500);

        led.set_low().unwrap();
        systick.delay(500);

        led.set_high().unwrap();
        systick.delay(150);
        led.set_low().unwrap();
        systick.delay(150);
        led.set_high().unwrap();
        systick.delay(150);

        led.set_low().unwrap();
        systick.delay(300);
        // wfi();
    }
}
