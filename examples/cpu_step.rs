use risc_v::machine::init_vm;
use risc_v::assembler;
use risc_v::cpu::Address;

fn main() {
    let mut m = init_vm();

    // Assemble: addi x1, x0, 42
    let inst = assembler::assemble_addi(1, 0, 42);

    // Write instruction to memory address 0x0
    m.bus.write32(Address(0), inst);

    // Step the CPU once
    m.cpu.step(&mut m.bus);

    // Dump register x1
    println!("x1 = {}", m.cpu.registers[1]);
}
