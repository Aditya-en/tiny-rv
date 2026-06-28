#![allow(warnings)]
use core::prelude::rust_2015;
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
                    0b100 => {
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let imm = inst.i_imm();
                        return INSTRUCTION::XORI(rd, rs1, imm)
                    }
                    0b110 => {
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let imm = inst.i_imm();
                        return INSTRUCTION::ORI(rd, rs1, imm)
                    }
                    0b111 => {
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let imm = inst.i_imm();
                        return INSTRUCTION::ANDI(rd, rs1, imm)
                    }
                    0b001 => {
                        // let funct7 = inst.funct7();
                        let rd = inst.rd();
                        let s1 = inst.rs1();
                        let shamt = inst.rs2();
                        return INSTRUCTION::SLLI(rd, s1, shamt)
                    }
                    0b101 => {
                        let funct7 = inst.funct7();
                        match funct7.0 {
                            0b0 => {
                                let rd = inst.rd();
                                let rs1 = inst.rs1();
                                let shamt = inst.rs2();
                                INSTRUCTION::SRLI(rd, rs1, shamt)
                            }
                            0b0100000 => {
                                let rd = inst.rd();
                                let rs1 = inst.rs1();
                                let shamt = inst.rs2();
                                INSTRUCTION::SRAI(rd, rs1, shamt)
                            }
                            _ => {
                                panic!("unknown funct7 {:08b} for funct3 {:08b} for opcode {:08b}", funct7.0, funct3.0, opcode.0)
                            }
                        }
                    }
                    0b010 => {
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let imm = inst.i_imm();
                        return INSTRUCTION::SLTI(rd, rs1, imm)
                    }
                    0b011 => {
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let imm = inst.i_imm();
                        return INSTRUCTION::SLTIU(rd, rs1, imm)
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
                    0b001 => {
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let rs2 = inst.rs2();
                        return INSTRUCTION::SLL(rd, rs1, rs2)
                    }
                    0b101 => {
                        let funct7 = inst.funct7();
                        match funct7.0 {
                            0b0 => {
                                let rd = inst.rd();
                                let rs1 = inst.rs1();
                                let rs2 = inst.rs2();
                                return INSTRUCTION::SRL(rd, rs1, rs2)
                            }
                            0b0100000 => {
                                let rd = inst.rd();
                                let rs1 = inst.rs1();
                                let rs2 = inst.rs2();
                                return INSTRUCTION::SRA(rd, rs1, rs2)
                            }
                            _ => {
                                panic!("unknown funct7 {:08b} for funct3 {:08b} for opcode {:08b}", funct7.0, funct3.0, opcode.0)
                            }
                        }
                    }
                    0b010 => {
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let rs2 = inst.rs2();
                        return INSTRUCTION::SLT(rd, rs1, rs2)
                    }
                    0b011 => {
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let rs2 = inst.rs2();
                        return INSTRUCTION::SLT(rd, rs1, rs2)
                    }
                    _ => {
                        panic!("unknown funct3 with opcode 0b0110011")
                    }
                }
            }
            0b0000011 => {
                let funct3 = inst.funct3();
                let rd = inst.rd();
                let rs1 = inst.rs1();
                let imm = inst.i_imm();
                match funct3.0 {
                    0b000 => return INSTRUCTION::LB(rd, rs1, imm),
                    0b001 => return INSTRUCTION::LH(rd, rs1, imm),
                    0b010 => return INSTRUCTION::LW(rd, rs1, imm),
                    0b100 => return INSTRUCTION::LBU(rd, rs1, imm),
                    0b101 => return INSTRUCTION::LHU(rd, rs1, imm),
                    _ => panic!("unknown funct3 for LOAD opcode 0b0000011"),
                }
            }
            0b0100011 => {
                let funct3 = inst.funct3();
                let rs1 = inst.rs1();
                let rs2 = inst.rs2();
                let imm = inst.s_imm();
                match funct3.0 {
                    0b000 => return INSTRUCTION::SB(rs1, rs2, imm),
                    0b001 => return INSTRUCTION::SH(rs1, rs2, imm),
                    0b010 => return INSTRUCTION::SW(rs1, rs2, imm),
                    _ => panic!("unknown funct3 for STORE opcode 0b0100011"),
                }
            }
            0b1100011 => {
                let funct3 = inst.funct3();
                let rs1 = inst.rs1();
                let rs2 = inst.rs2();
                let imm = inst.b_imm();
                match funct3.0 {
                    0b000 => return INSTRUCTION::BEQ(rs1, rs2, imm),
                    0b001 => return INSTRUCTION::BNE(rs1, rs2, imm),
                    0b100 => return INSTRUCTION::BLT(rs1, rs2, imm),
                    0b101 => return INSTRUCTION::BGE(rs1, rs2, imm),
                    0b110 => return INSTRUCTION::BLTU(rs1, rs2, imm),
                    0b111 => return INSTRUCTION::BGEU(rs1, rs2, imm),
                    _ => panic!("unknown funct3 for BRANCH opcode 0b1100011"),
                }
            }
            0b1101111 => {
                return INSTRUCTION::JAL(inst.rd(), inst.j_imm());
            }
            0b1100111 => {
                let funct3 = inst.funct3();
                if funct3.0 == 0b000 {
                    return INSTRUCTION::JALR(inst.rd(), inst.rs1(), inst.i_imm());
                } else {
                    panic!("unknown funct3 for JALR opcode");
                }
            }
            0b0110111 => {
                return INSTRUCTION::LUI(inst.rd(), inst.u_imm());
            }
            0b0010111 => {
                return INSTRUCTION::AUIPC(inst.rd(), inst.u_imm());
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
            INSTRUCTION::ANDI(rd, rs1, imm ) => {
                let x = self.registers[rs1.0 as usize] & imm.0 as u32;
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::ORI(rd, rs1, imm ) => {
                let x = self.registers[rs1.0 as usize] | imm.0 as u32;
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::XORI(rd, rs1, imm ) => {
                let x = self.registers[rs1.0 as usize] ^ imm.0 as u32;
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::SLLI(rd, rs1, shamt ) => {
                let x = self.registers[rs1.0 as usize] << ((shamt.0 as u32) & 0b1_1111); 
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::SRLI(rd, rs1, shamt ) => {
                let x = self.registers[rs1.0 as usize] >> ((shamt.0 as u32) & 0b1_1111); 
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::SRLI(rd, rs1, shamt ) => {
                let x = (self.registers[rs1.0 as usize] as i32) >> ((shamt.0 as u32) & 0b1_1111); 
                self.registers[rd.0 as usize] = x as u32;
            }
            INSTRUCTION::SRAI(rd, rs1, shamt) => {
                let x = (self.registers[rs1.0 as usize] as i32) >> ((shamt.0 as u32) & 0b1_1111);
                self.registers[rd.0 as usize] = x as u32;
            }
            INSTRUCTION::SLL(rd, rs1, rs2  ) => {
                let x = self.registers[rs1.0 as usize] << (self.registers[rs2.0 as usize]); 
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::SRL(rd, rs1, rs2  ) => {
                let x = self.registers[rs1.0 as usize] >> (self.registers[rs2.0 as usize]); 
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::SRA(rd, rs1, rs2  ) => {
                let x = (self.registers[rs1.0 as usize] as i32) >> (self.registers[rs2.0 as usize]); 
                self.registers[rd.0 as usize] = x as u32;
            }
            INSTRUCTION::SLT(rd, rs1,rs2 ) => {
                let x = (self.registers[rs1.0 as usize] as i32)< self.registers[rs2.0 as usize] as i32; 
                if x {
                    self.registers[rd.0 as usize] = 1;
                } else {
                    self.registers[rd.0 as usize] = 0;
                }
            }
            INSTRUCTION::SLTU(rd, rs1,rs2 ) => {
                let x = self.registers[rs1.0 as usize] < self.registers[rs2.0 as usize]; 
                if x {
                    self.registers[rd.0 as usize] = 1;
                } else {
                    self.registers[rd.0 as usize] = 0;
                }
            }
            INSTRUCTION::SLTI(rd, rs1,imm ) => {
                let x = (self.registers[rs1.0 as usize] as i32) < imm.0; 
                if x {
                    self.registers[rd.0 as usize] = 1;
                } else {
                    self.registers[rd.0 as usize] = 0;
                }
            }
            INSTRUCTION::SLTIU(rd, rs1,imm ) => {
                let x = self.registers[rs1.0 as usize] < imm.0 as u32; 
                if x {
                    self.registers[rd.0 as usize] = 1;
                } else {
                    self.registers[rd.0 as usize] = 0;
                }
            }
            INSTRUCTION::LB(rd, rs1, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                let val = mem.read(addr);
                // Cast to i8 to preserve sign, then up to i32/u32 for sign-extension
                self.registers[rd.0 as usize] = (val as i8) as i32 as u32; 
            }
            INSTRUCTION::LH(rd, rs1, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                let val = mem.read16(addr);
                // Cast to i16 for sign-extension
                self.registers[rd.0 as usize] = (val as i16) as i32 as u32; 
            }
            INSTRUCTION::LW(rd, rs1, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                self.registers[rd.0 as usize] = mem.read32(addr);
            }
            INSTRUCTION::LBU(rd, rs1, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                // Unsigned: No sign extension, just pad with zeros
                self.registers[rd.0 as usize] = mem.read(addr) as u32; 
            }
            INSTRUCTION::LHU(rd, rs1, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                self.registers[rd.0 as usize] = mem.read16(addr) as u32; 
            }
            INSTRUCTION::SB(rs1, rs2, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                let val = (self.registers[rs2.0 as usize] & 0xFF) as u8;
                mem.write(addr, val);
            }
            INSTRUCTION::SH(rs1, rs2, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                let val = (self.registers[rs2.0 as usize] & 0xFFFF) as u16;
                mem.write16(addr, val);
            }
            INSTRUCTION::SW(rs1, rs2, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                let val = self.registers[rs2.0 as usize];
                mem.write32(addr, val);
            }
            INSTRUCTION::LUI(rd, imm) => {
                // LUI places the U-immediate value in the top 20 bits of the destination register
                self.registers[rd.0 as usize] = imm;
            }
            INSTRUCTION::AUIPC(rd, imm) => {
                // AUIPC forms a 32-bit offset from the U-immediate, filling in the lowest 12 bits with zeros, 
                // and adds it to the address of the AUIPC instruction (PC - 4)
                let current_pc = self.pc.0.wrapping_sub(4);
                self.registers[rd.0 as usize] = current_pc.wrapping_add(imm);
            }
            INSTRUCTION::JAL(rd, imm) => {
                // Save the address of the NEXT instruction (which is currently self.pc.0) into rd
                self.registers[rd.0 as usize] = self.pc.0; 
                let current_pc = self.pc.0.wrapping_sub(4);
                self.pc = Address(current_pc.wrapping_add(imm));
            }
            INSTRUCTION::JALR(rd, rs1, imm) => {
                // Save the return address first
                let return_addr = self.pc.0; 
                // Target is rs1 + imm, with the least significant bit set to 0 (RISC-V quirk)
                let target = self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32) & !1;
                self.pc = Address(target);
                self.registers[rd.0 as usize] = return_addr;
            }
            INSTRUCTION::BEQ(rs1, rs2, imm) => {
                if self.registers[rs1.0 as usize] == self.registers[rs2.0 as usize] {
                    let current_pc = self.pc.0.wrapping_sub(4);
                    self.pc = Address(current_pc.wrapping_add(imm.0 as u32));
                }
            }
            INSTRUCTION::BNE(rs1, rs2, imm) => {
                if self.registers[rs1.0 as usize] != self.registers[rs2.0 as usize] {
                    let current_pc = self.pc.0.wrapping_sub(4);
                    self.pc = Address(current_pc.wrapping_add(imm.0 as u32));
                }
            }
            INSTRUCTION::BLT(rs1, rs2, imm) => {
                if (self.registers[rs1.0 as usize] as i32) < (self.registers[rs2.0 as usize] as i32) {
                    let current_pc = self.pc.0.wrapping_sub(4);
                    self.pc = Address(current_pc.wrapping_add(imm.0 as u32));
                }
            }
            INSTRUCTION::BGE(rs1, rs2, imm) => {
                if (self.registers[rs1.0 as usize] as i32) >= (self.registers[rs2.0 as usize] as i32) {
                    let current_pc = self.pc.0.wrapping_sub(4);
                    self.pc = Address(current_pc.wrapping_add(imm.0 as u32));
                }
            }
            INSTRUCTION::BLTU(rs1, rs2, imm) => {
                if self.registers[rs1.0 as usize] < self.registers[rs2.0 as usize] {
                    let current_pc = self.pc.0.wrapping_sub(4);
                    self.pc = Address(current_pc.wrapping_add(imm.0 as u32));
                }
            }
            INSTRUCTION::BGEU(rs1, rs2, imm) => {
                if self.registers[rs1.0 as usize] >= self.registers[rs2.0 as usize] {
                    let current_pc = self.pc.0.wrapping_sub(4);
                    self.pc = Address(current_pc.wrapping_add(imm.0 as u32));
                }
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
    fn write(&mut self, add: Address, value: u8) {
        if add.0 as usize >= MEMORY_SIZE {
            panic!("invalid memory write: Out of Bounds")
        }
        self.data[add.0 as usize] = value
    }
    fn write16(&mut self, add: Address, value: u16) {
        let b1 = (value & 0xFF) as u8;
        let b2 = (value >> 8) as u8;
        self.write(add, b1);
        self.write(add + Address(1), b2);
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
    fn read(&self, add: Address) -> u8 {
        if add.0 as usize >= MEMORY_SIZE {
            panic!("invalid memory read: Out of Bounds")
        }
        return self.data[add.0 as usize]
    }
    fn read16(&self, add: Address) -> u16 {
        let b1 = self.read(add) as u16;
        let b2 = self.read(add + Address(1)) as u16;
        b1 | (b2 << 8)
    }
    fn read32(&self, add: Address) -> u32 {
        let b1 = self.read(add) as u32;
        let b2 = self.read(add + Address(1)) as u32;
        let b3 = self.read(add + Address(2)) as u32;
        let b4 = self.read(add + Address(3)) as u32;
        b1 | (b2 << 8) | (b3 << 16) | (b4 << 24)
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
        let bit_12 = (self.0 >> 31) & 0x1;
        let bit_11 = (self.0 >> 7) & 0x1;
        let bits_10_5 = (self.0 >> 25) & 0x3F;
        let bits_4_1 = (self.0 >> 8) & 0xF;

        let mut imm = (bit_12 << 12) | (bit_11 << 11) | (bits_10_5 << 5) | (bits_4_1 << 1);
        if bit_12 == 1 {
            imm |= 0xFFFF_E000; // Sign extend
        }
        Immediate(imm as i32)
    }
    fn u_imm(&self) -> u32 {
        let code = self.0 & u32::MAX<<12;
        code
    }
    fn j_imm(&self) -> u32 {
        let bit_20 = (self.0 >> 31) & 0x1;
        let bits_19_12 = (self.0 >> 12) & 0xFF;
        let bit_11 = (self.0 >> 20) & 0x1;
        let bits_10_1 = (self.0 >> 21) & 0x3FF;

        let mut imm = (bit_20 << 20) | (bits_19_12 << 12) | (bit_11 << 11) | (bits_10_1 << 1);
        if bit_20 == 1 {
            imm |= 0xFFE0_0000; // Sign extend
        }
        imm
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
    XORI(Destination, Source1, Immediate),
    ANDI(Destination, Source1, Immediate),
    ORI(Destination, Source1, Immediate),
    ADD(Destination, Source1, Source2),
    SUB(Destination, Source1, Source2),
    AND(Destination, Source1, Source2),
    OR(Destination, Source1, Source2),
    XOR(Destination, Source1, Source2),
    SLLI(Destination, Source1, Source2),
    SRLI(Destination, Source1, Source2),
    SRAI(Destination, Source1, Source2),
    SLL(Destination, Source1, Source2),
    SRL(Destination, Source1, Source2),
    SRA(Destination, Source1, Source2),
    SLT(Destination, Source1, Source2),
    SLTU(Destination, Source1, Source2),
    SLTI(Destination, Source1, Immediate),
    SLTIU(Destination, Source1, Immediate),
    LB(Destination, Source1, Immediate),
    LH(Destination, Source1, Immediate),
    LW(Destination, Source1, Immediate),
    LBU(Destination, Source1, Immediate),
    LHU(Destination, Source1, Immediate),
    SB(Source1, Source2, Immediate),
    SH(Source1, Source2, Immediate),
    SW(Source1, Source2, Immediate),
    BEQ(Source1, Source2, Immediate),
    BNE(Source1, Source2, Immediate),
    BLT(Source1, Source2, Immediate),
    BGE(Source1, Source2, Immediate),
    BLTU(Source1, Source2, Immediate),
    BGEU(Source1, Source2, Immediate),
    JAL(Destination, u32),                 
    JALR(Destination, Source1, Immediate),
    LUI(Destination, u32),
    AUIPC(Destination, u32),
}

// generic instruction builders
fn assemble_r_type(opcode: u32, funct3: u32, funct7: u32, rd: u32, rs1: u32, rs2: u32) -> u32 {
    (opcode & 0x7F)
        | ((rd & 0x1F) << 7)
        | ((funct3 & 0x7) << 12)
        | ((rs1 & 0x1F) << 15)
        | ((rs2 & 0x1F) << 20)
        | ((funct7 & 0x7F) << 25)
}

fn assemble_i_type(opcode: u32, funct3: u32, rd: u32, rs1: u32, imm: u32) -> u32 {
    (opcode & 0x7F)
        | ((rd & 0x1F) << 7)
        | ((funct3 & 0x7) << 12)
        | ((rs1 & 0x1F) << 15)
        | ((imm & 0xFFF) << 20)
}

fn assemble_b_type(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: u32) -> u32 {
    let imm12 = (imm >> 12) & 0x1;
    let imm10_5 = (imm >> 5) & 0x3F;
    let imm4_1 = (imm >> 1) & 0xF;
    let imm11 = (imm >> 11) & 0x1;

    (opcode & 0x7F)
        | (imm11 << 7)
        | (imm4_1 << 8)
        | ((funct3 & 0x7) << 12)
        | ((rs1 & 0x1F) << 15)
        | ((rs2 & 0x1F) << 20)
        | (imm10_5 << 25)
        | (imm12 << 31)
}

fn assemble_bne(rs1: u32, rs2: u32, imm: i32) -> u32 {
    // We cast to u32 so the bitwise ops in assemble_b_type handle the sign correctly
    assemble_b_type(0b1100011, 0b001, rs1, rs2, imm as u32)
}

// I-Type Arithmetics (Opcode: 0b0010011)
fn assemble_addi(rd: u32, rs1: u32, imm: u32) -> u32 { assemble_i_type(0b0010011, 0b000, rd, rs1, imm) }
fn assemble_slti(rd: u32, rs1: u32, imm: u32) -> u32 { assemble_i_type(0b0010011, 0b010, rd, rs1, imm) }
fn assemble_xori(rd: u32, rs1: u32, imm: u32) -> u32 { assemble_i_type(0b0010011, 0b100, rd, rs1, imm) }
fn assemble_ori(rd: u32, rs1: u32, imm: u32)  -> u32 { assemble_i_type(0b0010011, 0b110, rd, rs1, imm) }
fn assemble_andi(rd: u32, rs1: u32, imm: u32) -> u32 { assemble_i_type(0b0010011, 0b111, rd, rs1, imm) }

// I-Type Shifts (Technically I-type, but the top 7 bits of the immediate act as a funct7)
fn assemble_slli(rd: u32, rs1: u32, shamt: u32) -> u32 { assemble_i_type(0b0010011, 0b001, rd, rs1, shamt & 0x1F) }
fn assemble_srli(rd: u32, rs1: u32, shamt: u32) -> u32 { assemble_i_type(0b0010011, 0b101, rd, rs1, shamt & 0x1F) }
// For SRAI, the 10th bit of the immediate field (bit 30 of the instruction) must be 1. (0x400 = 0b0100_0000_0000)
fn assemble_srai(rd: u32, rs1: u32, shamt: u32) -> u32 { assemble_i_type(0b0010011, 0b101, rd, rs1, (shamt & 0x1F) | 0x400) }

// R-Type Arithmetics (Opcode: 0b0110011)
fn assemble_add(rd: u32, rs1: u32, rs2: u32) -> u32 { assemble_r_type(0b0110011, 0b000, 0b0000000, rd, rs1, rs2) }
fn assemble_sub(rd: u32, rs1: u32, rs2: u32) -> u32 { assemble_r_type(0b0110011, 0b000, 0b0100000, rd, rs1, rs2) }
fn assemble_sll(rd: u32, rs1: u32, rs2: u32) -> u32 { assemble_r_type(0b0110011, 0b001, 0b0000000, rd, rs1, rs2) }
fn assemble_slt(rd: u32, rs1: u32, rs2: u32) -> u32 { assemble_r_type(0b0110011, 0b010, 0b0000000, rd, rs1, rs2) }
fn assemble_xor(rd: u32, rs1: u32, rs2: u32) -> u32 { assemble_r_type(0b0110011, 0b100, 0b0000000, rd, rs1, rs2) }
fn assemble_srl(rd: u32, rs1: u32, rs2: u32) -> u32 { assemble_r_type(0b0110011, 0b101, 0b0000000, rd, rs1, rs2) }
fn assemble_sra(rd: u32, rs1: u32, rs2: u32) -> u32 { assemble_r_type(0b0110011, 0b101, 0b0100000, rd, rs1, rs2) }
fn assemble_or(rd:  u32, rs1: u32, rs2: u32) -> u32 { assemble_r_type(0b0110011, 0b110, 0b0000000, rd, rs1, rs2) }
fn assemble_and(rd: u32, rs1: u32, rs2: u32) -> u32 { assemble_r_type(0b0110011, 0b111, 0b0000000, rd, rs1, rs2) }

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

    // A simple loop:
    // x1 = 0 (sum)
    // x2 = 5 (counter)
    // loop_start:
    //   x1 = x1 + x2
    //   x2 = x2 - 1
    //   if x2 != 0, goto loop_start (PC - 8 bytes)
    
    let program = [
        assemble_addi(1, 0, 0),             // pc=0:  x1 = 0
        assemble_addi(2, 0, 5),             // pc=4:  x2 = 5
        
        // --- loop_start ---
        assemble_add(1, 1, 2),              // pc=8:  x1 = x1 + x2
        assemble_addi(2, 2, -1i32 as u32),  // pc=12: x2 = x2 - 1
        assemble_bne(2, 0, -8),             // pc=16: if x2 != 0, jump back 8 bytes (to pc=8)
    ];

    // Load program into memory
    for (i, &inst) in program.iter().enumerate() {
        mem.write32(Address((i * 4) as u32), inst);
    }

    println!("Executing Program...");
    
    // Instead of stepping exactly program.len() times, we step until 
    // the Program Counter points to memory AFTER our program ends.
    // The program is 5 instructions * 4 bytes = 20 bytes long.
    let mut cycle = 0;
    while cpu.pc.0 < 20 {
        cpu.step(&mut mem);
        cycle += 1;
    }

    println!("Program finished in {} cycles.", cycle);
    println!("--- Final Register States ---");
    println!("x1 (Sum)     = {} (Expected: 15)", cpu.registers[1]);
    println!("x2 (Counter) = {} (Expected: 0)", cpu.registers[2]);
    println!("x0 (Zero)    = {} (Expected: 0)", cpu.registers[0]); 
}