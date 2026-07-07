use risc_v::assembler;
use risc_v::bus::{Bus, MappedDevice};
use risc_v::cpu::Address;
use risc_v::devices::{Device, Memory, Timer, UART};
use risc_v::interrupt::InterruptController;
use risc_v::machine::init_vm;
use risc_v::platform::uart_registers::*;
use risc_v::utils;

#[test]
fn assembler_r_type_add() {
    let encoded = assembler::assemble_add(1, 2, 3);
    let expected = 0b0000000_00011_00010_000_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_i_type_addi() {
    let encoded = assembler::assemble_addi(1, 2, 0x123);

    assert_eq!(encoded & 0x7F, 0b0010011);
    assert_eq!((encoded >> 7) & 0x1F, 1);
    assert_eq!((encoded >> 15) & 0x1F, 2);
    assert_eq!((encoded >> 20) & 0xFFF, 0x123);
}

#[test]
fn assembler_i_type_srai() {
    let encoded = assembler::assemble_srai(1, 2, 3);
    let expected = 0b0100000_00011_00010_101_00001_0010011;
    assert_eq!(encoded, expected);
}

#[test]
fn bus_memory_read_write() {
    let mut bus = Bus::new();
    let mem = Memory::new();
    let device = MappedDevice(Address(0), Address(0x0000ffff), Box::new(mem));
    bus.add_device(device);

    bus.write8(Address(0x10), 0x42);
    assert_eq!(bus.read8(Address(0x10)), 0x42);
    assert_eq!(bus.read16(Address(0x10)), 0x42);

    bus.write16(Address(0x20), 0xBEEF);
    assert_eq!(bus.read16(Address(0x20)), 0xBEEF);
    assert_eq!(bus.read32(Address(0x20)), 0x0000_BEEF);
}

#[test]
fn uart_device_io() {
    let mut uart = UART::new();
    assert_eq!(uart.read8(Address(DATA)), 0);
    assert_eq!(uart.read8(Address(STATUS)), 0);

    uart.receive_byte(b'H');
    assert_eq!(uart.read8(Address(STATUS)), 1);
    assert_eq!(uart.read8(Address(DATA)), b'H');
    assert_eq!(uart.read8(Address(STATUS)), 0);
}

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

#[test]
fn timer_write_single_byte() {
    let mut timer = Timer::new();
    timer.write8(Address(0), 0x42);
    assert_eq!(timer.read8(Address(0)), 0x42);
}

#[test]
fn timer_write_multiple_bytes() {
    let mut timer = Timer::new();
    timer.write8(Address(0), 0x12);
    timer.write8(Address(1), 0x34);
    timer.write8(Address(2), 0x56);
    timer.write8(Address(3), 0x78);

    assert_eq!(timer.read8(Address(0)), 0x12);
    assert_eq!(timer.read8(Address(1)), 0x34);
    assert_eq!(timer.read8(Address(2)), 0x56);
    assert_eq!(timer.read8(Address(3)), 0x78);
}

#[test]
fn timer_tick_increments_counter() {
    let mut timer = Timer::new();
    let mut int_controller = InterruptController::new();
    assert_eq!(timer.read8(Address(0)), 0);

    timer.tick(&mut int_controller);
    assert_eq!(timer.read8(Address(0)), 1);

    timer.tick(&mut int_controller);
    assert_eq!(timer.read8(Address(0)), 2);
}

#[test]
fn timer_tick_overflow() {
    let mut timer = Timer::new();
    let mut int_controller = InterruptController::new();
    timer.write8(Address(0), 0xFF);
    timer.write8(Address(1), 0xFF);
    timer.write8(Address(2), 0xFF);
    timer.write8(Address(3), 0xFF);

    timer.tick(&mut int_controller);
    assert_eq!(timer.read8(Address(0)), 0);
    assert_eq!(timer.read8(Address(1)), 0);
    assert_eq!(timer.read8(Address(2)), 0);
    assert_eq!(timer.read8(Address(3)), 0);
}

#[test]
fn timer_partial_write() {
    let mut timer = Timer::new();
    timer.write8(Address(0), 0xAA);
    timer.write8(Address(1), 0xBB);

    timer.write8(Address(0), 0x11);
    assert_eq!(timer.read8(Address(0)), 0x11);
    assert_eq!(timer.read8(Address(1)), 0xBB);
}

#[test]
fn timer_via_bus() {
    let mut bus = Bus::new();
    let timer = Timer::new();
    let device = MappedDevice(Address(0x80000000), Address(0x80000003), Box::new(timer));
    bus.add_device(device);

    bus.write8(Address(0x80000000), 0x55);
    assert_eq!(bus.read8(Address(0x80000000)), 0x55);

    bus.write8(Address(0x80000001), 0xAA);
    assert_eq!(bus.read8(Address(0x80000001)), 0xAA);
}

use risc_v::cpu::INTERRUPT;
use risc_v::platform::RAM_BASE;

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
fn assembler_r_type_mul() {
    let encoded = assembler::assemble_mul(1, 2, 3);
    let expected = 0b0000001_00011_00010_000_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_mulh() {
    let encoded = assembler::assemble_mulh(1, 2, 3);
    let expected = 0b0000001_00011_00010_001_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_mulhsu() {
    let encoded = assembler::assemble_mulhsu(1, 2, 3);
    let expected = 0b0000001_00011_00010_010_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_mulhu() {
    let encoded = assembler::assemble_mulhu(1, 2, 3);
    let expected = 0b0000001_00011_00010_011_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_div() {
    let encoded = assembler::assemble_div(1, 2, 3);
    let expected = 0b0000001_00011_00010_100_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_divu() {
    let encoded = assembler::assemble_divu(1, 2, 3);
    let expected = 0b0000001_00011_00010_101_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_rem() {
    let encoded = assembler::assemble_rem(1, 2, 3);
    let expected = 0b0000001_00011_00010_110_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_remu() {
    let encoded = assembler::assemble_remu(1, 2, 3);
    let expected = 0b0000001_00011_00010_111_00001_0110011;
    assert_eq!(encoded, expected);
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
