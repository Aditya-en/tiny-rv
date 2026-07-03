use crate::cpu::{CPU, Address};
use crate::bus::{Bus, MappedDevice};
use crate::devices::Memory;

pub struct Machine {
    pub cpu: CPU,
    pub bus: Bus,
}

impl Machine {
    pub fn new() -> Self {
        let cpu = CPU::new();
        let mem = Memory::new();
        let mut bus = Bus::new();
        let m_dev = MappedDevice(Address(0), Address(0x0000ffff), Box::new(mem));
        bus.add_device(m_dev);
        Machine { cpu, bus }
    }
}

pub fn init_vm() -> Machine {
    Machine::new()
}
