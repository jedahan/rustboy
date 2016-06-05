use std::fmt;

use interconnect;

pub const ZERO_BIT: u8 = 1 << 7;
pub const SUBTRACT_BIT: u8 = 1 << 6;
pub const HALFCARRY_BIT: u8 = 1 << 5;
pub const CARRY_BIT: u8 = 1 << 4;

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
            operations: 0,

            interconnect: interconnect
        }
    }

    pub fn run(&mut self) {
        println!("rustboy is running");
        loop {
            let opcode = self.interconnect[self.pc];

            match opcode {
                0x00 => {
                    println!("NOP");
                }
                0x20 => {
                    println!("JR NZ,r8");
                }
                0x21 => {
                    self.pc += 1;
                    self.reg_l = self.interconnect[self.pc];
                    self.pc += 1;
                    self.reg_h = self.interconnect[self.pc];
                    println!("LD HL, d16");
                }
                0x31 => {
                    self.pc = self.pc + 1;
                    let address = self.read_word(self.pc);
                    self.sp = address;
                    self.pc = self.pc + 1;
                    println!("LD SP, {:0>4x}", address);
                }
                0x32 => {
                    //println!("ldd (HL),A");
                    // Stands for Load and Decrement
                    // TODO: set the zero flag if something bad happened
                    let address = ((self.reg_h as u16) << 8) |
                                  ((self.reg_l as u16) << 0);
                    println!("LDD {:0>4X}, A", address);
                    self.interconnect[address] = self.reg_a - 1;
                }
                0xAF => {
                    // the docs say this zeros out $8000-$FFFE
                    // I am not sure if that means load 0 into $8000->self.reg_sp
                    // or actually xor $8000->self.reg_sp
                    // it should only take 4 clock cycles, and we are already are zero'd out
                    // so I think we are fine doing basically nothing
                    // TODO: figure out what this actually does.
                    println!("XOR A");
                }
                0xC3 => {
                    let pc_address = self.pc + 1;
                    let jump_address = self.read_word(pc_address);
                    println!("JMP {:0>4X}", jump_address);
                    self.jump(jump_address);
                    self.pc -= 1; // TODO: CHECK THIS
                }
                0xE0 => {
                    let offset = self.interconnect[self.pc+1];
                    let address = 0xFF00 + offset as u16;
                    let value = self.interconnect[address];
                    self.reg_a = value;
                    println!("LDH ({}), A", offset);
                    self.pc = self.pc + 1;
                }
                0xE1 => {
                    self.sp = self.sp + 1;
                    self.reg_h = self.interconnect[self.sp];
                    self.interconnect[self.sp] = 0;
                    self.sp = self.sp + 1;
                    self.reg_l = self.interconnect[self.sp];
                    self.interconnect[self.sp] = 0;
                    println!("POP HL");
                }
                0xC9 => {
                    self.ret();
                    println!("RET");
                    self.pc = self.pc - 1; // TODO CHECKME
                }
                0xCB => {
                    self.pc += 1;
                    let z80opcode = self.interconnect[self.pc];
                    match z80opcode {
                        0x7C => {
                            let reg_h = self.reg_h;
                            self.reg_h = self.bit_shift(7, reg_h);
                            println!("BIT 7, H");
                        }
                        _ => panic!("unrecognized z80 opcode {:0>2X}", opcode)
                    }
                }
                _ => panic!("unrecognized opcode {:0>2X}", opcode)
            }

            self.pc += 1;
            self.operations = self.operations + 1;
            println!("{}", self);
        }

    }

    fn bit_shift(&mut self, amount: u8, reg: u8) -> u8 {
        self.unset_subtract();
        self.set_half_carry();
        reg >> amount
    }

    fn unset_subtract(&mut self) {
        self.reg_f = self.reg_f & ! (1<<6);
    }

    fn set_half_carry(&mut self) {
        self.reg_f = self.reg_f | (1<<5);
    }

    fn ret(&mut self) {
        self.pc = self.sp;
        self.sp = 0x0000; // TODO: what do we do with the stack pointer? put the return value?
        self.sp = self.sp + 2; // move back "up"
    }

    /* when we jump to a new address, make sure to save the current program counter
     * address to the bottom of the stack, so when we can return to the current address
     */
    fn jump(&mut self, address: u16) {
        self.sp = self.sp - 2;
        let address_high = (self.pc >> 0) as u8;
        let address_low  = (self.pc >> 8) as u8;
        self.interconnect[self.sp + 0] = address_high;
        self.interconnect[self.sp + 1] = address_low;
        self.pc = address;
    }

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

    fn flag_zero(&self) -> bool { &self.reg_f & ZERO_BIT > 0 }
    fn flag_subtract(&self) -> bool { &self.reg_f & SUBTRACT_BIT > 0 }
    fn flag_halfcarry(&self) -> bool { &self.reg_f & HALFCARRY_BIT > 0 }
    fn flag_carry(&self) -> bool { &self.reg_f & CARRY_BIT > 0 }
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
