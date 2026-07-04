use risc_v::bus::{Bus, MappedDevice};
use risc_v::devices::{Memory, UART};
use risc_v::cpu::Address;

fn main() {
    let mut bus = Bus::new();

    // Add RAM at 0x0 .. 0x0000ffff
    let mem = Memory::new();
    bus.add_device(MappedDevice(Address(0), Address(0x0000ffff), Box::new(mem)));

    // Create a UART and inject a byte from the host before mapping it
    let mut uart = UART::new();
    uart.receive_byte(b'H');

    // Map UART at 0x1000
    bus.add_device(MappedDevice(Address(0x1000), Address(0x10ff), Box::new(uart)));

    // Guest reads DATA register (should get the injected 'H')
    let b = bus.read8(Address(0x1000 + UART::DATA));
    println!("Bus read UART DATA => {}", b as char);

    // Guest writes to DATA register (this will print to host stdout via UART::write8)
    bus.write8(Address(0x1000 + UART::DATA), b'A');
}
