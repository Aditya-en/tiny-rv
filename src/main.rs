#![allow(warnings)]

use risc_v::devices::{Device, UART};
use risc_v::cpu::Address;
use risc_v::platform::uart_registers::*;

fn main() {
    let mut uart = UART::new();

    println!("UART Direct Test");

    uart.receive_byte(b'H');
    uart.receive_byte(b'i');

    println!("Status = {}", uart.read8(Address(STATUS)));
    println!("{}", uart.read8(Address(DATA)) as char);

    println!("Status = {}", uart.read8(Address(STATUS)));
    println!("{}", uart.read8(Address(DATA)) as char);

    println!("Status = {}", uart.read8(Address(STATUS)));

    uart.write8(Address(DATA), b'!');
    println!();
}
