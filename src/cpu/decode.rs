
use super::types::*;
use super::cpu::CPU;

impl CPU {
    pub fn decode(inst: RawInstruction) -> INSTRUCTION {
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
                        let rd = inst.rd();
                        let s1 = inst.rs1();
                        let s2 = inst.rs2();
                        match funct7.0 {
                            0b0000000 => return INSTRUCTION::ADD(rd, s1, s2),
                            0b0100000 => return INSTRUCTION::SUB(rd, s1, s2),
                            0b0000001 => return INSTRUCTION::MUL(rd, s1, s2),
                            _ => panic!("unknown funct7 in funct3 0b0 opcode 0b0110011"),
                        }
                    }
                    0b100 => {
                        let funct7 = inst.funct7();
                        let rd = inst.rd();
                        let s1 = inst.rs1();
                        let s2 = inst.rs2();
                        match funct7.0 {
                            0b0 => return INSTRUCTION::XOR(rd, s1, s2),
                            0b0000001 => return INSTRUCTION::DIV(rd, s1, s2),
                            _ => panic!("unknown funct7 in 0b100 and opcode 0b0110011"),
                        }
                    }
                    0b110 => {
                        let funct7 = inst.funct7();
                        let rd = inst.rd();
                        let s1 = inst.rs1();
                        let s2 = inst.rs2();
                        match funct7.0 {
                            0b0 => return INSTRUCTION::OR(rd, s1, s2),
                            0b0000001 => return INSTRUCTION::REM(rd, s1, s2),
                            _ => panic!("unknown funct7 in 0b110 and opcode 0b0110011"),
                        }
                    }
                    0b111 => {
                        let funct7 = inst.funct7();
                        let rd = inst.rd();
                        let s1 = inst.rs1();
                        let s2 = inst.rs2();
                        match funct7.0 {
                            0b0 => return INSTRUCTION::AND(rd, s1, s2),
                            0b0000001 => return INSTRUCTION::REMU(rd, s1, s2),
                            _ => panic!("unknown funct7 in 0b111 and opcode 0b0110011"),
                        }
                    }
                    0b001 => {
                        let funct7 = inst.funct7();
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let rs2 = inst.rs2();
                        match funct7.0 {
                            0b0000001 => return INSTRUCTION::MULH(rd, rs1, rs2),
                            _ => return INSTRUCTION::SLL(rd, rs1, rs2),
                        }
                    }
                    0b101 => {
                        let funct7 = inst.funct7();
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let rs2 = inst.rs2();
                        match funct7.0 {
                            0b0 => return INSTRUCTION::SRL(rd, rs1, rs2),
                            0b0100000 => return INSTRUCTION::SRA(rd, rs1, rs2),
                            0b0000001 => return INSTRUCTION::DIVU(rd, rs1, rs2),
                            _ => panic!(
                                "unknown funct7 {:08b} for funct3 {:08b} for opcode {:08b}",
                                funct7.0, funct3.0, opcode.0
                            ),
                        }
                    }
                    0b010 => {
                        let funct7 = inst.funct7();
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let rs2 = inst.rs2();
                        match funct7.0 {
                            0b0000001 => return INSTRUCTION::MULHSU(rd, rs1, rs2),
                            _ => return INSTRUCTION::SLT(rd, rs1, rs2),
                        }
                    }
                    0b011 => {
                        let funct7 = inst.funct7();
                        let rd = inst.rd();
                        let rs1 = inst.rs1();
                        let rs2 = inst.rs2();
                        match funct7.0 {
                            0b0000001 => return INSTRUCTION::MULHU(rd, rs1, rs2),
                            _ => return INSTRUCTION::SLTU(rd, rs1, rs2),
                        }
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
            0b1110011 => {
                let funct3 = inst.funct3();
                let csr = inst.csr();

                match funct3.0 {
                    0b000 => {
                        if csr == 0x000 {
                            return INSTRUCTION::ECALL;
                        } else if csr == 0x302 {
                            return INSTRUCTION::MRET;
                        } else {
                            panic!("Unknown SYSTEM instruction or EBREAK");
                        }
                    }

                    // CSRRW
                    0b001 => {
                        return INSTRUCTION::CSRRW(
                            inst.rd(),
                            inst.rs1(),
                            csr,
                        );
                    }

                    // CSRRS
                    0b010 => {
                        return INSTRUCTION::CSRRS(
                            inst.rd(),
                            inst.rs1(),
                            csr,
                        );
                    }

                    // CSRRC
                    0b011 => {
                        return INSTRUCTION::CSRRC(
                            inst.rd(),
                            inst.rs1(),
                            csr,
                        );
                    }

                    // CSRRWI
                    0b101 => {
                        return INSTRUCTION::CSRRWI(
                            inst.rd(),
                            inst.rs1().0,
                            csr,
                        );
                    }

                    // CSRRSI
                    0b110 => {
                        return INSTRUCTION::CSRRSI(
                            inst.rd(),
                            inst.rs1().0,
                            csr,
                        );
                    }

                    // CSRRCI
                    0b111 => {
                        return INSTRUCTION::CSRRCI(
                            inst.rd(),
                            inst.rs1().0,
                            csr,
                        );
                    }

                    _ => panic!("Unknown SYSTEM instruction"),
                }
            }
            _ => {
                panic!("Unknown opcode")
            }
        }

    }
}