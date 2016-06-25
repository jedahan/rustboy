extern crate minifb;

use std::ops::Range;
use self::minifb::WindowOptions;

#[derive(Debug)]
pub enum Mode {
    Hblank, Vblank, Oam, Vram
}

pub struct LCD {
    mode: Mode,
    scroll: u16,
    control: u8,
    buffer: Vec<u32>,
    window: minifb::Window,
}

impl LCD {

    pub fn new() -> Self {
        LCD {
            mode: Mode::Hblank,
            scroll: 0x0000,
            control: 0,
            buffer: vec![0; 160*144],
            window: minifb::Window::new(
                "rustboy",
                160,
                144,
                WindowOptions {
                    borderless: true,
                    scale: minifb::Scale::X4,
                    ..Default::default()
                }
            ).unwrap()
        }
    }

    pub fn step(&mut self) {
        if self.enable() {
            match self.mode {
                Mode::Hblank => return,
                Mode::Vblank => return,
                Mode::Oam => return,
                Mode::Vram => return
            }
        }
    }

    pub fn enable(&self) -> bool {
        self.control & 0b10000000 != 0
    }

    pub fn window_tile_map_display_select(&self) -> Range<u16> {
        // give me a bit of self.control ...
        if self.control & 0b01000000 == 0 {
          0x9800..0x9BFF
        } else {
          0x9C00..0x9FFF
        }
    }

    pub fn window_display_enable(&self) -> bool {
        self.control & 0b00100000 != 0
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
        (8,8)
      } else {
        (8,16)
      }
    }

    pub fn obj_display_enable(&self) -> bool {
      self.control & 0b00000010 != 0
    }

    pub fn bg_display(&self) -> bool {
      self.control & 0b00000001 != 0
    }
}
