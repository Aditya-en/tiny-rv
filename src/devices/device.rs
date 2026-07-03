use crate::cpu::Address;

pub trait Device {
    fn read8(&mut self, addr: Address) -> u8;
    fn write8(&mut self, addr: Address, data: u8);
    fn tick(&mut self) {}
}
