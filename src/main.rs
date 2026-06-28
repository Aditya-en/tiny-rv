
#![allow(warnings)]
use std::{ops::Add, u32};

// constants
const MEMORY_SIZE:usize = 4*1024*1024;


#[derive(Debug)]
struct CPU {
    registers :[u32; 32],
    pc : Address
}

impl CPU {
    fn new() -> Self {
        Self { registers: [0; 32],  pc: Address(0x0) }
    }
    fn fetch(&mut self, mem: &Memory) -> RawInstruction {
        let b1 = mem.read(self.pc) as u32;
        let b2 = mem.read(self.pc + Address(1)) as u32;
        let b3 = mem.read(self.pc + Address(2)) as u32;
        let b4 = mem.read(self.pc + Address(3)) as u32;
        let inst : u32 = b4 << 24 | b3 << 16 | b2 << 8 | b1;

        self.pc = self.pc + Address(4);
        return RawInstruction(inst);
    }
    fn decode(inst: RawInstruction) -> INSTRUCTION{
        // TODO implement this completely
        
        let opcode = inst.opcode();
        // decide the type of opcode
        match opcode.0 {
            0b0010011 => {
                let funct3 = inst.funct3();
                match funct3.0 {
                    0b0 => {
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let imm = inst.i_imm();
                        return INSTRUCTION::ADDI(rd, rs1, imm)
                    }
                    _ => {
                        panic!("unknown funct3 with opcode 0b0010011")
                    }
                }
            }
            0b0110011 => {
                let funct3 = inst.funct3();
                match funct3.0 {
                    0b000 => {
                        let funct7 = inst.funct7();
                        match funct7.0 {
                            0b0000000 => {
                                let rd = inst.rd();
                                let s1 = inst.rs1();
                                let s2 = inst.rs2();
                                return INSTRUCTION::ADD(rd, s1, s2)
                            }
                            0b0100000 => {
                                let rd = inst.rd();
                                let s1 = inst.rs1();
                                let s2 = inst.rs2();
                                return INSTRUCTION::SUB(rd, s1, s2)
                            }
                            _ => {
                                panic!("unknown funct7 in funct3 0b0 opcode 0b0110011")
                            }
                        }
                    }
                    0b100 => {
                        let funct7 = inst.funct7();
                        match funct7.0 {
                            0b0 => {
                                let rd = inst.rd();
                                let s1 = inst.rs1();
                                let s2 = inst.rs2();
                                return INSTRUCTION::XOR(rd, s1, s2)
                            }
                            _ => {
                                panic!("unknown funct7 in 0b100 and opcode 0b0110011")
                            }
                        }
                    }
                    0b110 => {
                        let funct7 = inst.funct7();
                        match funct7.0 {
                            0b0 => {
                                let rd = inst.rd();
                                let s1 = inst.rs1();
                                let s2 = inst.rs2();
                                return INSTRUCTION::OR(rd, s1, s2)
                            }
                            _ => {
                                panic!("unknown funct7 in 0b110 and opcode 0b0110011")
                            }
                        }
                    }
                    0b111 => {
                        let funct7 = inst.funct7();
                        match funct7.0 {
                            0b0 => {
                                let rd = inst.rd();
                                let s1 = inst.rs1();
                                let s2 = inst.rs2();
                                return INSTRUCTION::AND(rd, s1, s2)
                            }
                            _ => {
                                panic!("unknown funct7 in 0b111 and opcode 0b0110011")
                            }
                        }
                    }
                    _ => {
                        panic!("unknown funct3 with opcode 0b0110011")
                    }
                }
            }
            _ => {
                panic!("Unknown opcode")
            }
        }

    }
    fn execute(&mut self, inst: INSTRUCTION,  mem: &mut Memory) {
        match inst {
            INSTRUCTION::ADDI(rd, rs1, imm ) => {
                let x = self.registers[rs1.0 as usize]
                    .wrapping_add(imm.0 as u32);

                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::ADD(rd, rs1, rs2) => {
                let x = self.registers[rs1.0 as usize]
                    .wrapping_add(self.registers[rs2.0 as usize]);
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::SUB(rd, rs1, rs2) => {
                let x = self.registers[rs1.0 as usize]
                    .wrapping_sub(self.registers[rs2.0 as usize]);
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::AND(rd, rs1, rs2) => {
                let x = self.registers[rs1.0 as usize] & (self.registers[rs2.0 as usize]);
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::OR(rd, rs1, rs2) => {
                let x = self.registers[rs1.0 as usize] | (self.registers[rs2.0 as usize]);
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::XOR(rd, rs1, rs2) => {
                let x = self.registers[rs1.0 as usize] ^ (self.registers[rs2.0 as usize]);
                self.registers[rd.0 as usize] = x;
            }
            _ => {
                println!("instruction not implemented")
            }
        }
        self.registers[0] = 0; // after executing any instruction ensure x0 is always 0
    }
    fn step(&mut self, mem: &mut Memory) {
        let raw_inst = self.fetch(mem);
        let inst = CPU::decode(raw_inst);
        self.execute(inst, mem);
    }

}
#[derive(Debug, Clone, Copy)]
struct Address(u32);
impl Add for Address {
    type Output = Self;

    fn add(self, rhs: Address) -> Self {
        return Address(self.0 + rhs.0)
    }
}
#[derive(Debug)]
struct RegisterIndex(u8); // there are only 32 registers
impl RegisterIndex{
    fn new(x: u8) -> Self{
        if x > 31 {
            panic!("Invalid index for register")
        }
        Self(x)
    }
}

struct Memory{
    data: Vec<u8>
}

impl Memory {
    fn new() -> Self {
        Self { data: vec![0; MEMORY_SIZE] }
    }
    fn read(&self, add: Address) -> u8 {
        if add.0 as usize >= MEMORY_SIZE {
            panic!("invalid memory read: Out of Bounds")
        }
        return self.data[add.0 as usize]
    }
    fn write(&mut self, add: Address, value: u8) {
        if add.0 as usize >= MEMORY_SIZE {
            panic!("invalid memory write: Out of Bounds")
        }
        self.data[add.0 as usize] = value
    }
    fn write32(&mut self, add: Address, value: u32) {
        if (add + Address(4)).0 as usize >= MEMORY_SIZE {
            panic!("invalid memory write: Out of Bounds")
        }
        let b1 = (value >> 24) as u8;
        let b2 = (value >> 16 & 0b1111_1111) as u8;
        let b3 = (value >> 8 & 0b1111_1111) as u8;
        let b4 = (value & 0b1111_1111) as u8;
        self.write(add, b4);
        self.write(add + Address(1), b3);
        self.write(add + Address(2), b2);
        self.write(add + Address(3), b1);
    }
    fn write_n(&mut self, add: Address, bytes: &[u8]){
        let mut i = 0;
        while i < bytes.len() {
            self.write(add + Address(i as u32), bytes[i]);
            i += 1;
        }
    }
}

type Destination = RegisterIndex;
type Source1 = RegisterIndex;
type Source2 = RegisterIndex;
#[derive(Debug)]
struct Immediate(i32);
impl Immediate{
    fn new(code: u16) -> Self {
        if code > 0b1111_1111_1111 {
            panic!("invalid immediate")
        }
        // do sign extension if value is negative
        let mut  new_code = code as u32;
        if (code & 1 << 11) != 0 {
            new_code = (u32::MAX) << 12 | code as u32;
        }
        Self(new_code as i32)
    }
}
#[derive(Debug)]
struct RawInstruction(u32);
impl RawInstruction {
    fn opcode(&self) -> Opcode {
        let code = self.0 & 0b111_1111 as u32;
        Opcode::new(code as u8)
    }
    fn funct3(&self) -> Funct3 {
        let code = (self.0 >> 12) & 0b111 as u32;
        Funct3::new(code as u8)
    }
    fn funct7(&self) -> Funct7 {
        let code = self.0 >> 25 & 0b111_1111 as u32;
        Funct7::new(code as u8)
    }
    fn rd(&self) -> Destination {
        let code = self.0 >> 7 & 0b1_1111 as u32;
        RegisterIndex::new(code as u8)
    }
    fn rs1(&self) -> Source1 {
        let code = self.0 >> 15 & 0b1_1111 as u32;
        RegisterIndex::new(code as u8)
    }
    fn rs2(&self) -> Source2 {
        let code = self.0 >> 20 & 0b1_1111 as u32;
        RegisterIndex::new(code as u8)
    }
    fn i_imm(&self) -> Immediate {
        let code = self.0 >> 20 & 0b1111_1111_1111 as u32;
        Immediate::new(code as u16)
    }
    fn s_imm(&self) -> Immediate {
        let code_1 = self.0 >> 7 & 0b1_1111 as u32;
        let code_2 = self.0 >> 25 & 0b111_1111 as u32;
        let code = code_2 << 5 | code_1;
        Immediate::new(code as u16) // will fit as only 12 bits are used
    }
    fn b_imm(&self) -> Immediate {
        // TODO can't understand this :(
        Immediate(0)
    }
    fn u_imm(&self) -> u32 {
        let code = self.0 & u32::MAX<<12;
        code
    }
    fn j_imm(&self) -> u32 {
        // TODO can't understand this :(
        0
    }

}
#[derive(Debug)]
struct Opcode(u8);
impl Opcode {
    fn new(code: u8) -> Self {
        if code > 127 {
            panic!("Invalid opcode")
        }
        Self(code)
    }
}
#[derive(Debug)]
struct Funct3(u8);
impl Funct3 {
    fn new(code: u8) -> Self{
        if code > 7 {
            panic!("Invalid Funct3")
        }
        Self(code)
    }
}
#[derive(Debug)]
struct Funct7(u8);
impl Funct7 {
    fn new(code: u8) -> Self {
        if code > 127 {
            panic!("invalid funct7")
        }
        Funct7(code)
    }
}
#[derive(Debug)]
enum INSTRUCTION {
    ADDI(Destination, Source1, Immediate),
    ADD(Destination, Source1, Source2),
    SUB(Destination, Source1, Source2),
    AND(Destination, Source1, Source2),
    OR(Destination, Source1, Source2),
    XOR(Destination, Source1, Source2)
}

fn assembler_add(rd: u32, rs1: u32, rs2: u32) -> u32 {
    let opcode: u8 = 0b0110011;
    // let funct3: u8 = 0b000;
    // let funct7: u8 = 0b0;
    let mut x: u32 = 0;
    x = x | opcode as u32;
    x = x | rd << 7;
    x = x | rs1 << 15;
    x = x | rs2 << 20;
    x
}
fn assembler_sub(rd: u32, rs1: u32, rs2: u32) -> u32 {
    let opcode: u8 = 0b0110011;
    // let funct3: u8 = 0b000;
    let funct7: u8 = 0b0100000;
    let mut x: u32 = 0;
    x = x | opcode as u32;
    x = x | rd  << 7;
    x = x | rs1 << 15;
    x = x | rs2 << 20;
    x = x | (funct7 as u32) << 25;
    x
}
fn assemble_addi(rd: u32, rs1: u32, imm: u32) -> u32{
    let opcode: u8 = 0b0010011;
    let mut x = 0;
    x = x | opcode as u32;
    x = x | rd << 7;
    x = x | rs1 << 15;
    x = x | imm << 20;
    x
}

fn dump(cpu: &CPU) {
    println!("pc = {}", cpu.pc.0);
    for i in 0..8 {
        println!("x{:02} = {}", i, cpu.registers[i]);
    }
    println!("-------------------------");
}

fn main() {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();

    // Initial register values
    cpu.registers[1] = 100;
    cpu.registers[2] = 25;
    cpu.registers[3] = 7;

    // Program:
    //
    // x4 = x1 + 10
    // x5 = x4 + x2
    // x6 = x5 - x3
    // x7 = x6 + 1
    //
    mem.write32(Address(0),  assemble_addi(4, 1, 10));
    mem.write32(Address(4),  assembler_add(5, 4, 2));
    mem.write32(Address(8),  assembler_sub(6, 5, 3));
    mem.write32(Address(12), assemble_addi(7, 6, 1));

    println!("Before execution:");
    dump(&cpu);

    for i in 0..4 {
        println!("Executing instruction {}", i);
        cpu.step(&mut mem);
        dump(&cpu);
    }

    println!("Final results:");
    println!("x4 = {} (expected 110)", cpu.registers[4]);
    println!("x5 = {} (expected 135)", cpu.registers[5]);
    println!("x6 = {} (expected 128)", cpu.registers[6]);
    println!("x7 = {} (expected 129)", cpu.registers[7]);
}