use std::fmt;

use cpu;
use cart;
use memory;

pub const BOOTROM_SIZE: usize = 256;

pub struct GameBoy {
    cpu: cpu::Cpu
}

impl GameBoy {
    pub fn new(boot: [u8; BOOTROM_SIZE], cart: cart::Cart) -> GameBoy {
        let memory = memory::Memory::new(boot, cart);
        GameBoy {
            cpu: cpu::Cpu::new(memory)
        }
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
