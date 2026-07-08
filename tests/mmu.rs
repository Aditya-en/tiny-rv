use risc_v::cpu::{Address, PrivilegeMode};
use risc_v::cpu::cpu::{SATP, MCAUSE, MTVEC, MEPC};
use risc_v::machine::init_vm;
use risc_v::mmu::{
    EXC_INST_PAGE_FAULT,
    EXC_LOAD_PAGE_FAULT,
    EXC_STORE_PAGE_FAULT,
};

const ROOT_PT: u32 = 0x1000;
const LEAF_PT: u32 = 0x2000;
const PHYS_PAGE: u32 = 0x3000;

fn leaf(ppn: u32, r: bool, w: bool, x: bool, u: bool) -> u32 {
    (ppn << 10)
        | 1
        | ((r as u32) << 1)
        | ((w as u32) << 2)
        | ((x as u32) << 3)
        | ((u as u32) << 4)
}

fn setup_mapping(vm: &mut risc_v::machine::Machine, r: bool, w: bool, x: bool, u: bool) {
    let vpn1 = 0;
    let vpn0 = 1;

    // root[0] -> leaf table
    vm.bus.write32(
        Address(ROOT_PT + vpn1 * 4),
        leaf(LEAF_PT >> 12, false, false, false, false),
    );

    // leaf[1] -> physical page
    vm.bus.write32(
        Address(LEAF_PT + vpn0 * 4),
        leaf(PHYS_PAGE >> 12, r, w, x, u),
    );

    vm.cpu.mode = PrivilegeMode::User;

    vm.cpu
        .csr_file
        .write(SATP, (1 << 31) | (ROOT_PT >> 12));

    vm.cpu.csr_file.write(MTVEC, 0x5000);
}

#[test]
fn paging_disabled_identity_mapping() {
    let mut vm = init_vm();

    vm.cpu.mode = PrivilegeMode::User;
    vm.cpu.csr_file.write(SATP, 0);

    // ADDI x0, x0, 0 (NOP) — 0xdeadbeef's low bits happen to decode as JAL,
    // which would hijack pc and make this test meaningless.
    vm.bus.write32(Address(0x1000), 0x00000013);

    vm.cpu.pc = Address(0x1000);

    vm.step();

    assert_eq!(vm.cpu.pc.0, 0x1004);
}

#[test]
fn machine_mode_bypasses_mmu() {
    let mut vm = init_vm();

    vm.cpu.mode = PrivilegeMode::Machine;

    vm.cpu
        .csr_file
        .write(SATP, 1 << 31);

    vm.bus.write32(Address(0x400), 0x00000013);

    vm.cpu.pc = Address(0x400);

    vm.step();

    assert_eq!(vm.cpu.pc.0, 0x404);
}

#[test]
fn instruction_fetch_translation() {
    let mut vm = init_vm();

    setup_mapping(&mut vm, true, true, true, true);

    vm.bus.write32(Address(PHYS_PAGE), 0x00000013);

    vm.cpu.pc = Address(0x1000);

    vm.step();

    assert_eq!(vm.cpu.pc.0, 0x1004);
}

#[test]
fn instruction_page_fault() {
    let mut vm = init_vm();

    setup_mapping(&mut vm, true, true, false, true);

    vm.cpu.pc = Address(0x1000);

    vm.step();

    assert_eq!(
        vm.cpu.csr_file.read(MCAUSE),
        EXC_INST_PAGE_FAULT
    );
}

#[test]
fn invalid_root_pte_faults() {
    let mut vm = init_vm();

    vm.cpu.mode = PrivilegeMode::User;

    vm.cpu
        .csr_file
        .write(SATP, (1 << 31) | (ROOT_PT >> 12));

    vm.cpu.csr_file.write(MTVEC, 0x5000);

    vm.cpu.pc = Address(0x1000);

    vm.step();

    assert_eq!(
        vm.cpu.csr_file.read(MCAUSE),
        EXC_INST_PAGE_FAULT
    );
}

#[test]
fn user_cannot_access_supervisor_page() {
    let mut vm = init_vm();

    setup_mapping(&mut vm, true, true, true, false);

    vm.cpu.pc = Address(0x1000);

    vm.step();

    assert_eq!(
        vm.cpu.csr_file.read(MCAUSE),
        EXC_INST_PAGE_FAULT
    );
}

#[test]
fn supervisor_cannot_access_user_page() {
    let mut vm = init_vm();

    setup_mapping(&mut vm, true, true, true, true);

    vm.cpu.mode = PrivilegeMode::Supervisor;

    vm.cpu.pc = Address(0x1000);

    vm.step();

    assert_eq!(
        vm.cpu.csr_file.read(MCAUSE),
        EXC_INST_PAGE_FAULT
    );
}

#[test]
fn write_only_page_is_invalid() {
    let mut vm = init_vm();

    setup_mapping(&mut vm, false, true, false, true);

    vm.cpu.pc = Address(0x1000);

    vm.step();

    assert_eq!(
        vm.cpu.csr_file.read(MCAUSE),
        EXC_INST_PAGE_FAULT
    );
}

#[test]
fn lw_uses_translation() {
    let mut vm = init_vm();

    setup_mapping(&mut vm, true, true, true, true);

    vm.bus.write32(Address(PHYS_PAGE), 0x11223344);

    vm.cpu.registers[2] = 0x1000;

    vm.bus.write32(
        Address(PHYS_PAGE + 4),
        risc_v::assembler::assemble_lw(1, 2, 0),
    );

    vm.cpu.pc = Address(0x1004);

    vm.step();

    assert_eq!(vm.cpu.registers[1], 0x11223344);
}

#[test]
fn load_permission_fault() {
    let mut vm = init_vm();

    // Execute-only page: legal encoding (r=0,w=0,x=1), so fetch succeeds
    // and we isolate the load-permission check specifically.
    setup_mapping(&mut vm, false, false, true, true);

    vm.cpu.registers[2] = 0x1000;

    vm.bus.write32(
        Address(PHYS_PAGE),
        risc_v::assembler::assemble_lw(1, 2, 0),
    );

    vm.cpu.pc = Address(0x1000);

    vm.step();

    assert_eq!(
        vm.cpu.csr_file.read(MCAUSE),
        EXC_LOAD_PAGE_FAULT
    );
}

#[test]
fn store_permission_fault() {
    let mut vm = init_vm();

    setup_mapping(&mut vm, true, false, true, true);

    vm.cpu.registers[1] = 0x55;
    vm.cpu.registers[2] = 0x1000;

    vm.bus.write32(
        Address(PHYS_PAGE),
        risc_v::assembler::assemble_sw(2, 1, 0),
    );

    vm.cpu.pc = Address(0x1000);

    vm.step();

    assert_eq!(
        vm.cpu.csr_file.read(MCAUSE),
        EXC_STORE_PAGE_FAULT
    );
}

// ---------------------------------------------------------------
// New tests below
// ---------------------------------------------------------------

#[test]
fn mega_page_translation_succeeds() {
    // Root PTE directly has R/W/X set -> it's a 4MB leaf, not a pointer.
    // vaddr must have vpn0 == 0 for this to be legal.
    let mut vm = init_vm();

    // vpn1 = 0, vpn0 = 0 -> vaddr 0x0
    vm.bus.write32(
        Address(ROOT_PT),
        leaf(PHYS_PAGE >> 12, true, true, true, true),
    );

    vm.cpu.mode = PrivilegeMode::User;
    vm.cpu.csr_file.write(SATP, (1 << 31) | (ROOT_PT >> 12));
    vm.cpu.csr_file.write(MTVEC, 0x5000);

    vm.bus.write32(Address(PHYS_PAGE), 0x00000013); // NOP

    vm.cpu.pc = Address(0x0);

    vm.step();

    assert_eq!(vm.cpu.pc.0, 0x4);
}

#[test]
fn misaligned_mega_page_faults() {
    // Root PTE is a mega-page leaf (R/W/X set at root level), but the
    // virtual address has a nonzero vpn0 — illegal per Sv32, must fault.
    let mut vm = init_vm();

    vm.bus.write32(
        Address(ROOT_PT),
        leaf(PHYS_PAGE >> 12, true, true, true, true),
    );

    vm.cpu.mode = PrivilegeMode::User;
    vm.cpu.csr_file.write(SATP, (1 << 31) | (ROOT_PT >> 12));
    vm.cpu.csr_file.write(MTVEC, 0x5000);

    // vaddr 0x1000 -> vpn1 = 0, vpn0 = 1 (nonzero) with same root PTE
    vm.cpu.pc = Address(0x1000);

    vm.step();

    assert_eq!(
        vm.cpu.csr_file.read(MCAUSE),
        EXC_INST_PAGE_FAULT
    );
}

#[test]
fn satp_mode_bit_clear_bypasses_even_with_ppn_set() {
    // Mode bit (bit 31) is 0, so translation should be bypassed regardless
    // of whatever garbage is in the PPN field.
    let mut vm = init_vm();

    vm.cpu.mode = PrivilegeMode::User;
    vm.cpu.csr_file.write(SATP, ROOT_PT >> 12); // mode bit NOT set

    vm.bus.write32(Address(0x2000), 0x00000013); // NOP

    vm.cpu.pc = Address(0x2000);

    vm.step();

    assert_eq!(vm.cpu.pc.0, 0x2004);
}

#[test]
fn csrrw_from_user_mode_faults() {
    // CSR instructions are machine-mode only in this implementation;
    // executing one from User mode should raise an illegal-instruction
    // exception (cause 2) rather than silently mutating the CSR.
    let mut vm = init_vm();

    vm.cpu.mode = PrivilegeMode::User;
    vm.cpu.csr_file.write(SATP, 0); // paging off so fetch isn't blocked
    vm.cpu.csr_file.write(MTVEC, 0x5000);

    // CSRRW x1, x2, MTVEC (0x305)
    vm.bus.write32(Address(0x0), risc_v::assembler::assemble_i_type(
        0b1110011, 0b001, 1, 2, 0x305,
    ));

    vm.cpu.pc = Address(0x0);

    vm.step();

    assert_eq!(vm.cpu.csr_file.read(MCAUSE), 2);
    assert_eq!(vm.cpu.mode, PrivilegeMode::Machine); // trap escalates privilege
}

#[test]
fn ecall_cause_differs_by_privilege_mode() {
    // Machine mode ECALL -> cause 11
    let mut vm = init_vm();
    vm.cpu.mode = PrivilegeMode::Machine;
    vm.cpu.csr_file.write(SATP, 0);
    vm.cpu.csr_file.write(MTVEC, 0x5000);
    vm.bus.write32(Address(0x0), risc_v::assembler::assemble_ecall());
    vm.cpu.pc = Address(0x0);
    vm.step();
    assert_eq!(vm.cpu.csr_file.read(MCAUSE), 11);

    // User mode ECALL -> cause 8
    let mut vm2 = init_vm();
    vm2.cpu.mode = PrivilegeMode::User;
    vm2.cpu.csr_file.write(SATP, 0);
    vm2.cpu.csr_file.write(MTVEC, 0x5000);
    vm2.bus.write32(Address(0x0), risc_v::assembler::assemble_ecall());
    vm2.cpu.pc = Address(0x0);
    vm2.step();
    assert_eq!(vm2.cpu.csr_file.read(MCAUSE), 8);
}

#[test]
fn mret_restores_pc_and_privilege() {
    let mut vm = init_vm();

    vm.cpu.mode = PrivilegeMode::Machine;
    vm.cpu.csr_file.write(SATP, 0);

    // Simulate having trapped from User mode: MEPC set, MPP saved as User (0)
    vm.cpu.csr_file.write(MEPC, 0x2000);
    vm.cpu.csr_file.set_mpp(0);
    vm.cpu.csr_file.set_mpie(true);

    vm.bus.write32(Address(0x0), risc_v::assembler::assemble_mret());
    vm.cpu.pc = Address(0x0);

    vm.step();

    assert_eq!(vm.cpu.pc.0, 0x2000);
    assert_eq!(vm.cpu.mode, PrivilegeMode::User);
    assert!(vm.cpu.csr_file.mie_enabled());
}