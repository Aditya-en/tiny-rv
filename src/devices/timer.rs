use super::device::Device;
use crate::cpu::{Address, INTERRUPT};
use crate::interrupt::{InterruptController};
use crate::platform::timer_registers::*;

pub struct Timer {
    counter: u32,
    compare: u32,
    interrupt_pending: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self { counter: 0, compare: 0, interrupt_pending: false }
    }
}

impl Device for Timer {
    fn read8(&mut self, offset: Address) -> u8 {
        let addr = offset.0;

        if addr >= COUNTER && addr < COUNTER + 4 {
            let shift = (addr - COUNTER) * 8;
            ((self.counter >> shift) & 0xFF) as u8
        } else if addr >= COMPARE && addr < COMPARE + 4 {
            let shift = (addr - COMPARE) * 8;
            ((self.compare >> shift) & 0xFF) as u8
        } else if addr == STATUS {
            self.interrupt_pending as u8
        } else {
            panic!("invalid Timer register offset: 0x{:08x}", addr);
        }
    }

    fn write8(&mut self, offset: Address, value: u8) {
        let addr = offset.0;

        if addr >= COUNTER && addr < COUNTER + 4 {
            let shift = (addr - COUNTER) * 8;
            let mask = !(0xFFu32 << shift);
            self.counter = (self.counter & mask) | ((value as u32) << shift);
        } else if addr >= COMPARE && addr < COMPARE + 4 {
            let shift = (addr - COMPARE) * 8;
            let mask = !(0xFFu32 << shift);
            self.compare = (self.compare & mask) | ((value as u32) << shift);
        } else if addr == STATUS {
            // Any write acknowledges (clears) the pending interrupt.
            self.interrupt_pending = false;
        } else {
            panic!("invalid Timer register offset: 0x{:08x}", addr);
        }
    }
    fn tick(&mut self, int_controller: &mut InterruptController) {
        self.counter = self.counter.wrapping_add(1);
        if self.counter >= self.compare && !self.interrupt_pending {
            self.interrupt_pending = true;
            
            int_controller.add_interrupt(INTERRUPT::TIMER);
        }
    }
}
