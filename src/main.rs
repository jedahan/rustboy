use std::env;
use std::path::Path;
use std::fmt;

extern crate rustboylib;

use rustboylib::load;

struct Cpu {
    pc: u16,
    sp: u16,
    registers: [u8; 8],
    wram: [u8; 1024],
    vram: [u8; 1024]
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            pc: 0x0100,
            sp: 0xFFFE,
            // A, B, D, H, F, C, E, L?
            registers: [0 as u8; 8],
            wram: [0; 1024],
            vram: [0; 1024]
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "{:?}", self.registers));
        try!(writeln!(f, "pc: {:0>4X}", self.pc));
        try!(writeln!(f, "sp: {:0>4X}", self.sp));
        Ok(())
    }
}

fn main() {
    let boot_rom_filename = env::args().nth(1).unwrap();
    let boot = load::rom(Path::new(&boot_rom_filename));
    println!("{:?}", boot);

    let cart_rom_filename = env::args().nth(2).unwrap();
    let cart = load::cart(Path::new(&cart_rom_filename));
    println!("{}", cart);

    let cpu = Cpu::new();
    println!("{}", cpu);
}

/*
*  map to something?
fn map(address: u8) -> &[u8] {
  if address < 0x100 {
    boot.mem[address]
  } else {
    game.mem[address-0x100]
  }
}
*/

