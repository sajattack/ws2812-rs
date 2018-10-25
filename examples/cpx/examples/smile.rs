#![feature(used, asm)]
#![no_std]
#![no_main]

extern crate circuit_playground_express;

#[macro_use(entry)]
extern crate cortex_m_rt;
extern crate cortex_m;

extern crate panic_abort;
extern crate ws2812;

use circuit_playground_express::clock::GenericClockController;
use circuit_playground_express::delay::Delay;
use circuit_playground_express::prelude::*;
use circuit_playground_express::{CorePeripherals, Peripherals};

use ws2812::WS2812;

entry!(main);

fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
     
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
     
    let mut pins = circuit_playground_express::Pins::new(peripherals.PORT);
    let mut neopixel = pins.neopixel.into_push_pull_output(&mut pins.port);

    let data: [u8;30] = [0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00,0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00];
    let mut neopixel = WS2812::new(neopixel);
    neopixel.write(&data);
    loop {}
}
