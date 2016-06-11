extern crate minifb;

use self::minifb::{WindowOptions, Key, MouseMode};
use memory;

use std::time::Duration;
use std::thread::sleep;

pub struct Screen {
    window: minifb::Window,
    buffer: Vec<u32>
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen {
            buffer: vec![0; width*height],
            window: minifb::Window::new(
                "rustboy",
                width,
                height,
                WindowOptions {
                    borderless: true,
                    scale: minifb::Scale::X4,
                    ..Default::default()
                }
            ).unwrap()
        }
    }

    pub fn debug(&self, memory: & memory::Memory) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window.get_mouse_pos(MouseMode::Clamp).map(|mouse|{
                let offset = ((mouse.1 as usize) * self.window.get_size().0) as u16 + mouse.0 as u16;
                println!("{:0>4X}: {:0>2X}", offset, memory[offset] );
            });

            sleep(Duration::new(1, 0));
        }
    }

    pub fn draw(&mut self, buffer: & memory::Memory) {
        let mut count: u16 = 0xFFFF;
        for i in self.buffer.iter_mut() {
            let gray = buffer[count] as u32;
            *i = gray << 16 | gray << 8 | gray;
            count -= 1;
        }
        self.window.update_with_buffer(&self.buffer);
    }
}
