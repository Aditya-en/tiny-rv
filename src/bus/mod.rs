use crate::cpu::Address;
use crate::devices::Device;

pub struct MappedDevice(pub Address, pub Address, pub Box<dyn Device>);

pub struct Bus {
    devices: Vec<MappedDevice>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            devices: Vec::new(),
        }
    }

    fn get_device_mut(&mut self, addr: Address) -> (&mut dyn Device, Address) {
        for d in &mut self.devices {
            if (d.0.0 <= addr.0) && (d.1.0 >= addr.0) {
                return (d.2.as_mut(), Address(addr.0 - d.0.0));
            }
        }
        panic!("device not found with address {:?}", addr);
    }

    pub fn add_device(&mut self, m_device: MappedDevice) {
        self.devices.push(m_device);
    }

    pub fn read8(&mut self, addr: Address) -> u8 {
        let (device, offset) = self.get_device_mut(addr);
        device.read8(offset)
    }

    pub fn write8(&mut self, addr: Address, data: u8) {
        let device = self.get_device_mut(addr);
        device.0.write8(device.1, data);
    }

    pub fn read16(&mut self, addr: Address) -> u16 {
        let b1 = self.read8(addr) as u16;
        let b2 = self.read8(addr + Address(1)) as u16;
        b1 | (b2 << 8)
    }

    pub fn read32(&mut self, addr: Address) -> u32 {
        let b1 = self.read8(addr) as u32;
        let b2 = self.read8(addr + Address(1)) as u32;
        let b3 = self.read8(addr + Address(2)) as u32;
        let b4 = self.read8(addr + Address(3)) as u32;
        b1 | (b2 << 8) | (b3 << 16) | (b4 << 24)
    }

    pub fn write16(&mut self, addr: Address, value: u16) {
        self.write8(addr, (value & 0xFF) as u8);
        self.write8(addr + Address(1), (value >> 8) as u8);
    }

    pub fn write32(&mut self, addr: Address, value: u32) {
        self.write8(addr, (value & 0xFF) as u8);
        self.write8(addr + Address(1), ((value >> 8) & 0xFF) as u8);
        self.write8(addr + Address(2), ((value >> 16) & 0xFF) as u8);
        self.write8(addr + Address(3), ((value >> 24) & 0xFF) as u8);
    }
}
