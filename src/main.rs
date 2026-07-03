#![allow(warnings)]
use std::{ops::Add, u32};
use std::collections::VecDeque;

mod cpu;
use cpu::{CPU, Address};

// constants
const MEMORY_SIZE:usize = 4*1024*1024;

trait Device {
    fn read8(&mut self, addr: Address) -> u8;
    fn write8(&mut self, addr: Address, data: u8);
}

struct Memory{
    data: Vec<u8>
}

impl Memory {
    fn new() -> Self {
        Self { data: vec![0; MEMORY_SIZE] }
    }
}

impl Device for Memory {
    fn read8(&mut self, addr: Address) -> u8 {
        if addr.0 as usize >= MEMORY_SIZE {
            panic!("invalid memory read: Out of Bounds")
        }
        return self.data[addr.0 as usize]
    }
    fn write8(&mut self, addr: Address, data: u8) {
        if addr.0 as usize >= MEMORY_SIZE {
            panic!("invalid memory write: Out of Bounds")
        }
        self.data[addr.0 as usize] = data
    }
}

struct MappedDevice(Address, Address, Box<dyn Device>);
struct Bus {
    devices: Vec<MappedDevice>
}

impl Bus {
    fn new() -> Self {
        Bus {devices: Vec::new()}
    }
    fn get_device(&self, addr: Address) -> (&dyn Device, Address) {
        for d in &self.devices {
            if ( d.0.0 <= addr.0 ) && (d.1.0 >= addr.0 ){
                return (d.2.as_ref(), Address(addr.0 - d.0.0));
            }
        }
        panic!("device not found with address {:?}", addr);
    }
    fn get_device_mut(&mut self, addr: Address) -> (&mut dyn Device, Address) {
        for d in &mut self.devices {
            if ( d.0.0 <= addr.0 ) && (d.1.0 >= addr.0 ){
                return (d.2.as_mut(), Address(addr.0 - d.0.0));
            }
        }
        panic!("device not found with address {:?}", addr);
    }
    fn add_device(&mut self, m_device: MappedDevice) {
        self.devices.push(m_device);
    }
    fn read8(&mut self, addr: Address) -> u8 {
        let (device, offset) = self.get_device_mut(addr);
        device.read8(offset)
    }
    fn write8(&mut self, addr: Address, data: u8) {
        let device = self.get_device_mut(addr);
        device.0.write8(device.1, data);
    }
    fn read16(&mut self, addr: Address) -> u16 {
        let b1 = self.read8(addr) as u16;
        let b2 = self.read8(addr + Address(1)) as u16;
        b1 | (b2 << 8)
    }
    fn read32(&mut self, addr: Address) -> u32 {
        let b1 = self.read8(addr) as u32;
        let b2 = self.read8(addr + Address(1)) as u32;
        let b3 = self.read8(addr + Address(2)) as u32;
        let b4 = self.read8(addr + Address(3)) as u32;
        b1 | (b2 << 8) | (b3 << 16) | (b4 << 24)
    }
    fn write16(&mut self, addr: Address, value: u16) {
        self.write8(addr, (value & 0xFF) as u8);
        self.write8(addr + Address(1), (value >> 8) as u8);
    }
    fn write32(&mut self, addr: Address, value: u32) {
        self.write8(addr, (value & 0xFF) as u8);
        self.write8(addr + Address(1), ((value >> 8) & 0xFF) as u8);
        self.write8(addr + Address(2), ((value >> 16) & 0xFF) as u8);
        self.write8(addr + Address(3), ((value >> 24) & 0xFF) as u8);
    }
}

struct UART {
    rx: VecDeque<u8>,
}

impl UART {
    const DATA: u32 = 0;
    const STATUS: u32 = 4;
    const CONTROL: u32 = 8;

    fn new() -> Self {
        Self {
            rx: VecDeque::new(),
        }
    }

    // Host injects a received byte.
    pub fn receive_byte(&mut self, byte: u8) {
        self.rx.push_back(byte);
    }
}

impl Device for UART {
    fn read8(&mut self, offset: Address) -> u8 {
        match offset.0 {
            Self::DATA => {
                self.rx.pop_front().unwrap_or(0)
            }

            Self::STATUS => {
                if self.rx.is_empty() {
                    0
                } else {
                    1
                }
            }

            Self::CONTROL => 0,

            _ => panic!("invalid UART register"),
        }
    }

    fn write8(&mut self, offset: Address, value: u8) {
        match offset.0 {
            Self::DATA => {
                print!("{}", value as char);
            }

            Self::STATUS => {}

            Self::CONTROL => {}

            _ => panic!("invalid UART register"),
        }
    }
}

use cpu::{RegisterIndex, Immediate, RawInstruction, Opcode, Funct3, Funct7, INSTRUCTION, Destination, Source1, Source2};

fn init_vm() -> Machine {
    let cpu = CPU::new();
    let mem = Memory::new();
    let mut bus = Bus::new();
    let m_dev = MappedDevice(Address(0), Address(0x0000ffff), Box::new(mem));
    bus.add_device(m_dev);
    Machine { cpu: cpu, bus: bus }
}

struct Machine{
    cpu: CPU,
    bus: Bus
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
    for i in 0..32 {
        println!("x{:02} = {}", i, cpu.registers[i]);
    }
}


fn main() {
    let mut uart = UART::new();

    println!("UART Direct Test");

    uart.receive_byte(b'H');
    uart.receive_byte(b'i');

    println!("Status = {}", uart.read8(Address(UART::STATUS)));
    println!("{}", uart.read8(Address(UART::DATA)) as char);

    println!("Status = {}", uart.read8(Address(UART::STATUS)));
    println!("{}", uart.read8(Address(UART::DATA)) as char);

    println!("Status = {}", uart.read8(Address(UART::STATUS)));

    uart.write8(Address(UART::DATA), b'!');
    println!();
}