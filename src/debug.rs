extern crate minifb;

use std::time::{Duration, Instant};
use self::minifb::{WindowOptions, Key, MouseMode};
use memory;
use window;

pub struct DebugScreen {
    pub window: minifb::Window,
    pub scroll: u16,
    pub buffer: Vec<u32>,
}

impl DebugScreen {
    pub fn new(width: usize, height: usize) -> DebugScreen {
        DebugScreen {
            buffer: vec![0; width*height],
            scroll: 0xFFFF,
            window: minifb::Window::new("rustboy debug",
                                        width,
                                        height,
                                        WindowOptions {
                                            borderless: true,
                                            scale: minifb::Scale::X4,
                                            ..Default::default()
                                        })
                .unwrap(),
        }
    }
}

impl window::Drawable for DebugScreen {
    fn update(&mut self, memory: &memory::Memory) {
        if self.window.is_open() {
            self.window.get_scroll_wheel().map(|scroll| {
                let width = self.window.get_size().0 as u16 / 4;
                self.scroll = self.scroll.wrapping_sub(width * scroll.1 as u16);
            });

            self.window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                let width = self.window.get_size().0 as u16 / 4;
                let x = mouse.0 as u16;
                let y = mouse.1 as u16;
                let offset = self.scroll - (y * width + x);
                let s = format!("0x{:0>4X}: {:0>4X}: {:0>2X}",
                                self.scroll,
                                offset,
                                memory[offset]);
                self.window.set_title(&s);
            });
        }
    }

    fn draw(&mut self, buffer: &memory::Memory) {
        let mut count: u16 = self.scroll;
        for i in self.buffer.iter_mut() {
            let gray = buffer[count] as u32;
            *i = gray << 16 | gray << 8 | gray;
            count -= 1;
        }
        self.window.update_with_buffer(&self.buffer);
    }
}
