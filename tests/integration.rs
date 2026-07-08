use risc_v::assembler;
use risc_v::cpu::{Address, PrivilegeMode};
use risc_v::machine::init_vm;
use risc_v::utils;




#[test]
fn machine_init_and_cpu_step() {
    let mut machine = init_vm();

    let instr = assembler::assemble_addi(1, 0, 5);
    machine.bus.write32(Address(0), instr);
    machine.cpu.step(&mut machine.bus);

    assert_eq!(machine.cpu.registers[1], 5);
    assert_eq!(machine.cpu.pc.0, 4);
}

#[test]
fn utils_dump_does_not_panic() {
    let machine = init_vm();
    utils::dump(&machine.cpu);
}

use risc_v::cpu::INTERRUPT;
use risc_v::platform::RAM_BASE;

#[test]
fn test_csr_instructions() {
    let mut vm = init_vm();
    vm.cpu.pc = Address(RAM_BASE.0);

    // Initial CPU state
    vm.cpu.registers[1] = 0xDEADBEEF; // Source register
    vm.cpu.registers[2] = 0x000000FF; // Another source register
    vm.cpu.csr_file.write(risc_v::cpu::cpu::MEPC, 0x12345678); // Put some data in MEPC

    // --- PROGRAM ---
    // 0x00: CSRRW x3, x1, MEPC (0x341) -> Read MEPC into x3, write x1 into MEPC
    // Instruction: 0x341091f3 (funct3=001, rs1=1, rd=3, csr=0x341)
    vm.bus.write32(Address(RAM_BASE.0), 0x341091f3);

    // 0x04: CSRRS x4, x2, MEPC (0x341) -> Read MEPC into x4, set bits from x2 into MEPC
    // Instruction: 0x34112273 (funct3=010, rs1=2, rd=4, csr=0x341)
    vm.bus.write32(Address(RAM_BASE.0 + 4), 0x34112273);

    // Step 1: CSRRW
    vm.step();
    assert_eq!(vm.cpu.registers[3], 0x12345678, "CSRRW should load the old CSR value into rd");
    assert_eq!(vm.cpu.csr_file.read(risc_v::cpu::cpu::MEPC), 0xDEADBEEF, "CSRRW should write rs1 into the CSR");

    // Step 2: CSRRS
    vm.step();
    assert_eq!(vm.cpu.registers[4], 0xDEADBEEF, "CSRRS should load the old CSR value into rd");
    
    // MEPC should now be 0xDEADBEEF | 0x000000FF = 0xDEADBEFF
    assert_eq!(vm.cpu.csr_file.read(risc_v::cpu::cpu::MEPC), 0xDEADBEFF, "CSRRS should set the bits specified in rs1");
}

#[test]
fn test_nested_interrupt_protection() {
    let mut vm = init_vm();

    // Setup: Enable interrupts and set trap vector
    vm.cpu.csr_file.write(risc_v::cpu::cpu::MTVEC, RAM_BASE.0 + 0x100);
    vm.cpu.csr_file.set_mie(true);
    vm.cpu.pc = Address(RAM_BASE.0);

    // Program: just a bunch of NOPs (ADDI x0, x0, 0) -> 0x00000013
    vm.bus.write32(Address(RAM_BASE.0), 0x00000013);
    vm.bus.write32(Address(RAM_BASE.0 + 4), 0x00000013);
    
    // Handler: Also a NOP
    vm.bus.write32(Address(RAM_BASE.0 + 0x100), 0x00000013);

    // Step 1: Fire an interrupt
    vm.int_controller.add_interrupt(INTERRUPT::TIMER);
    vm.step(); // This will execute 0x00 then immediately trap to 0x100
    
    assert_eq!(vm.cpu.pc.0, RAM_BASE.0 + 0x100);
    assert_eq!(vm.cpu.csr_file.mie_enabled(), false, "MIE should be false inside handler");

    // Step 2: Fire ANOTHER interrupt while we are inside the handler
    vm.int_controller.add_interrupt(INTERRUPT::KEYBOARD);
    vm.step(); // This executes the handler's NOP

    // The CPU should NOT have trapped again. It should just advance to 0x104
    assert_eq!(vm.cpu.pc.0, RAM_BASE.0 + 0x104, "CPU should ignore interrupts when MIE is false");
    
    // MEPC should remain untouched (still pointing to the original instruction at 0x04)
    assert_eq!(vm.cpu.csr_file.read(risc_v::cpu::cpu::MEPC), RAM_BASE.0 + 4);
}

#[test]
fn test_ecall_from_user_mode() {
    let mut vm = init_vm();

    // 1. Setup the Trap Handler Address
    let trap_handler_addr = RAM_BASE.0 + 0x100;
    vm.cpu.csr_file.write(risc_v::cpu::cpu::MTVEC, trap_handler_addr);

    // 2. Drop the CPU into User Mode
    vm.cpu.mode = PrivilegeMode::User;
    vm.cpu.pc = Address(RAM_BASE.0);

    // 3. Write an ECALL instruction (opcode: 0b1110011, funct3: 0, csr: 0) -> 0x00000073
    vm.bus.write32(Address(RAM_BASE.0), 0x00000073);

    // Step the CPU!
    vm.step();

    // 4. Verify the CPU trapped securely
    assert_eq!(vm.cpu.mode, PrivilegeMode::Machine, "CPU must escalate to Machine Mode on trap");
    assert_eq!(vm.cpu.pc.0, trap_handler_addr, "CPU must jump to MTVEC");
    assert_eq!(vm.cpu.csr_file.read(risc_v::cpu::cpu::MCAUSE), 8, "MCAUSE must be 8 (U-mode ECALL)");
    assert_eq!(vm.cpu.csr_file.read(risc_v::cpu::cpu::MEPC), RAM_BASE.0, "MEPC must hold the ECALL address");
}
#[test]
fn test_user_mode_cannot_access_csrs() {
    let mut vm = init_vm();

    let trap_handler_addr = RAM_BASE.0 + 0x100;
    vm.cpu.csr_file.write(risc_v::cpu::cpu::MTVEC, trap_handler_addr);
    
    // Put secret data in MEPC
    vm.cpu.csr_file.write(risc_v::cpu::cpu::MEPC, 0xDEADBEEF); 

    // Drop CPU to User Mode
    vm.cpu.mode = PrivilegeMode::User;
    vm.cpu.pc = Address(RAM_BASE.0);

    // Write a CSRRW instruction attempting to read MEPC (0x341) into x3
    // Instruction: 0x341091f3
    vm.bus.write32(Address(RAM_BASE.0), 0x341091f3);

    // Step the CPU!
    vm.step();

    // Verify the CPU blocked it and trapped!
    assert_eq!(vm.cpu.mode, PrivilegeMode::Machine, "CPU must escalate to handle the exception");
    assert_eq!(vm.cpu.pc.0, trap_handler_addr, "CPU must jump to handler");
    assert_eq!(vm.cpu.csr_file.read(risc_v::cpu::cpu::MCAUSE), 2, "MCAUSE must be 2 (Illegal Instruction)");
    
    // Ensure the user register x3 was NOT populated with the secret data
    assert_eq!(vm.cpu.registers[3], 0, "User program should not have read the CSR");
}
