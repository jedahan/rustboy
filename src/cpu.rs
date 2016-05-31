use std::fmt;

use interconnect;

pub struct Cpu {
    pc: u16,
    sp: u16,
    reg_a: u8,
    reg_f: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,

    interconnect: interconnect::Interconnect
}

impl Cpu {
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            reg_a: 0,
            reg_f: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,

            interconnect: interconnect
        }
    }

    pub fn run(&mut self) {
        println!("I am running!");
        while self.pc < 0x250 {
            let opcode = self.read_word(self.pc as usize);
            println!("opcode {:0>2X}", opcode);
            self.pc = self.pc + 1;
        }

    }

    fn read_word(&self, address: usize) -> u8 {
        self.interconnect[address]
    }

    pub fn reset(&mut self) {
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
