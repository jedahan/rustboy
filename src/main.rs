mod lib;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::fmt;

use lib::cart;

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
        writeln!(f, "{:?}", self.registers);
        writeln!(f, "pc: {:0>4X}", self.pc);
        writeln!(f, "sp: {:0>4X}", self.sp);
        Ok(())
    }
}

fn main() {
    let boot_rom_file_name = env::args().nth(1).unwrap();
    let cart_rom_file_name = env::args().nth(2).unwrap();
    let boot = load_rom(boot_rom_file_name);
    let cart = load_cart(cart_rom_file_name);

    println!("{}", cart);

    let cpu = Cpu::new();
    println!("{}", cpu);
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn load_cart<P: AsRef<Path>>(path: P) -> cart::Cart {
    cart::Cart {
        mem: load_rom(path),
        ..Default::default()
    }
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

#[test]
fn checksums() {
    let dir = Path::new("roms");
    if fs::metadata(dir).unwrap().is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                let filepath = entry.path();
                println!("testing {:?}", filepath);
                assert!(load_cart(filepath).is_valid());
            }
        }
    }
}
