//! Tiny-RV Platform Memory Map

use crate::cpu::Address;

/// RAM
pub const RAM_BASE: Address = Address(0x0000_0000);
pub const RAM_SIZE: u32     = 0x0001_0000; // 64 KiB

/// UART
pub const UART_BASE: Address = Address(0x1000_0000);
pub const UART_SIZE: u32 = 0x100;

/// Timer
pub const TIMER_BASE: Address = Address(0x1000_0100);
pub const TIMER_SIZE: u32 = 0x100;


/// Interrupt Vector Table
pub const INTERRUPT_VECTOR_BASE: Address = Address(0x0000_8000);
pub mod timer_registers {
    pub const COUNTER: u32 = 0x00;
    pub const COMPARE: u32 = 0x04;
    pub const STATUS:  u32 = 0x08;
}
pub mod uart_registers {
    pub const DATA:   u32 = 0x00;
    pub const STATUS: u32 = 0x04;
    pub const CONTROL: u32 = 0x08;
}