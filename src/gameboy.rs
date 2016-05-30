use std::fmt;

use cpu;
use interconnect;

pub const BOOTROM_SIZE: usize = 256;

pub struct GameBoy {
    cpu: cpu::Cpu
}

impl GameBoy {
    pub fn new(boot: [u8; BOOTROM_SIZE]) -> GameBoy {
        let interconnect = interconnect::Interconnect::new(boot);
        GameBoy {
            cpu: cpu::Cpu::new(interconnect)
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
        try!(writeln!(f, "gameboy {{"));
        try!(writeln!(f, "{}", self.cpu));
        writeln!(f, "}}")
    }
}
