use embedded_hal::digital::OutputPin;
use crate::WS2812;

impl <P: OutputPin> WS2812<P> {
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

