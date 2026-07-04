use risc_v::bus::{Bus, MappedDevice};
use risc_v::devices::UART;
use risc_v::cpu::Address;
use risc_v::platform::uart_registers::DATA;

fn main() {
    let mut bus = Bus::new();

    // Create UART and inject host keyboard input
    let mut uart = UART::new();
    uart.receive_byte(b'Z');

    // Map UART at 0x2000
    bus.add_device(MappedDevice(Address(0x2000), Address(0x20ff), Box::new(uart)));

    // Guest reads DATA register
    let ch = bus.read8(Address(0x2000 + DATA));
    println!("Guest read from UART DATA: {}", ch as char);

    // Guest writes characters to UART (host sees terminal output)
    for c in b"Hello\n".iter() {
        bus.write8(Address(0x2000 + DATA), *c);
    }
}
