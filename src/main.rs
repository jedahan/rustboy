mod lib;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::fmt;

use lib::cart;

#[derive(Default, Debug)]
struct Cpu {
    pc: u16,
    sp: u16,
    reg_a: u8,
    reg_f: u8,

    reg_b: u8,
    reg_c: u8,

    reg_d: u8,
    reg_e: u8,

    reg_h: u8,
    reg_l: u8
}

impl Cpu {
    fn new() -> Cpu {
        Default::default()
    }
    fn reset(&mut self) {
        self.pc = 0x0100;
        self.sp = 0xFFFE;
        self.reg_a = 0x01;
        self.reg_f = 0xB0;

        self.reg_b = 0x00;
        self.reg_c = 0x13;

        self.reg_d = 0x00;
        self.reg_e = 0xD8;

        self.reg_h = 0x01;
        self.reg_l = 0x4D;
    }
    fn flag_zero(&self) -> bool {
        &self.reg_f & 0b10000000 > 0
    }
    fn flag_subtract(&self) -> bool {
        &self.reg_f & 0b01000000 > 0
    }
    fn flag_half_carry(&self) -> bool {
        &self.reg_f & 0b00100000 > 0
    }
    fn flag_carry(&self) -> bool {
        &self.reg_f & 0b00010000 > 0
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "cpu {{"));
        try!(writeln!(f, "  pc: {:0>4X}", self.pc));
        try!(writeln!(f, "  sp: {:0>4X}", self.sp));
        try!(writeln!(f, "  registers {{"));
        try!(writeln!(f,
            "    {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>2}",
            "a", "f", "b", "c", "d", "e", "h", "l"
        ));

        try!(writeln!(f,
            "    {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X} {:0>2X}",
            self.reg_a, self.reg_f, self.reg_b, self.reg_c, self.reg_d, self.reg_e, self.reg_h, self.reg_l
        ));
        try!(writeln!(f, "  }}"));

        try!(writeln!(f, "  flags {{"));
        try!(write!(f, "    zero: {}", self.flag_zero()));
        try!(write!(f, ", sub: {}", self.flag_subtract()));
        try!(write!(f, ", half: {}", self.flag_half_carry()));
        try!(writeln!(f, ", carry: {}", self.flag_carry()));
        try!(writeln!(f, "  }}"));
        try!(writeln!(f, "}}"));
        Ok(())
    }
}

fn main() {
    let boot_rom_file_name = env::args().nth(1).unwrap();
    let boot = load_rom(Path::new(&boot_rom_file_name));
    println!("boot: {:?}", boot);

    let cart_rom_file_name = env::args().nth(2).unwrap();
    let cart = load_cart(Path::new(&cart_rom_file_name));
    println!("{}", cart);

    let mut cpu = Cpu::new();
    cpu.reset();
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
