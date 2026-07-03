use crate::cpu::Address;
use super::device::Device;

const MEMORY_SIZE: usize = 4 * 1024 * 1024;

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: vec![0; MEMORY_SIZE],
        }
    }
}

impl Device for Memory {
    fn read8(&mut self, addr: Address) -> u8 {
        if addr.0 as usize >= MEMORY_SIZE {
            panic!("invalid memory read: Out of Bounds")
        }
        self.data[addr.0 as usize]
    }
    fn write8(&mut self, addr: Address, data: u8) {
        if addr.0 as usize >= MEMORY_SIZE {
            panic!("invalid memory write: Out of Bounds")
        }
        self.data[addr.0 as usize] = data
    }
}
