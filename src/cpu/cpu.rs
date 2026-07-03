use super::types::*;
use crate::bus::Bus;

#[derive(Debug)]
pub struct CPU {
    pub registers: [u32; 32],
    pub pc: Address,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: Address(0x0),
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let raw_inst = self.fetch(bus);
        let inst = Self::decode(raw_inst);
        self.execute(inst, bus);
    }
}
