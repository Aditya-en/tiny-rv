use super::types::*;
use super::cpu::CPU;
use crate::bus::Bus;
use crate::mmu::{MMU, AccessType};

impl CPU {
    pub fn fetch(&mut self, bus: &mut Bus) -> Result<RawInstruction, u32> {
        let satp = self.csr_file.read(0x180);

        match MMU::translate(self.pc.0, &AccessType::Fetch, satp, self.mode, bus) {
            Ok(paddr) => {
                let inst = bus.read32(Address(paddr));
                self.pc = self.pc + Address(4);
                
                Ok(RawInstruction(inst))
            },
            Err(exception_cause) => {
                Err(exception_cause)
            }
        }
    }
}