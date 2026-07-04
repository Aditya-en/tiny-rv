use risc_v::devices::Memory;
use risc_v::cpu::Address;
use risc_v::devices::Device;

fn main() {
    let mut mem = Memory::new();
    let addr = Address(0x10);

    mem.write8(addr, 0x2A);
    let val = mem.read8(addr);

    println!("Memory[0x{:08x}] = {:08x}", addr.0, val);
}
