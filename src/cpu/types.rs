use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrivilegeMode {
    User = 0b00,
    Machine = 0b11,
}

#[derive(Debug, Clone, Copy)]
pub struct Address(pub u32);
impl Add for Address {
    type Output = Self;

    fn add(self, rhs: Address) -> Self {
        Address(self.0 + rhs.0)
    }
}

#[derive(Debug)]
pub struct RegisterIndex(pub u8); // there are only 32 registers
impl RegisterIndex {
    pub fn new(x: u8) -> Self {
        if x > 31 {
            panic!("Invalid index for register")
        }
        Self(x)
    }
}

#[derive(Debug)]
pub struct Immediate(pub i32);
impl Immediate {
    pub fn new(code: u16) -> Self {
        if code > 0b1111_1111_1111 {
            panic!("invalid immediate")
        }
        // do sign extension if value is negative
        let mut new_code = code as u32;
        if (code & 1 << 11) != 0 {
            new_code = (u32::MAX) << 12 | code as u32;
        }
        Self(new_code as i32)
    }
}

#[derive(Debug)]
pub struct RawInstruction(pub u32);
impl RawInstruction {
    pub fn opcode(&self) -> Opcode {
        let code = self.0 & 0b111_1111 as u32;
        Opcode::new(code as u8)
    }
    pub fn funct3(&self) -> Funct3 {
        let code = (self.0 >> 12) & 0b111 as u32;
        Funct3::new(code as u8)
    }
    pub fn funct7(&self) -> Funct7 {
        let code = self.0 >> 25 & 0b111_1111 as u32;
        Funct7::new(code as u8)
    }
    pub fn rd(&self) -> Destination {
        let code = self.0 >> 7 & 0b1_1111 as u32;
        RegisterIndex::new(code as u8)
    }
    pub fn rs1(&self) -> Source1 {
        let code = self.0 >> 15 & 0b1_1111 as u32;
        RegisterIndex::new(code as u8)
    }
    pub fn rs2(&self) -> Source2 {
        let code = self.0 >> 20 & 0b1_1111 as u32;
        RegisterIndex::new(code as u8)
    }
    pub fn csr(&self) -> u16 {
        ((self.0 >> 20) & 0xFFF) as u16
    }
    pub fn i_imm(&self) -> Immediate {
        let code = self.0 >> 20 & 0b1111_1111_1111 as u32;
        Immediate::new(code as u16)
    }
    pub fn s_imm(&self) -> Immediate {
        let code_1 = self.0 >> 7 & 0b1_1111 as u32;
        let code_2 = self.0 >> 25 & 0b111_1111 as u32;
        let code = code_2 << 5 | code_1;
        Immediate::new(code as u16) // will fit as only 12 bits are used
    }
    pub fn b_imm(&self) -> Immediate {
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
    pub fn u_imm(&self) -> u32 {
        let code = self.0 & u32::MAX << 12;
        code
    }
    pub fn j_imm(&self) -> u32 {
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
pub struct Opcode(pub u8);
impl Opcode {
    pub fn new(code: u8) -> Self {
        if code > 127 {
            panic!("Invalid opcode")
        }
        Self(code)
    }
}

#[derive(Debug)]
pub struct Funct3(pub u8);
impl Funct3 {
    pub fn new(code: u8) -> Self {
        if code > 7 {
            panic!("Invalid Funct3")
        }
        Self(code)
    }
}

#[derive(Debug)]
pub struct Funct7(pub u8);
impl Funct7 {
    pub fn new(code: u8) -> Self {
        if code > 127 {
            panic!("invalid funct7")
        }
        Funct7(code)
    }
}

pub type Destination = RegisterIndex;
pub type Source1 = RegisterIndex;
pub type Source2 = RegisterIndex;

#[derive(Debug)]
pub enum INSTRUCTION {
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
    ECALL,
    MRET,
    MUL(Destination, Source1, Source2),
    MULH(Destination, Source1, Source2),
    MULHSU(Destination, Source1, Source2),
    MULHU(Destination, Source1, Source2),
    DIV(Destination, Source1, Source2),
    DIVU(Destination, Source1, Source2),
    REM(Destination, Source1, Source2),
    REMU(Destination, Source1, Source2),
    CSRRW(Destination, Source1, u16),
    CSRRS(Destination, Source1, u16),
    CSRRC(Destination, Source1, u16),

    CSRRWI(Destination, u8, u16),
    CSRRSI(Destination, u8, u16),
    CSRRCI(Destination, u8, u16),
}
#[derive(PartialEq)]
pub enum INTERRUPT {
    TIMER,
    UART,
    SOFTWARE,
    KEYBOARD,
}