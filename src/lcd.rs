extern crate minifb;

use std::ops::Range;
use self::minifb::WindowOptions;
use std::time::{Duration, Instant};
use std::fmt;
use window;
use memory;
use std::sync::{Arc, RwLock};
use std::thread::sleep;

pub struct LcdScreen {
    scroll: u16,
    control: u8,
    buffer: Vec<u32>,
    memory: Arc<RwLock<memory::Memory>>, // TODO: switch to Arc<Mutex<memory::Memory>>
    window: minifb::Window,
}

impl LcdScreen {
    pub fn new(width: usize, height: usize, memory: Arc<RwLock<memory::Memory>>) -> Self {
        LcdScreen {
            scroll: 0x0000,
            control: 0,
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
    pub fn enabled(&self) -> bool {
        self.control & 0b10000000 != 0
    }

    pub fn window_display_enable(&self) -> bool {
        self.control & 0b00100000 != 0
    }

    pub fn obj_display_enable(&self) -> bool {
        self.control & 0b00000010 != 0
    }

    pub fn bg_display(&self) -> bool {
        self.control & 0b00000001 != 0
    }

    pub fn window_tile_map_display_select(&self) -> Range<u16> {
        // give me a bit of self.control ...
        if self.control & 0b01000000 == 0 {
            0x9800..0x9BFF
        } else {
            0x9C00..0x9FFF
        }
    }

    pub fn bg_and_window_tile_data_select(&self) -> Range<u16> {
        if self.control & 0b00010000 == 0 {
            0x8800..0x97FF
        } else {
            0x8000..0x8FFF
        }
    }

    pub fn bg_tile_map_display_select(&self) -> Range<u16> {
        if self.control & 0b00001000 == 0 {
            0x9800..0x9BFF
        } else {
            0x9C00..0x9FFF
        }
    }

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
        self.control = { self.memory.read().unwrap()[0xFF40 as u16] };
        if self.enabled() {
            self.draw();
        }
    }

    fn draw(&mut self) {
        let color = { self.memory.read().unwrap()[10 as u16] };
        for i in &mut self.buffer {
            let gray = color as u32;
            *i = gray << 16 | gray << 8 | gray;
        }
        self.window.update_with_buffer(&self.buffer);
    }

    fn run(&mut self) {
        println!("LcdScreen::run");
        self.memory.write().unwrap()[0xFF40 as u16] |= 0b10000000;
    }
}

impl fmt::Display for LcdScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //let memory = self.memory.read().unwrap();
        try!(writeln!(f, "control: {control:0>4X}", control=self.control));
        Ok(())
    }
}
