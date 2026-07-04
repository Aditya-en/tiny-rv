// src/devices/screen.rs
use super::device::Device;
use crate::cpu::{Address};
use crate::interrupt::InterruptController;

pub const SCREEN_WIDTH: usize = 320;
pub const SCREEN_HEIGHT: usize = 240;
pub const BYTES_PER_PIXEL: usize = 4; // 32-bit RGBA
pub const FRAMEBUFFER_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * BYTES_PER_PIXEL;

// Define where the registers live relative to the device's base address
pub const BUFFER_START: u32 = 0x0;
pub const BUFFER_END: u32 = FRAMEBUFFER_SIZE as u32;
pub const CONTROL_REG: u32 = BUFFER_END; // Write 1 to request swap
pub const STATUS_REG: u32 = BUFFER_END + 4; // Read 1 if swap is pending

pub struct Screen {
    // The front buffer is public so your main.rs host loop can read it and render to your monitor
    pub front_buffer: Vec<u8>, 
    back_buffer: Vec<u8>,
    
    swap_requested: bool,
    cycles: u32,
    ticks_per_frame: u32, 
}

impl Screen {
    pub fn new(cpu_hz: u32, target_fps: u32) -> Self {
        Self {
            front_buffer: vec![0; FRAMEBUFFER_SIZE],
            back_buffer: vec![0; FRAMEBUFFER_SIZE],
            swap_requested: false,
            cycles: 0,
            ticks_per_frame: cpu_hz / target_fps,
        }
    }
}

impl Device for Screen {
    fn read8(&mut self, offset: Address) -> u8 {
        let addr = offset.0;

        if addr < BUFFER_END {
            self.back_buffer[addr as usize]
        } else if addr == STATUS_REG {
            self.swap_requested as u8
        } else {
            // Unmapped or control register (write-only) reads as 0
            0 
        }
    }

    fn write8(&mut self, offset: Address, value: u8) {
        let addr = offset.0;

        if addr < BUFFER_END {
            // CPU writes pixel data to the BACK buffer
            self.back_buffer[addr as usize] = value;
        } else if addr == CONTROL_REG {
            // Guest OS writes 1 to request a buffer swap at the next VSYNC
            if value == 1 {
                self.swap_requested = true;
            }
        }
    }
    fn get_data(&self) -> Option<&[u8]> {
        Some(&self.front_buffer)
    }

    fn tick(&mut self, _int_controller: &mut InterruptController) {
        self.cycles = self.cycles.wrapping_add(1);

        // Check if it is time for a screen refresh (VSYNC)
        if self.cycles >= self.ticks_per_frame {
            self.cycles = 0; // Reset cycle counter

            // If the guest OS finished rendering and requested a swap, do it now
            if self.swap_requested {
                // Copy back buffer to front buffer
                self.front_buffer.copy_from_slice(&self.back_buffer);
                self.swap_requested = false;
                
                // Optional: Fire an interrupt to tell the OS the swap is complete 
                // so it can start drawing the next frame immediately.
                // int_controller.add_interrupt(INTERRUPT::SOFTWARE); // Or a dedicated GPU interrupt
            }
        }
    }
}