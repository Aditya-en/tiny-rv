use minifb::{Key, KeyRepeat, Window, WindowOptions};

use risc_v::devices::keyboard::Keyboard;
use risc_v::machine::init_vm;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut vm = init_vm();

    let mut window = Window::new(
        "Keyboard Test",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let framebuffer = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Poll every key currently pressed this frame.
        for key in window.get_keys_pressed(KeyRepeat::No) {
            if let Some(kbd) = vm.bus.get_device_mut::<Keyboard>() {

                if let Some(byte) = key_to_ascii(key) {
                    kbd.push_key(byte);

                    println!(
                        "Pressed: {:?} '{}' (0x{:02X})",
                        key,
                        byte as char,
                        byte
                    );
                } else {
                    println!("Pressed: {:?}", key);
                }
            }
        }

        window
            .update_with_buffer(&framebuffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn key_to_ascii(key: Key) -> Option<u8> {
    use Key::*;

    Some(match key {
        A => b'A',
        B => b'B',
        C => b'C',
        D => b'D',
        E => b'E',
        F => b'F',
        G => b'G',
        H => b'H',
        I => b'I',
        J => b'J',
        K => b'K',
        L => b'L',
        M => b'M',
        N => b'N',
        O => b'O',
        P => b'P',
        Q => b'Q',
        R => b'R',
        S => b'S',
        T => b'T',
        U => b'U',
        V => b'V',
        W => b'W',
        X => b'X',
        Y => b'Y',
        Z => b'Z',

        Key0 => b'0',
        Key1 => b'1',
        Key2 => b'2',
        Key3 => b'3',
        Key4 => b'4',
        Key5 => b'5',
        Key6 => b'6',
        Key7 => b'7',
        Key8 => b'8',
        Key9 => b'9',

        Space => b' ',
        Enter => b'\n',
        Backspace => 8,
        Tab => b'\t',

        _ => return None,
    })
}