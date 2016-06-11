extern crate minifb;

use std::time::{Duration, Instant};
use self::minifb::{WindowOptions, Key, MouseMode};
use memory;

pub struct Screen {
    window: minifb::Window,
    scroll: u16,
    buffer: Vec<u32>
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen {
            buffer: vec![0; width*height],
            scroll: 0xFFFF,
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

    pub fn debug(&mut self, memory: & memory::Memory) {
        let frame_duration = Duration::from_millis(16);
        let mut previous_draw = Instant::now();

        self.draw(memory);
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let now = Instant::now();
            if now - previous_draw > frame_duration {
                self.draw(memory);

                self.window.get_scroll_wheel().map(|scroll| {
                    let width = self.window.get_size().0 as u16 / 4;
                    self.scroll = self.scroll.wrapping_sub(width * scroll.1 as u16);
                });

                self.window.get_mouse_pos(MouseMode::Clamp).map(|mouse|{
                    let width = self.window.get_size().0 as u16 / 4;
                    let x = mouse.0 as u16;
                    let y = mouse.1 as u16;
                    let offset = self.scroll - (y * width + x);
                    println!("0x{:0>4X}: {:0>4X}: {:0>2X}", self.scroll, offset, memory[offset] );
                });
                previous_draw = now;
            }
        }
    }

    pub fn draw(&mut self, buffer: & memory::Memory) {
        let mut count: u16 = self.scroll;
        for i in self.buffer.iter_mut() {
            let gray = buffer[count] as u32;
            *i = gray << 16 | gray << 8 | gray;
            count -= 1;
        }
        self.window.update_with_buffer(&self.buffer);
    }
}
