use risc_v::assembler;

fn main() {
    let a = assembler::assemble_addi(1, 0, 10);
    let b = assembler::assemble_add(2, 1, 1);

    println!("assemble_addi -> 0x{:08x}", a);
    println!("assemble_add ->  0x{:08x}", b);
}
