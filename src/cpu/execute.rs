use super::types::*;
use super::cpu::CPU;
use crate::bus::Bus;
use crate::cpu::cpu::MEPC;

impl CPU {
    pub fn execute(&mut self, inst: INSTRUCTION, bus: &mut Bus) {
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
            INSTRUCTION::SRAI(rd, rs1, shamt ) => {
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
                let val = bus.read8(addr);
                // Cast to i8 to preserve sign, then up to i32/u32 for sign-extension
                self.registers[rd.0 as usize] = (val as i8) as i32 as u32; 
            }
            INSTRUCTION::LH(rd, rs1, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                let val = bus.read16(addr);
                // Cast to i16 for sign-extension
                self.registers[rd.0 as usize] = (val as i16) as i32 as u32; 
            }
            INSTRUCTION::LW(rd, rs1, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                self.registers[rd.0 as usize] = bus.read32(addr);
            }
            INSTRUCTION::LBU(rd, rs1, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                // Unsigned: No sign extension, just pad with zeros
                self.registers[rd.0 as usize] = bus.read8(addr) as u32; 
            }
            INSTRUCTION::LHU(rd, rs1, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                self.registers[rd.0 as usize] = bus.read16(addr) as u32; 
            }
            INSTRUCTION::SB(rs1, rs2, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                let val = (self.registers[rs2.0 as usize] & 0xFF) as u8;
                bus.write8(addr, val);
            }
            INSTRUCTION::SH(rs1, rs2, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                let val = (self.registers[rs2.0 as usize] & 0xFFFF) as u16;
                bus.write16(addr, val);
            }
            INSTRUCTION::SW(rs1, rs2, imm) => {
                let addr = Address(self.registers[rs1.0 as usize].wrapping_add(imm.0 as u32));
                let val = self.registers[rs2.0 as usize];
                bus.write32(addr, val);
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
            INSTRUCTION::MRET => {
                self.pc = Address(self.csr_file.read(MEPC));
                
                let mpie = self.csr_file.mpie();
                self.csr_file.set_mie(mpie);
                self.csr_file.set_mpie(true);
            }
            INSTRUCTION::MUL(rd, rs1, rs2) => {
                let x = self.registers[rs1.0 as usize].wrapping_mul(self.registers[rs2.0 as usize]);
                self.registers[rd.0 as usize] = x;
            }
            INSTRUCTION::MULH(rd, rs1, rs2) => {
                let a = self.registers[rs1.0 as usize] as i32 as i64;
                let b = self.registers[rs2.0 as usize] as i32 as i64;
                let result = (a * b) >> 32;
                self.registers[rd.0 as usize] = result as u32;
            }
            INSTRUCTION::MULHSU(rd, rs1, rs2) => {
                let a = self.registers[rs1.0 as usize] as i32 as i64;
                let b = self.registers[rs2.0 as usize] as u64 as i64; // zero-extend unsigned
                let result = (a * b) >> 32;
                self.registers[rd.0 as usize] = result as u32;
            }
            INSTRUCTION::MULHU(rd, rs1, rs2) => {
                let a = self.registers[rs1.0 as usize] as u64;
                let b = self.registers[rs2.0 as usize] as u64;
                let result = (a * b) >> 32;
                self.registers[rd.0 as usize] = result as u32;
            }
            INSTRUCTION::DIV(rd, rs1, rs2) => {
                let a = self.registers[rs1.0 as usize] as i32;
                let b = self.registers[rs2.0 as usize] as i32;
                let result = if b == 0 {
                    -1i32
                } else if a == i32::MIN && b == -1 {
                    i32::MIN
                } else {
                    a.wrapping_div(b)
                };
                self.registers[rd.0 as usize] = result as u32;
            }
            INSTRUCTION::DIVU(rd, rs1, rs2) => {
                let a = self.registers[rs1.0 as usize];
                let b = self.registers[rs2.0 as usize];
                let result = if b == 0 { u32::MAX } else { a.wrapping_div(b) };
                self.registers[rd.0 as usize] = result;
            }
            INSTRUCTION::REM(rd, rs1, rs2) => {
                let a = self.registers[rs1.0 as usize] as i32;
                let b = self.registers[rs2.0 as usize] as i32;
                let result = if b == 0 {
                    a
                } else if a == i32::MIN && b == -1 {
                    0
                } else {
                    a.wrapping_rem(b)
                };
                self.registers[rd.0 as usize] = result as u32;
            }
            INSTRUCTION::REMU(rd, rs1, rs2) => {
                let a = self.registers[rs1.0 as usize];
                let b = self.registers[rs2.0 as usize];
                let result = if b == 0 { a } else { a.wrapping_rem(b) };
                self.registers[rd.0 as usize] = result;
            }
            INSTRUCTION::CSRRW(rd, rs1, csr_addr) => {
                let csr_value = self.csr_file.read(csr_addr as usize);
                self.csr_file.write(csr_addr as usize, self.registers[rs1.0 as usize]);
                self.registers[rd.0 as usize] = csr_value;
            }
            INSTRUCTION::CSRRS(rd, rs1, csr_addr) => {
                let old = self.csr_file.read(csr_addr as usize);
                let value = old | self.registers[rs1.0 as usize];

                self.csr_file.write(csr_addr as usize, value);
                self.registers[rd.0 as usize] = old;
            }

            INSTRUCTION::CSRRC(rd, rs1, csr_addr) => {
                let old = self.csr_file.read(csr_addr as usize);
                let value = old & !self.registers[rs1.0 as usize];

                self.csr_file.write(csr_addr as usize, value);
                self.registers[rd.0 as usize] = old;
            }

            INSTRUCTION::CSRRWI(rd, zimm, csr_addr) => {
                let old = self.csr_file.read(csr_addr as usize);

                self.csr_file.write(csr_addr as usize, zimm as u32);
                self.registers[rd.0 as usize] = old;
            }

            INSTRUCTION::CSRRSI(rd, zimm, csr_addr) => {
                let old = self.csr_file.read(csr_addr as usize);
                let value = old | (zimm as u32);

                self.csr_file.write(csr_addr as usize, value);
                self.registers[rd.0 as usize] = old;
            }

            INSTRUCTION::CSRRCI(rd, zimm, csr_addr) => {
                let old = self.csr_file.read(csr_addr as usize);
                let value = old & !(zimm as u32);

                self.csr_file.write(csr_addr as usize, value);
                self.registers[rd.0 as usize] = old;
            }
        }
        self.registers[0] = 0; // after executing any instruction ensure x0 is always 0
    }
}