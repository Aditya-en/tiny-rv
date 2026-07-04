use risc_v::machine::init_vm;
use risc_v::assembler;
use risc_v::bus::MappedDevice;
use risc_v::devices::UART;
use risc_v::cpu::Address;

fn main() {
    let mut m = init_vm();

    // Map a UART at 0x1000 so we can observe output
    let uart = UART::new();
    m.bus.add_device(MappedDevice(Address(0x1000), Address(0x10ff), Box::new(uart)));

    // Load a tiny program: addi x1, x0, 65 (ASCII 'A')
    let inst = assembler::assemble_addi(1, 0, 65);
    m.bus.write32(Address(0), inst);

    // Step CPU to execute the instruction
    m.cpu.step(&mut m.bus);

    println!("After 1 step, x1 = {}", m.cpu.registers[1]);

    // As an example of producing UART output from the host side:
    m.bus.write8(Address(0x1000 + 0), b'!');
}
