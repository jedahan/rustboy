extern crate minifb;

use std::ops::Range;
use std::thread;
use self::minifb::WindowOptions;
use std::time::{Duration, Instant};
use window;
use memory;

// TODO: figure out what these numbers actually mean
const HBLANKS_BEFORE_DRAW: u8 = 204;
const LINES_BEFORE_VBLANK: u16 = 456;
const VBLANKS_BEFORE_HBLANK: u16 = 10 * LINES_BEFORE_VBLANK;
const VBLANK_LINES_BEFORE_OAM: u8 = 153;
const OAMS_BEFORE_VRAM: u8 = 80;
const VRAMS_BEFORE_HBLANK: u8 = 172;

#[derive(Debug)]
pub enum Mode {
    Hblank,
    Vblank,
    Oam,
    Vram,
}

pub struct LcdScreen {
    mode: Mode,
    // TODO: switch modeclock to just be calculated off of the lcd m/t clocks
    modeclock: u8,
    line: u8,
    scroll: u16,
    control: u8,
    buffer: Vec<u32>,
    memory: memory::Memory,
    window: minifb::Window,
}

impl LcdScreen {
    pub fn new(width: usize, height: usize, memory: memory::Memory) -> Self {
        LcdScreen {
            mode: Mode::Hblank,
            modeclock: 0,
            line: 0,
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

    fn mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.modeclock = 0;
    }

    fn render_scanline(&self) {
        return;
    }

    // ported from http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings
    // because i am lazy and not sure how things work
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
            (8, 8)
        } else {
            (8, 16)
        }
    }

    pub fn obj_display_enable(&self) -> bool {
        self.control & 0b00000010 != 0
    }

    pub fn bg_display(&self) -> bool {
        self.control & 0b00000001 != 0
    }
}

impl window::Drawable for LcdScreen {
    fn update(&mut self) {
        if self.enable() {
            match self.mode {
                Mode::Hblank => {
                    if self.modeclock > HBLANKS_BEFORE_DRAW {
                        self.modeclock = 0;
                        self.line += 1;

                        if self.line as u16 == LINES_BEFORE_VBLANK {
                            self.mode = Mode::Vblank;
                            self.draw();
                        }
                    }
                }
                Mode::Vblank => {
                    if self.modeclock as u16 >= VBLANKS_BEFORE_HBLANK {
                        self.modeclock = 0;
                        self.line += 1;

                        if self.line > VBLANK_LINES_BEFORE_OAM {
                            self.mode(Mode::Oam);
                        }
                    }
                }
                Mode::Oam => {
                    if self.modeclock >= OAMS_BEFORE_VRAM {
                        self.mode(Mode::Vram);
                    }
                }
                Mode::Vram => {
                    if self.modeclock >= VRAMS_BEFORE_HBLANK {
                        self.mode(Mode::Hblank);

                        self.render_scanline();
                    }
                }
            }
        }
    }

    fn draw(&mut self) {
        for i in self.buffer.iter_mut() {
            let gray = 6 as u32;
            *i = gray << 16 | gray << 8 | gray;
        }
        self.window.update_with_buffer(&self.buffer);
    }

    fn run(&mut self) {
        let frame_duration = Duration::from_millis(16);
        let mut previous_draw = Instant::now();

        self.update();

        let now = Instant::now();
        if now - previous_draw > frame_duration {
            self.draw();
            previous_draw = now;
        }
    }
}
