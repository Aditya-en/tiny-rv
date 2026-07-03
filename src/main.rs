#![allow(warnings)]

mod cpu;
mod devices;
mod bus;
mod machine;
mod assembler;
mod utils;

use cpu::{CPU, Address, RegisterIndex, Immediate, RawInstruction, Opcode, Funct3, Funct7, INSTRUCTION, Destination, Source1, Source2};
use bus::Bus;
use machine::{Machine, init_vm};
use devices::{UART, Device};


fn main() {
    let mut uart = UART::new();

    println!("UART Direct Test");

    uart.receive_byte(b'H');
    uart.receive_byte(b'i');

    println!("Status = {}", uart.read8(Address(UART::STATUS)));
    println!("{}", uart.read8(Address(UART::DATA)) as char);

    println!("Status = {}", uart.read8(Address(UART::STATUS)));
    println!("{}", uart.read8(Address(UART::DATA)) as char);

    println!("Status = {}", uart.read8(Address(UART::STATUS)));

    uart.write8(Address(UART::DATA), b'!');
    println!();
}