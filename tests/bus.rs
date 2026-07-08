use risc_v::{bus::{Bus, MappedDevice}, cpu::Address, devices::Memory};


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
