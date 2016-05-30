use std::fmt;

use cpu;
use interconnect;

#[derive(Default)]
pub struct GameBoy {
    cpu: cpu::Cpu,
    interconnect: interconnect::Interconnect
}

impl GameBoy {
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
