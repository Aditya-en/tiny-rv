
use super::types::*;
use super::cpu::CPU;
use crate::bus::Bus;

impl CPU {
    pub fn fetch(&mut self, bus: &mut Bus) -> RawInstruction {
        let b1 = bus.read8(self.pc) as u32;
        let b2 = bus.read8(self.pc + Address(1)) as u32;
        let b3 = bus.read8(self.pc + Address(2)) as u32;
        let b4 = bus.read8(self.pc + Address(3)) as u32;
        let inst : u32 = b4 << 24 | b3 << 16 | b2 << 8 | b1;

        self.pc = self.pc + Address(4);
        return RawInstruction(inst);
    }
}