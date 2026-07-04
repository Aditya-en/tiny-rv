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

    // Set the base address where our handlers will live
    vm.cpu.interrupt_base = Address(RAM_BASE.0 + 0x100);

    // --- MAIN PROGRAM (at RAM_BASE) ---
    // 0x00: ADDI x4, x0, 1 (x4 = 1) -> 0x00100213
    vm.bus.write32(Address(RAM_BASE.0), 0x00100213);
    // 0x04: JAL x0, -4     (Infinite loop back to 0x00) -> 0xffdff06f
    vm.bus.write32(Address(RAM_BASE.0 + 4), 0xffdff06f);

    // --- INTERRUPT HANDLER (at RAM_BASE + 0x100) ---
    // We assume TIMER interrupt, which goes to interrupt_base + 0x00
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
    assert_eq!(vm.cpu.interrupt_enabled, false);
    assert_eq!(vm.cpu.mepc, RAM_BASE.0); // It saved the loop target

    // Step 4: Execute the handler's ADDI x5, x5, 10
    vm.step();
    assert_eq!(vm.cpu.registers[5], 10); // Handler logic worked!

    // Step 5: Execute MRET
    vm.step();

    // CPU should be back in the main program, and interrupts re-enabled
    assert_eq!(vm.cpu.pc.0, RAM_BASE.0);
    assert_eq!(vm.cpu.interrupt_enabled, true);
}