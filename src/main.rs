use minifb::{Window, WindowOptions};
use risc_v::machine::{init_vm, Machine};
use risc_v::platform::RAM_BASE;
use std::fs::File;
use std::io::Read;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;


pub fn load_binary(machine: &mut Machine, filename: &str, base_addr: u32) {
    let mut file = File::open(filename).expect("Failed to open binary");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read binary");

    for (i, byte) in buffer.iter().enumerate() {
        machine.bus.write8(risc_v::cpu::Address(base_addr + i as u32), *byte);
    }
    
    // Ensure the CPU starts at the beginning of the program
    machine.cpu.pc = risc_v::cpu::Address(base_addr); 
}


fn main() {
    let mut vm = init_vm();
    
    // Load your compiled C program into RAM
    load_binary(&mut vm, "program.bin", RAM_BASE.0);

    // Create a physical window on your real computer
    let mut window = Window::new(
        "RISC-V Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    // We need a u32 buffer for minifb (ARGB format)
    let mut display_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        
        // Execute a batch of CPU instructions per frame
        // (e.g., 10,000 instructions per visual frame)
        for _ in 0..10_000 {
            vm.step();
        }

        // Read the front buffer from your custom Screen device
        // You'll need to add a getter to your Bus to access the Screen's front_buffer safely
        let screen_device = vm.bus.get_screen_pixels(); // Implement this helper!
        let raw_pixels = screen_device;

        // Convert the emulator's RGBA byte array into minifb's ARGB u32 array
        for i in 0..(WIDTH * HEIGHT) {
            let r = raw_pixels[i * 4 + 0] as u32;
            let g = raw_pixels[i * 4 + 1] as u32;
            let b = raw_pixels[i * 4 + 2] as u32;
            
            display_buffer[i] = (r << 16) | (g << 8) | b;
        }

        // Push the pixels to the screen
        window.update_with_buffer(&display_buffer, WIDTH, HEIGHT).unwrap();
    }
}