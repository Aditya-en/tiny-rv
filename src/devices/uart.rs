use std::collections::VecDeque;

use crate::cpu::Address;
use crate::platform::uart_registers::{CONTROL, DATA, STATUS};

use super::device::Device;

pub struct UART {
    rx: VecDeque<u8>,
}

impl UART {
    pub fn new() -> Self {
        Self {
            rx: VecDeque::new(),
        }
    }

    pub fn receive_byte(&mut self, byte: u8) {
        self.rx.push_back(byte);
    }
}

impl Device for UART {
    fn read8(&mut self, offset: Address) -> u8 {
        match offset.0 {
            DATA => {
                self.rx.pop_front().unwrap_or(0)
            }

            STATUS => {
                if self.rx.is_empty() {
                    0
                } else {
                    1
                }
            }

            CONTROL => {
                // No control functionality yet.
                0
            }

            _ => panic!("invalid UART register offset: 0x{:08x}", offset.0),
        }
    }

    fn write8(&mut self, offset: Address, value: u8) {
        match offset.0 {
            DATA => {
                // Transmit a byte to the host.
                print!("{}", value as char);
            }

            STATUS => {
                // Read-only for now.
            }

            CONTROL => {
                // Reserved for future UART configuration.
            }

            _ => panic!("invalid UART register offset: 0x{:08x}", offset.0),
        }
    }
}