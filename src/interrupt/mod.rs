use std::collections::VecDeque;

use crate::cpu::INTERRUPT;

pub struct InterruptController {
    // create a set of pending interrupts
    pending: VecDeque<INTERRUPT>
}

impl InterruptController {
    pub fn new() -> Self {
        Self { pending: VecDeque::new() }
    }
    pub fn add_interrupt(&mut self, interrupt: INTERRUPT) {
        // only interrupt if not already pending
        if !self.pending.contains(&interrupt) {
            self.pending.push_back(interrupt);
        }
    }

    pub fn next_interrupt(&mut self) -> Option<INTERRUPT> {
        // remove one pending Interrupt from the list and return it
        self.pending.pop_front()
    }
}