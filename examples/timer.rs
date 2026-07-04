use risc_v::devices::Timer;
use risc_v::cpu::Address;
use risc_v::devices::Device;

fn main() {
    let mut timer = Timer::new();

    // Tick the timer a few times
    for _ in 0..5 {
        timer.tick();
    }

    // Read the 32-bit counter as four bytes
    for i in 0..4 {
        let b = timer.read8(Address(i));
        println!("timer byte {} = {}", i, b);
    }
}
