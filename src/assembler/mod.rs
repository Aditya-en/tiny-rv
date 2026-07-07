// Generic instruction builders
pub fn assemble_r_type(opcode: u32, funct3: u32, funct7: u32, rd: u32, rs1: u32, rs2: u32) -> u32 {
    (opcode & 0x7F)
        | ((rd & 0x1F) << 7)
        | ((funct3 & 0x7) << 12)
        | ((rs1 & 0x1F) << 15)
        | ((rs2 & 0x1F) << 20)
        | ((funct7 & 0x7F) << 25)
}

pub fn assemble_i_type(opcode: u32, funct3: u32, rd: u32, rs1: u32, imm: u32) -> u32 {
    (opcode & 0x7F)
        | ((rd & 0x1F) << 7)
        | ((funct3 & 0x7) << 12)
        | ((rs1 & 0x1F) << 15)
        | ((imm & 0xFFF) << 20)
}

pub fn assemble_b_type(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: u32) -> u32 {
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

pub fn assemble_bne(rs1: u32, rs2: u32, imm: i32) -> u32 {
    // We cast to u32 so the bitwise ops in assemble_b_type handle the sign correctly
    assemble_b_type(0b1100011, 0b001, rs1, rs2, imm as u32)
}

// I-Type Arithmetics (Opcode: 0b0010011)
pub fn assemble_addi(rd: u32, rs1: u32, imm: u32) -> u32 {
    assemble_i_type(0b0010011, 0b000, rd, rs1, imm)
}

pub fn assemble_slti(rd: u32, rs1: u32, imm: u32) -> u32 {
    assemble_i_type(0b0010011, 0b010, rd, rs1, imm)
}

pub fn assemble_xori(rd: u32, rs1: u32, imm: u32) -> u32 {
    assemble_i_type(0b0010011, 0b100, rd, rs1, imm)
}

pub fn assemble_ori(rd: u32, rs1: u32, imm: u32) -> u32 {
    assemble_i_type(0b0010011, 0b110, rd, rs1, imm)
}

pub fn assemble_andi(rd: u32, rs1: u32, imm: u32) -> u32 {
    assemble_i_type(0b0010011, 0b111, rd, rs1, imm)
}

// I-Type Shifts (Technically I-type, but the top 7 bits of the immediate act as a funct7)
pub fn assemble_slli(rd: u32, rs1: u32, shamt: u32) -> u32 {
    assemble_i_type(0b0010011, 0b001, rd, rs1, shamt & 0x1F)
}

pub fn assemble_srli(rd: u32, rs1: u32, shamt: u32) -> u32 {
    assemble_i_type(0b0010011, 0b101, rd, rs1, shamt & 0x1F)
}

// For SRAI, the 10th bit of the immediate field (bit 30 of the instruction) must be 1. (0x400 = 0b0100_0000_0000)
pub fn assemble_srai(rd: u32, rs1: u32, shamt: u32) -> u32 {
    assemble_i_type(0b0010011, 0b101, rd, rs1, (shamt & 0x1F) | 0x400)
}

// R-Type Arithmetics (Opcode: 0b0110011)
pub fn assemble_add(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b000, 0b0000000, rd, rs1, rs2)
}

pub fn assemble_sub(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b000, 0b0100000, rd, rs1, rs2)
}

pub fn assemble_sll(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b001, 0b0000000, rd, rs1, rs2)
}

pub fn assemble_slt(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b010, 0b0000000, rd, rs1, rs2)
}

pub fn assemble_xor(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b100, 0b0000000, rd, rs1, rs2)
}

pub fn assemble_srl(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b101, 0b0000000, rd, rs1, rs2)
}

pub fn assemble_sra(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b101, 0b0100000, rd, rs1, rs2)
}

pub fn assemble_or(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b110, 0b0000000, rd, rs1, rs2)
}

pub fn assemble_and(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b111, 0b0000000, rd, rs1, rs2)
}

// M-Extension (Opcode: 0b0110011, Funct7: 0b0000001)
pub fn assemble_mul(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b000, 0b0000001, rd, rs1, rs2)
}
pub fn assemble_mulh(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b001, 0b0000001, rd, rs1, rs2)
}
pub fn assemble_mulhsu(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b010, 0b0000001, rd, rs1, rs2)
}
pub fn assemble_mulhu(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b011, 0b0000001, rd, rs1, rs2)
}
pub fn assemble_div(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b100, 0b0000001, rd, rs1, rs2)
}
pub fn assemble_divu(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b101, 0b0000001, rd, rs1, rs2)
}
pub fn assemble_rem(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b110, 0b0000001, rd, rs1, rs2)
}
pub fn assemble_remu(rd: u32, rs1: u32, rs2: u32) -> u32 {
    assemble_r_type(0b0110011, 0b111, 0b0000001, rd, rs1, rs2)
}