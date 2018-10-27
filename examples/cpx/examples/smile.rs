#![feature(used, asm, alloc, alloc_error_handler)]
#![no_std]
#![no_main]

extern crate circuit_playground_express;

#[macro_use(entry)]
extern crate cortex_m_rt;
extern crate cortex_m;
extern crate alloc_cortex_m;
extern crate alloc;

extern crate panic_abort;
extern crate ws2812;

use circuit_playground_express::clock::GenericClockController;
use circuit_playground_express::delay::Delay;
use circuit_playground_express::prelude::*;
use circuit_playground_express::{CorePeripherals, Peripherals};

use alloc_cortex_m::CortexMHeap;

use ws2812::WS2812;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

entry!(main);

fn main() -> ! {
    let start = cortex_m_rt::heap_start() as usize;
    let size = 32768; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
     
    let clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    /*
     *cortex_m::interrupt::disable();
     *peripherals.NVMCTRL.ctrlb.modify(|r, w| {
     *    let read_mode_w = w.readmode();
     *    read_mode_w.deterministic()
     *});
     */
     
    let mut pins = circuit_playground_express::Pins::new(peripherals.PORT);
    let neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = WS2812::new(neopixel_pin, 10, 1, false);
    neopixel.write(&[0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00,  0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00, 0x01, 0x01, 0x00]);
    /*
     *let yellow = [128, 128, 0];
     *neopixel.set_led_color(1, yellow);
     *neopixel.set_led_color(2, yellow);
     *neopixel.set_led_color(3, yellow);
     *neopixel.set_led_color(5, yellow);
     *neopixel.set_led_color(6, yellow);
     *neopixel.set_led_color(8, yellow);
     *neopixel.set_led_color(9, yellow);
     *neopixel.set_led_color(10, yellow);
     *neopixel.show();
     */
    loop {}
}

#[alloc_error_handler]
fn error(_: core::alloc::Layout) -> ! {
   panic!()
}
