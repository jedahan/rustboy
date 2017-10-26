extern crate minifb;

use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use self::minifb::{Key, WindowOptions, MouseMode};
use std::thread::sleep;
use std::fmt;
use memory;
use window;

pub struct DebugScreen {
    pub window: minifb::Window,
    pub scroll: u16,
    pub offset: u16,
    pub width: usize,
    pub buffer: Vec<u32>,
    pub memory: Arc<RwLock<memory::Memory>>,
}

impl window::Drawable for DebugScreen {
    fn update(&mut self) {
        if self.window.is_open() {
            self.window.get_scroll_wheel().map(|scroll| {
                let amount = self.width.wrapping_mul(scroll.1 as usize);
                self.scroll = self.scroll.wrapping_add(amount as u16);
            });

            self.window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                let x = mouse.0 as u16;
                let y = mouse.1 as u16;
                self.offset = y.wrapping_mul(self.width as u16).wrapping_add(x);
            });
        }
    }

    fn draw(&mut self) {
        let offset = self.scroll.wrapping_sub(self.offset);
        let byte = { self.memory.read().unwrap()[offset] };
        let s = format!("0x{:0>4X}: {:0>4X}: {:0>2X}",
                        self.scroll,
                        offset,
                        byte);
        self.window.set_title(&s);


        let mut count = self.scroll;
        let memory = { self.memory.read().unwrap() };

        for i in &mut self.buffer {
            let gray = memory[count] as u32;
            *i = gray << 16 | gray << 8 | gray;
            count = count.wrapping_sub(1);
        }

        let _ = self.window.update_with_buffer(&self.buffer);
    }

    fn pause(&mut self) {
        let frame_duration = Duration::from_millis(16);
        let ms = Duration::from_millis(1);
        let mut previous_draw = Instant::now();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.update();

            let now = Instant::now();
            if now - previous_draw > frame_duration {
                self.draw();
                previous_draw = now;
            };
            sleep(ms);
        }
    }
}

impl fmt::Display for DebugScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "offset: {offset:0>4X}", offset=self.offset));
        Ok(())
    }
}

impl DebugScreen {
    pub fn new(width: usize, height: usize, memory: Arc<RwLock<memory::Memory>>) -> DebugScreen {
        DebugScreen {
            buffer: vec![0; width * height],
            memory: memory,
            scroll: 0xFFFF,
            offset: 0x0000,
            width: width,
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

    fn run(&mut self) {
        self.width = self.window.get_size().0 / 4;
    }

}

