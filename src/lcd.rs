extern crate minifb;

use std::ops::Range;
use self::minifb::{Key, WindowOptions, MouseMode};
use std::time;
use std::fmt;
use window;
use memory;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct LcdScreen {
    scroll: u16,
    control: u8,
    offset: u16,
    width: usize,
    buffer: Vec<u32>,
    memory: Arc<RwLock<memory::Memory>>, // TODO: switch to Arc<Mutex<memory::Memory>>
    window: minifb::Window,
}

impl LcdScreen {
    pub fn new(width: usize, height: usize, memory: Arc<RwLock<memory::Memory>>) -> Self {
        LcdScreen {
            scroll: 0x0000,
            control: 0,
            width: width,
            offset: 0x0000,
            memory: memory,
            buffer: vec![0; width * height],
            window: minifb::Window::new("rustboy",
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

    // ported from http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings
    // because i am lazy and not sure how things work
    #[allow(dead_code)]
    pub fn enabled(&self) -> bool {
        self.control & 0b10000000 != 0
    }

    #[allow(dead_code)]
    pub fn window_display_enable(&self) -> bool {
        self.control & 0b00100000 != 0
    }

    #[allow(dead_code)]
    pub fn obj_display_enable(&self) -> bool {
        self.control & 0b00000010 != 0
    }

    #[allow(dead_code)]
    pub fn bg_display(&self) -> bool {
        self.control & 0b00000001 != 0
    }

    #[allow(dead_code)]
    pub fn window_tile_map_display_select(&self) -> Range<u16> {
        // give me a bit of self.control ...
        if self.control & 0b01000000 == 0 {
            0x9800..0x9BFF
        } else {
            0x9C00..0x9FFF
        }
    }

    #[allow(dead_code)]
    pub fn bg_and_window_tile_data_select(&self) -> Range<u16> {
        if self.control & 0b00010000 == 0 {
            0x8800..0x97FF
        } else {
            0x8000..0x8FFF
        }
    }

    #[allow(dead_code)]
    pub fn bg_tile_map_display_select(&self) -> Range<u16> {
        if self.control & 0b00001000 == 0 {
            0x9800..0x9BFF
        } else {
            0x9C00..0x9FFF
        }
    }

    #[allow(dead_code)]
    pub fn obj_size(&self) -> (u8, u8) {
        if self.control & 0b00000100 == 0 {
            (8, 8)
        } else {
            (8, 16)
        }
    }

}

impl window::Drawable for LcdScreen {
    fn update(&mut self) {
        if self.window.is_open() {
            self.control = { self.memory.read().unwrap()[0xFF40 as u16] };
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

        self.window.update_with_buffer(&self.buffer);
    }

    fn run(&mut self) {
        self.memory.write().unwrap()[0xFF40 as u16] |= 0b10000000;
    }

    fn pause(&mut self) {
        let target_frame_duration = time::Duration::from_millis(16);
        let mut frame_start_time = time::Instant::now();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.update();

            if frame_start_time.elapsed() >= target_frame_duration {
                frame_start_time = time::Instant::now();
                self.draw();
            } else {
                thread::sleep(target_frame_duration - frame_start_time.elapsed());
            }
        }
    }
}

impl fmt::Display for LcdScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //let memory = self.memory.read().unwrap();
        try!(writeln!(f, "control: {control:0>4X}", control=self.control));
        Ok(())
    }
}
