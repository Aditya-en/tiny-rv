use crate::cpu::Address;
use crate::interrupt::InterruptController;

pub trait Device {
    fn read8(&mut self, addr: Address) -> u8;
    fn write8(&mut self, addr: Address, data: u8);
    fn tick(&mut self, int_controller: &mut InterruptController) {}
    fn get_data(&self) -> Option<&[u8]> {
        None
    }
}
