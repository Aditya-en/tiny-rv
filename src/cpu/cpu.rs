use super::types::*;
use crate::bus::Bus;
use crate::cpu::types::INTERRUPT;

#[derive(Debug)]
pub struct CPU {
    pub registers: [u32; 32],
    pub pc: Address,
    pub csr_file: CSRFile,
    pub mode: PrivilegeMode,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: Address(0x0),
            csr_file: CSRFile::new(),
            mode: PrivilegeMode::Machine,
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let raw_inst = self.fetch(bus);
        let inst = Self::decode(raw_inst);
        self.execute(inst, bus);
    }

    pub fn handle_interrupt(&mut self, interrupt_type: INTERRUPT) {
        if !self.csr_file.mie_enabled() {
            return;
        }

        let cause = match interrupt_type {
            INTERRUPT::TIMER    => 0x80000007,
            INTERRUPT::UART     => 0x8000000B,
            INTERRUPT::SOFTWARE => 0x80000003,
            INTERRUPT::KEYBOARD => 0x8000000C,
        };

        self.enter_trap(cause, self.pc.0);
    }

    pub fn handle_exception(&mut self, cause: u32) {
        // Fetch already advanced the PC by 4. 
        // Synchronous exceptions must save the address of the FAULTING instruction.
        let faulting_pc = self.pc.0.wrapping_sub(4);
        
        self.enter_trap(cause, faulting_pc);
    }

    fn enter_trap(&mut self, cause: u32, epc: u32) {
        // 1. Save PC to MEPC
        self.csr_file.write(MEPC, epc);

        // 2. Set MCAUSE
        self.csr_file.write(MCAUSE, cause);

        // 3. Save current mode into MPP, and current MIE into MPIE
        let current_mode = self.mode as u32;
        self.csr_file.set_mpp(current_mode);
        
        let current_mie = self.csr_file.mie_enabled();
        self.csr_file.set_mpie(current_mie);

        // 4. Disable interrupts and escalate to Machine Mode
        self.csr_file.set_mie(false);
        self.mode = PrivilegeMode::Machine;

        // 5. Jump to Trap Vector Base
        self.pc = Address(self.csr_file.read(MTVEC));
    }
}



pub const MSTATUS: usize = 0x300;
pub const MISA: usize = 0x301;
pub const MEDELEG: usize = 0x302;
pub const MIDELEG: usize = 0x303;
pub const MIE: usize = 0x304;
pub const MTVEC: usize = 0x305;

pub const MSCRATCH: usize = 0x340;
pub const MEPC: usize = 0x341;
pub const MCAUSE: usize = 0x342;
pub const MTVAL: usize = 0x343;
pub const MIP: usize = 0x344;

pub const CYCLE: usize = 0xC00;
pub const TIME: usize = 0xC01;
pub const INSTRET: usize = 0xC02;
pub const MSTATUS_MIE: u32 = 1 << 3;
pub const MSTATUS_MPIE: u32 = 1 << 7;

pub const MSTATUS_MPP_SHIFT: u32 = 11;
pub const MSTATUS_MPP_MASK: u32 = 0b11 << MSTATUS_MPP_SHIFT;


#[derive(Debug)]
pub struct CSRFile {
    registers: [u32; 4096],
}

impl CSRFile {
    pub fn new() -> Self {
        Self { registers: [0; 4096] }
    }

    pub fn read(&self, index: usize) -> u32 {
        self.registers[index]
    }

    pub fn write(&mut self, index: usize, value: u32) {
        self.registers[index] = value;
    }

    pub fn mie_enabled(&self) -> bool {
        self.read(MSTATUS) & MSTATUS_MIE != 0
    }
    
    pub fn set_mie(&mut self, enabled: bool) {
        let mut status = self.read(MSTATUS);
    
        if enabled {
            status |= MSTATUS_MIE;
        } else {
            status &= !MSTATUS_MIE;
        }
    
        self.write(MSTATUS, status);
    }
    pub fn mpie(&self ) -> bool {
        self.read(MSTATUS) & MSTATUS_MPIE != 0
    }
    pub fn set_mpie(&mut self, enabled: bool) {
        let mut status = self.read(MSTATUS);
    
        if enabled {
            status |= MSTATUS_MPIE;
        } else {
            status &= !MSTATUS_MPIE;
        }
    
        self.write(MSTATUS, status);
    }
    pub fn mpp(&self) -> u32 {
        (self.read(MSTATUS) & MSTATUS_MPP_MASK) >> MSTATUS_MPP_SHIFT
    }
    pub fn set_mpp(&mut self, mode: u32) {
        let mut status = self.read(MSTATUS);
        status &= !MSTATUS_MPP_MASK;
        status |= (mode << MSTATUS_MPP_SHIFT) & MSTATUS_MPP_MASK;
        self.write(MSTATUS, status);
    }

}
