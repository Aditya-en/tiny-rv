use crate::bus::{Bus, MappedDevice};
use crate::cpu::CPU;
use crate::devices::Memory;
use crate::interrupt::InterruptController;
use crate::platform::{RAM_BASE, RAM_SIZE};

pub struct Machine {
    pub cpu: CPU,
    pub bus: Bus,
    pub int_controller: InterruptController,
}

impl Machine {
    pub fn new() -> Self {
        let cpu = CPU::new();
        let mut bus = Bus::new();
        let int_controller = InterruptController::new();

        // RAM
        bus.add_device(MappedDevice(
            RAM_BASE,
            crate::cpu::Address(RAM_BASE.0 + RAM_SIZE - 1),
            Box::new(Memory::new()),
        ));

        Self {
            cpu,
            bus,
            int_controller,
        }
    }

    pub fn step(&mut self) {
        self.bus.tick_all(&mut self.int_controller);

        self.cpu.step(&mut self.bus);

        if let Some(interrupt) = self.int_controller.next_interrupt() {
            self.cpu.handle_interrupt(&mut self.bus, interrupt);
        }
    }
}

pub fn init_vm() -> Machine {
    Machine::new()
}