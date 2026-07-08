use risc_v::{assembler, cpu::{cpu::{MCAUSE, MEPC}, Address, PrivilegeMode, INTERRUPT}, machine::init_vm, platform::RAM_BASE};


#[test]
fn test_user_mode_cannot_execute_mret() {
    let mut vm = init_vm();

    let trap_handler_addr = RAM_BASE.0 + 0x100;
    vm.cpu.csr_file.write(risc_v::cpu::cpu::MTVEC, trap_handler_addr);

    // Drop CPU to User Mode
    vm.cpu.mode = PrivilegeMode::User;
    vm.cpu.pc = Address(RAM_BASE.0);

    // Write an MRET instruction (0x30200073)
    vm.bus.write32(Address(RAM_BASE.0), 0x30200073);

    // Step the CPU!
    vm.step();

    // Verify the CPU threw an Illegal Instruction exception
    assert_eq!(vm.cpu.mode, PrivilegeMode::Machine);
    assert_eq!(vm.cpu.pc.0, trap_handler_addr);
    assert_eq!(vm.cpu.csr_file.read(MCAUSE), 2, "MCAUSE must be 2 (Illegal Instruction)");
}

#[test]
fn test_mret_restores_privilege_mode() {
    let mut vm = init_vm();

    // CPU is in Machine mode by default
    assert_eq!(vm.cpu.mode, PrivilegeMode::Machine);

    // Setup the return state as if we just finished handling an interrupt for a User program
    let user_program_addr = RAM_BASE.0 + 0x400;
    vm.cpu.csr_file.write(MEPC, user_program_addr);
    
    // Set MPP (Machine Previous Privilege) to User (0)
    vm.cpu.csr_file.set_mpp(PrivilegeMode::User as u32);
    
    // Set MPIE to true (so interrupts turn back on when we return)
    vm.cpu.csr_file.set_mpie(true);

    vm.cpu.pc = Address(RAM_BASE.0);

    // Write an MRET instruction
    vm.bus.write32(Address(RAM_BASE.0), 0x30200073);

    // Step the CPU!
    vm.step();

    // Verify the CPU state was correctly restored
    assert_eq!(vm.cpu.mode, PrivilegeMode::User, "MRET must restore mode from MPP");
    assert_eq!(vm.cpu.pc.0, user_program_addr, "MRET must jump to MEPC");
    assert_eq!(vm.cpu.csr_file.mie_enabled(), true, "MRET must restore MIE from MPIE");
    assert_eq!(vm.cpu.csr_file.mpp(), 0, "MRET must set MPP back to User (0) per RISC-V spec");
}

#[test]
fn machine_mul() {
    let mut machine = init_vm();

    machine.cpu.registers[1] = 6;
    machine.cpu.registers[2] = 7;

    machine
        .bus
        .write32(Address(0), assembler::assemble_mul(3, 1, 2));

    machine.cpu.step(&mut machine.bus);

    assert_eq!(machine.cpu.registers[3], 42);
}

#[test]
fn machine_div() {
    let mut machine = init_vm();

    machine.cpu.registers[1] = 84;
    machine.cpu.registers[2] = 2;

    machine
        .bus
        .write32(Address(0), assembler::assemble_div(3, 1, 2));

    machine.cpu.step(&mut machine.bus);

    assert_eq!(machine.cpu.registers[3], 42);
}

#[test]
fn machine_divu() {
    let mut machine = init_vm();

    machine.cpu.registers[1] = 100;
    machine.cpu.registers[2] = 4;

    machine
        .bus
        .write32(Address(0), assembler::assemble_divu(3, 1, 2));

    machine.cpu.step(&mut machine.bus);

    assert_eq!(machine.cpu.registers[3], 25);
}

#[test]
fn machine_rem() {
    let mut machine = init_vm();

    machine.cpu.registers[1] = 100;
    machine.cpu.registers[2] = 6;

    machine
        .bus
        .write32(Address(0), assembler::assemble_rem(3, 1, 2));

    machine.cpu.step(&mut machine.bus);

    assert_eq!(machine.cpu.registers[3], 4);
}

#[test]
fn machine_remu() {
    let mut machine = init_vm();

    machine.cpu.registers[1] = 100;
    machine.cpu.registers[2] = 7;

    machine
        .bus
        .write32(Address(0), assembler::assemble_remu(3, 1, 2));

    machine.cpu.step(&mut machine.bus);

    assert_eq!(machine.cpu.registers[3], 2);
}

#[test]
fn machine_mulh() {
    let mut machine = init_vm();

    machine.cpu.registers[1] = 0xffff_fffe; // -2
    machine.cpu.registers[2] = 3;

    machine
        .bus
        .write32(Address(0), assembler::assemble_mulh(3, 1, 2));

    machine.cpu.step(&mut machine.bus);

    // (-2 * 3) = -6 = 0xfffffffffffffffA
    assert_eq!(machine.cpu.registers[3], 0xffff_ffff);
}
#[test]
fn machine_mulhu() {
    let mut machine = init_vm();

    machine.cpu.registers[1] = 0xffff_ffff;
    machine.cpu.registers[2] = 2;

    machine
        .bus
        .write32(Address(0), assembler::assemble_mulhu(3, 1, 2));

    machine.cpu.step(&mut machine.bus);

    // 0xffffffff * 2 = 0x1fffffffe
    assert_eq!(machine.cpu.registers[3], 1);
}

#[test]
fn machine_mulhsu() {
    let mut machine = init_vm();

    machine.cpu.registers[1] = 0xffff_fffe; // -2
    machine.cpu.registers[2] = 2;

    machine
        .bus
        .write32(Address(0), assembler::assemble_mulhsu(3, 1, 2));

    machine.cpu.step(&mut machine.bus);

    // (-2 * 2) = -4 -> high 32 bits = 0xffffffff
    assert_eq!(machine.cpu.registers[3], 0xffff_ffff);
}


#[test]
fn test_custom_interrupt_mret() {
    let mut vm = init_vm();

    // Set the base address where our handlers will live using MTVEC
    vm.cpu.csr_file.write(risc_v::cpu::cpu::MTVEC, RAM_BASE.0 + 0x100);
    
    // Enable interrupts globally via MSTATUS
    vm.cpu.csr_file.set_mie(true);

    // --- MAIN PROGRAM (at RAM_BASE) ---
    // 0x00: ADDI x4, x0, 1 (x4 = 1) -> 0x00100213
    vm.bus.write32(Address(RAM_BASE.0), 0x00100213);
    // 0x04: JAL x0, -4     (Infinite loop back to 0x00) -> 0xffdff06f
    vm.bus.write32(Address(RAM_BASE.0 + 4), 0xffdff06f);

    // --- INTERRUPT HANDLER (at RAM_BASE + 0x100) ---
    // 0x100: ADDI x5, x5, 10 (x5 += 10) -> 0x00a28293
    vm.bus.write32(Address(RAM_BASE.0 + 0x100), 0x00a28293);
    // 0x104: MRET -> 0x30200073
    vm.bus.write32(Address(RAM_BASE.0 + 0x104), 0x30200073);

    // Start CPU at RAM_BASE
    vm.cpu.pc = Address(RAM_BASE.0);

    // Step 1: Execute ADDI x4, x0, 1
    vm.step();
    assert_eq!(vm.cpu.registers[4], 1);
    assert_eq!(vm.cpu.pc.0, RAM_BASE.0 + 4);

    // Step 2: Trigger our custom interrupt mid-execution
    vm.int_controller.add_interrupt(INTERRUPT::TIMER);

    // Step 3: Call step. The infinite loop (JAL) will execute, 
    // but AFTER it executes, the machine checks for interrupts!
    vm.step(); 
    
    // The CPU should have jumped to the handler!
    assert_eq!(vm.cpu.pc.0, RAM_BASE.0 + 0x100);
    assert_eq!(vm.cpu.csr_file.mie_enabled(), false, "Interrupts should be disabled inside handler");
    assert_eq!(vm.cpu.csr_file.mpie(), true, "Previous interrupt state (true) should be saved to MPIE");
    assert_eq!(vm.cpu.csr_file.read(risc_v::cpu::cpu::MEPC), RAM_BASE.0, "MEPC should store the return address");

    // Step 4: Execute the handler's ADDI x5, x5, 10
    vm.step();
    assert_eq!(vm.cpu.registers[5], 10); // Handler logic worked!

    // Step 5: Execute MRET
    vm.step();

    // CPU should be back in the main program, and interrupts re-enabled
    assert_eq!(vm.cpu.pc.0, RAM_BASE.0);
    assert_eq!(vm.cpu.csr_file.mie_enabled(), true, "MRET should restore MIE from MPIE");
}
