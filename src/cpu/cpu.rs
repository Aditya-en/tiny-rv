use super::types::*;
use crate::bus::Bus;
use crate::cpu::types::INTERRUPT;

#[derive(Debug)]
pub struct CPU {
    pub registers: [u32; 32],
    pub pc: Address,
    pub interrupt_enabled: bool,
    pub interrupt_base: Address,
    pub mepc: u32,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: Address(0x0),
            interrupt_enabled: true,
            interrupt_base: Address(0x0),
            mepc: 0,
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let raw_inst = self.fetch(bus);
        let inst = Self::decode(raw_inst);
        self.execute(inst, bus);
    }

    pub fn handle_interrupt(&mut self, bus: &mut Bus, interrupt_type: INTERRUPT) {
        if self.interrupt_enabled {
            // self.registers[2] -= 4; // Decrement stack pointer
            // bus.write32(Address(self.registers[2]), self.pc.0); // Assuming x2 is the stack pointer
            self.mepc = self.pc.0;

            let handler_address = self.interrupt_base.0 + match interrupt_type {
                INTERRUPT::TIMER => 0x00,
                INTERRUPT::UART => 0x04,
                INTERRUPT::SOFTWARE => 0x08,
            };

            self.pc = Address(handler_address);

            // Clear interrupt flag finally
            self.interrupt_enabled = false;
        }
    }
}
