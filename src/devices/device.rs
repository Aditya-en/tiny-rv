use std::any::Any;

use crate::cpu::Address;
use crate::interrupt::InterruptController;

pub trait Device {
    fn read8(&mut self, addr: Address) -> u8;
    fn write8(&mut self, addr: Address, data: u8);
    fn tick(&mut self, _int_controller: &mut InterruptController) {}
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
