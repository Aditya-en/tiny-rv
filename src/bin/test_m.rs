use risc_v::cpu::Address;
use risc_v::machine::{init_vm, Machine};
use risc_v::utils::dump;
use std::fs::File;
use std::io::Read;

fn load_binary(machine: &mut Machine, filename: &str, base_addr: u32) {
    let mut file = File::open(filename).expect("Failed to open binary");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read binary");

    for (i, byte) in buffer.iter().enumerate() {
        machine.bus.write8(Address(base_addr + i as u32), *byte);
    }
    machine.cpu.pc = Address(base_addr);
}

fn main() {
    let mut vm = init_vm();
    load_binary(&mut vm, "test_m.bin", 0x0);

    // The program is ~30 instructions long and ends by spinning on an
    // infinite loop, so 50 steps is comfortably enough to reach it with
    // every result register populated.
    for _ in 0..50 {
        vm.step();
    }

    dump(&vm.cpu);
}