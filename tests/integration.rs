use risc_v::assembler;
use risc_v::bus::{Bus, MappedDevice};
use risc_v::cpu::Address;
use risc_v::devices::{Device, Memory, Timer, UART};
use risc_v::machine::init_vm;
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
    assert_eq!(uart.read8(Address(UART::DATA)), 0);
    assert_eq!(uart.read8(Address(UART::STATUS)), 0);

    uart.receive_byte(b'H');
    assert_eq!(uart.read8(Address(UART::STATUS)), 1);
    assert_eq!(uart.read8(Address(UART::DATA)), b'H');
    assert_eq!(uart.read8(Address(UART::STATUS)), 0);
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
    assert_eq!(timer.read8(Address(0)), 0);

    timer.tick();
    assert_eq!(timer.read8(Address(0)), 1);

    timer.tick();
    assert_eq!(timer.read8(Address(0)), 2);
}

#[test]
fn timer_tick_overflow() {
    let mut timer = Timer::new();
    timer.write8(Address(0), 0xFF);
    timer.write8(Address(1), 0xFF);
    timer.write8(Address(2), 0xFF);
    timer.write8(Address(3), 0xFF);

    timer.tick();
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
