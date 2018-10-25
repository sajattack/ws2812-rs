#![no_std]
#![feature(asm)]
#![feature(alloc)]

extern crate alloc;
extern crate embedded_hal;

mod write;

use embedded_hal::digital::OutputPin;

use alloc::vec;
use alloc::boxed::Box;
 
pub struct WS2812<P: OutputPin> {
    pin: P,
    n: usize,
    brightness: f64,
    auto_write: bool,
    buf: Box<[u8]>
}

impl <P: OutputPin> WS2812<P> {
    pub fn new(pin: P, n: usize, brightness: f64, auto_write: bool) -> WS2812<P> {
        let boxed_slice = vec![0u8; n*3].into_boxed_slice();
        WS2812 {
            pin: pin,
            n: n,
            brightness: brightness,
            auto_write: auto_write,
            buf: boxed_slice 
        }
    }

    pub fn set_pixel_color(&mut self, n: usize, r: u8, g: u8, b: u8) {
        {
            let buf  = &mut self.buf;
            buf[n-1 * 3] = g;
            buf[n * 3] = r;
            buf[(n + 1)*3] = b;
        }
        if self.auto_write { self.show() }
    }

    pub fn show(&mut self) {
        let ref_copy = self.buf.clone();
        self.write(&ref_copy);
    }
}
