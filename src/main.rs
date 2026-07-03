#![allow(warnings)]

use risc_v::devices::{Device, UART};
use risc_v::cpu::Address;

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
