use super::device::Device;
use crate::cpu::Address;

pub struct Timer {
    counter: u32,
}

impl Timer {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Device for Timer {
    fn read8(&mut self, offset: Address) -> u8 {
        match offset.0 {
            0..=3 => {
                let shift = offset.0 * 8;
                ((self.counter >> shift) & 0xFF) as u8
            }
            _ => panic!("invalid Timer register"),
        }
    }
    fn write8(&mut self, offset: Address, value: u8) {
        match offset.0 {
            0..=3 => {
                let shift = offset.0 * 8;
                let mask = !(0xFFu32 << shift);
                self.counter = (self.counter & mask) | ((value as u32) << shift);
            }
            _ => panic!("invalid Timer register"),
        }
    }
    fn tick(&mut self) {
        self.counter = self.counter.wrapping_add(1);
    }
}
