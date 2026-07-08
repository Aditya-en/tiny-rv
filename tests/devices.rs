use risc_v::{bus::{Bus, MappedDevice}, cpu::Address, devices::{keyboard::{DATA, STATUS}, Timer, UART}, interrupt::InterruptController};
use risc_v::devices::Device;

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
