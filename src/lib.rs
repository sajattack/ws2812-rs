#![feature(used, asm)]
#![no_std]

extern crate embedded_hal;

use embedded_hal::digital::OutputPin;

pub struct WS2812<P: OutputPin> {
    pin: P,
}

impl <P: OutputPin> WS2812<P> {
    pub fn new(pin: P) -> WS2812<P> {
        WS2812 {
            pin: pin
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        let mut counter: usize = 0;
        let mut p: u8 = data[counter];
        counter += 1;
        let mut bitmask: u8 = 0x80;

        loop {
            self.pin.set_high();
            unsafe{ 
                asm!("nop; nop;");
            }
            if p & bitmask != 0 {
                unsafe{ 
                    asm!("nop; nop; nop; nop; nop; nop; nop;")
                }
                self.pin.set_low();
            } else {
                self.pin.set_low();
                unsafe{ 
                    asm!("nop; nop;");
                }
            }
            bitmask >>= 1;
            if bitmask != 0 {
                unsafe {
                    asm!("nop; nop; nop; nop; nop;");
                }
            } else {
                if counter >= data.len() {
                    break;
                }
                p = data[counter];
                counter += 1;
                bitmask = 0x80;
            }
        }
    }
}
