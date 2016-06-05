use std::fmt;

use interconnect;
use std::fmt::Write;
use std::env;

pub const FLAG_ZERO: u8 = 1 << 7;
pub const FLAG_SUBTRACT: u8 = 1 << 6;
pub const FLAG_HALFCARRY: u8 = 1 << 5;
pub const FLAG_CARRY: u8 = 1 << 4;

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
    operations: u8,
    debug: bool,

    interconnect: interconnect::Interconnect
}

impl Cpu {
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu {
        let debug = match env::var("DEBUG") {
            Ok(_) => true,
            _ => false
        };
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
            operations: 0,
            debug: debug,

            interconnect: interconnect
        }
    }

    fn flag_zero(&self) -> bool { &self.reg_f & FLAG_ZERO > 0 }
    fn flag_subtract(&self) -> bool { &self.reg_f & FLAG_SUBTRACT > 0 }
    fn flag_halfcarry(&self) -> bool { &self.reg_f & FLAG_HALFCARRY > 0 }
    fn flag_carry(&self) -> bool { &self.reg_f & FLAG_CARRY > 0 }

    fn set(&mut self, bit: u8) { self.reg_f |= bit; }
    fn unset(&mut self, bit: u8) { self.reg_f &= ! bit; }

    // Not sure if this is little-endian or big-endian
    fn read_word(&self, address: u16) -> u16 {
        (self.interconnect[address + 1] as u16) << 8 |
        (self.interconnect[address + 0] as u16)
    }

    pub fn reset(&mut self) {
        self.pc = 0x0000;
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

    pub fn run(&mut self) {
        println!("rustboy is running");
        loop {
            let instruction = self.fetch();
            self.execute(instruction);
            self.operations += 1;
            if self.debug {
                println!("{}: {}", self.operations, self);
            }
        }
    }

    fn fetch(&mut self) -> (u8, u8) {
        let opcode = self.interconnect[self.pc];
        self.pc += 1;
        if opcode == 0xCB {
            self.pc += 1;
            return (opcode, self.interconnect[self.pc])
        }
        (0, opcode)
    }

    fn execute(&mut self, instruction: (u8, u8)) {
        match instruction {
            // z80prefix
            (0xCB, opcode) => {
                match opcode {
                    0x7C => { &self.bit_7_h(); }
                    _ => { panic!("unrecognized z80 opcode {:0>2X}", opcode) }
                }
            }
            (_, opcode) => {
                match opcode {
                    0x00 => { &self.nop(); }
                    0x20 => { &self.jr_nz_r8(); }
                    0x21 => { &self.ld_hl_d16(); }
                    0x31 => { &self.ld_sp_d16(); }
                    0x32 => { &self.ldd_d16_a(); }
                    0xAF => { &self.xor_a(); }
                    0xC3 => { &self.jmp_a16(); }
                    0xE0 => { &self.ldh_a8_a(); }
                    0xE1 => { &self.pop_hl(); }
                    0xC9 => { &self.ret(); }
                    _ => panic!("unrecognized opcode {:0>2X}", opcode)
                }
            }
        }
    }

    fn print_disassembly(&self, instruction: String, num_bytes: u16) {
        let start = self.pc - 1;

        let mut s = String::new();
        for &byte in &self.interconnect[start..start + num_bytes as u16] {
            write!(&mut s, "0x{:0X} ", byte).unwrap();
        }
        println!("[0x{:0>8X}] {:<15} {}", start, s, instruction)
    }

    // OPERATIONS START HERE

    fn ret(&mut self) {
        self.pc = self.sp;
         // move back "up" the stack, zeroing out
        self.sp += 1;
        self.sp = 0x00;
        self.sp += 1;
        self.sp = 0x00;
    }

    /* when we jump to a new address, make sure to save the current program counter
     * address to the bottom of the stack, so when we can return to the current address
     */
    fn jmp_a16(&mut self) {
        let address = self.read_word(self.pc + 1);
        println!("JMP {:0>4X}", address);

        let address_high = (self.pc >> 0) as u8;
        self.interconnect[self.sp] = address_high;
        self.sp -= 1;

        let address_low  = (self.pc >> 8) as u8;
        self.interconnect[self.sp] = address_low;
        self.sp -= 1;

        self.pc = address;
    }

    fn ld_hl_d16(&mut self) {
        self.reg_l = self.interconnect[self.pc+0];
        self.reg_h = self.interconnect[self.pc+1];
        self.print_disassembly(format!("LD HL,${:0>2X}{:0>2X}", self.reg_h, self.reg_l), 3);
        self.pc += 2;
    }

    fn nop(&self) {
        println!("NOP");
    }

    fn jr_nz_r8(&self) {
        println!("JR NZ, r8");
    }

    fn bit_7_h(&mut self) {
        self.unset(FLAG_SUBTRACT);
        self.set(FLAG_HALFCARRY);
        if self.reg_h < (1<<7) {
            self.set(FLAG_ZERO);
        }
        self.reg_h >>= 7;
    }

    fn ld_sp_d16(&mut self) {
        let address = self.read_word(self.pc);
        self.print_disassembly(format!("LD SP,${:0>4X}", address), 3);
        self.sp = address;
        self.pc = self.pc + 2;
    }

    // load and decrement a into the address (HL)
    fn ldd_d16_a(&mut self) {
        self.print_disassembly(format!("LDD (HL), A"), 1);
        let address = ((self.reg_h as u16) << 8) |
                      ((self.reg_l as u16) << 0);
        if self.reg_a == 0 {
            self.set(FLAG_ZERO);
            self.interconnect[address] = self.reg_a;
        } else {
            self.interconnect[address] = self.reg_a - 1;
        }
    }

    // the docs say this zeros out $8000-$FFFE
    // I am not sure if that means load 0 into $8000->self.reg_sp
    // or actually xor $8000->self.reg_sp
    // it should only take 4 clock cycles, and we are already are zero'd out
    // so I think we are fine doing basically nothing
    // TODO: figure out what this actually does.
    fn xor_a(&self) {
        self.print_disassembly(format!("XOR A"), 1);
    }

    fn ldh_a8_a(&mut self)  {
        let offset = self.interconnect[self.pc + 1];
        println!("LDH ({}), A", offset);

        let address = 0xFF00 + offset as u16;
        let value = self.interconnect[address];
        self.reg_a = value;
        self.pc = self.pc + 1;
    }

    fn pop_hl(&mut self) {
        println!("POP HL");
        self.sp = self.sp + 1;
        self.reg_h = self.interconnect[self.sp];
        self.interconnect[self.sp] = 0;
        self.sp = self.sp + 1;
        self.reg_l = self.interconnect[self.sp];
        self.interconnect[self.sp] = 0;
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f,
            "cpu {{\
            \n\tpc: {pc:0>4X} [{i0:0>2X} {i1:0>2X} {i2:0>2X} {i3:0>2X}]\
            \n\tsp: {sp:0>4X}\
            \n\tregisters: {{ a: {a:0>2X}, f: {f:0>2X}, b: {b:0>2X}, c: {c:0>2X}, d: {d:0>2X}, e: {e:0>2X}, h: {h:0>2X}, l: {l:0>2X} }}\
            \n\tflags: {{ zero: {zero}, sub: {sub}, half: {half}, carry: {carry} }}\
            \n}}
            ",
                pc=self.pc, i0=self.interconnect[self.pc+0], i1=self.interconnect[self.pc+1], i2=self.interconnect[self.pc+2], i3=self.interconnect[self.pc+3],
                sp=self.sp,
                a=self.reg_a, f=self.reg_f, b=self.reg_b, c=self.reg_c, d=self.reg_d, e=self.reg_e, h=self.reg_h, l=self.reg_l,
                zero=self.flag_zero(), sub=self.flag_subtract(), half=self.flag_halfcarry(), carry=self.flag_carry()));

        try!(writeln!(f, "mem {{\n  stack:\tvram:"));
        for depth in 0..8 {
            let byte = 0xFFFF - depth as u16;
            let arrow = if self.sp == byte { "❯" } else { " " };

            try!(writeln!(f, "{}   0x{:0>4X}: {:0>2X} \t  0x{:0>4X}: {:0>2X} \t\t",
                arrow, byte, self.interconnect[byte], byte-0x6000, self.interconnect[byte-0x6000]
            ));
        }
        writeln!(f, "}}")
    }
}
