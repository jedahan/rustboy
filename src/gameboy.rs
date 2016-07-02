use std::fmt;
use std::env;

use cpu;
use cart;
use memory;
use debug;
use lcd;
use window;

pub const BOOTROM_SIZE: usize = 256;

pub struct GameBoy {
    cpu: cpu::Cpu,
}

impl GameBoy {
    pub fn new(boot: [u8; BOOTROM_SIZE], cart: cart::Cart) -> GameBoy {
        let memory = memory::Memory::new(boot, cart);
        let screen: Box<window::Drawable> = match env::var("DEBUG") {
            Ok(_) => Box::new(debug::DebugScreen::new(160, 288)),
            _ => Box::new(lcd::LcdScreen::new(160, 144)),
        };

        GameBoy { cpu: cpu::Cpu::new(memory, screen) }
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}

impl fmt::Display for GameBoy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.cpu)
    }
}
