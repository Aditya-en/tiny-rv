use crate::cpu::CPU;

/// Print CPU state (program counter and all registers)
pub fn dump(cpu: &CPU) {
    println!("pc = {}", cpu.pc.0);
    for i in 0..32 {
        println!("x{:02} = {}", i, cpu.registers[i]);
    }
}
