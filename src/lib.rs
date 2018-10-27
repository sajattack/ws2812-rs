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
    brightness: u8,
    auto_write: bool,
    buf: Box<[u8]>
}

impl <P: OutputPin> WS2812<P> {
    pub fn new(pin: P, n: usize, brightness: u8, auto_write: bool) -> WS2812<P> {
        let boxed_slice = vec![0u8; n*3].into_boxed_slice();
        WS2812 {
            pin: pin,
            n: n,
            brightness: brightness,
            auto_write: auto_write,
            buf: boxed_slice 
        }
    }

    pub fn set_pin(&mut self, pin: P) {
        self.pin = pin;
    }

    pub fn set_led_color(&mut self, n: usize, color: [u8;3]) {
        {
            let buf  = &mut self.buf;
            let start = (n-1)*4;
            buf[start] = color[1];
            buf[start+1] = color[0];
            buf[start+2] = color[2];
        }
        if self.auto_write { self.show() }
    }

    pub fn get_led_color(self, n: usize) -> [u8;3] {
            let buf = self.buf;
            let start = (n-1)*4;
            let g = buf[start];
            let r = buf[start + 1];
            let b = buf[start + 2];
            return [r, g, b];

    }

    pub fn get_buf_ref(&mut self) -> &[u8] {
        return &mut self.buf
    }

    pub fn get_num_leds(self) -> usize {
        return self.n;
    }

    pub fn set_brightness(&mut self, b: u8) {
        let new_brightness: u8 = b + 1;
        if new_brightness != self.brightness {
            let old_brightness = self.brightness - 1;
            let scale: u16;
            if old_brightness == 0 { scale = 0; }
            else if b == 255 { scale = 65535 / old_brightness as u16 }
            else { scale = ((new_brightness as u16) << 8) -1 / old_brightness as u16; }
            for c in self.buf.iter_mut() {
                *c = ((*c as u16 * scale) >> 8) as u8;
            }
            self.brightness = b;
            if self.auto_write { self.show() }
        }
    }

    pub fn get_brightness(self) -> u8{
        return self.brightness - 1;
    }

    pub fn clear(&mut self) {
        for byte in self.buf.iter_mut() {
            *byte = 0;
        }
        if self.auto_write { self.show() }
    }

    pub fn show(&mut self) {
        let ref_copy = self.buf.clone();
        self.write(&ref_copy);
    }
}
