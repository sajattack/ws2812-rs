use crate::WS2812;
use embedded_hal::digital::OutputPin;
use embedded_hal::prelude::*;

impl <P: OutputPin> WS2812<P> {
    pub fn write(&mut self, data: &[u8]) {
        unimplemented!()
    }
}

