use std::collections::VecDeque;
use crate::cpu::Address;
use super::device::Device;

pub struct UART {
    pub rx: VecDeque<u8>,
}

impl UART {
    pub const DATA: u32 = 0;
    pub const STATUS: u32 = 4;
    pub const CONTROL: u32 = 8;

    pub fn new() -> Self {
        Self {
            rx: VecDeque::new(),
        }
    }

    // Host injects a received byte.
    pub fn receive_byte(&mut self, byte: u8) {
        self.rx.push_back(byte);
    }
}

impl Device for UART {
    fn read8(&mut self, offset: Address) -> u8 {
        match offset.0 {
            Self::DATA => self.rx.pop_front().unwrap_or(0),

            Self::STATUS => {
                if self.rx.is_empty() {
                    0
                } else {
                    1
                }
            }

            Self::CONTROL => 0,

            _ => panic!("invalid UART register"),
        }
    }

    fn write8(&mut self, offset: Address, value: u8) {
        match offset.0 {
            Self::DATA => {
                print!("{}", value as char);
            }

            Self::STATUS => {}

            Self::CONTROL => {}

            _ => panic!("invalid UART register"),
        }
    }
}
