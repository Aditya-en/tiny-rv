use std::collections::VecDeque;

use crate::cpu::{Address, INTERRUPT};
use crate::interrupt::InterruptController;

use super::device::Device;

pub const DATA: u32 = 0x00;
pub const STATUS: u32 = 0x04;
pub const CONTROL: u32 = 0x08;

pub struct Keyboard {
    queue: VecDeque<u8>,
    interrupt_enabled: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            interrupt_enabled: false,
        }
    }

    pub fn push_key(&mut self, key: u8) {
        self.queue.push_back(key);
    }
}

impl Device for Keyboard {

    fn read8(&mut self, offset: Address) -> u8 {
        match offset.0 {

            DATA => {
                self.queue.pop_front().unwrap_or(0)
            }

            STATUS => {
                if self.queue.is_empty() {
                    0
                } else {
                    1
                }
            }

            CONTROL => {
                self.interrupt_enabled as u8
            }

            _ => panic!("invalid keyboard register"),
        }
    }

    fn write8(&mut self, offset: Address, value: u8) {
        match offset.0 {

            CONTROL => {
                self.interrupt_enabled = value != 0;
            }

            DATA | STATUS => {}

            _ => panic!("invalid keyboard register"),
        }
    }

    fn tick(&mut self, int_controller: &mut InterruptController) {
        if self.interrupt_enabled && !self.queue.is_empty() {
            int_controller.add_interrupt(INTERRUPT::KEYBOARD);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

