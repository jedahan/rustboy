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
        println!("I am running!");
        while self.operations < 5 {
            let opcode = self.interconnect[self.pc];

            match opcode {
                0x00 => {
                    println!("NOP");
                    self.pc = self.pc + 1;
                    continue;
                }
                0xC3 => {
                    let pc_address = self.pc + 1;
                    let jump_address = self.read_word(pc_address);
                    println!("JMP {:0>4X}", jump_address);
                    self.jump(jump_address);
                }
                0xE0 => {
                    let offset = self.interconnect[self.pc+1];
                    let address = 0xFF00 + offset as u16;
                    let value = self.interconnect[address];
                    self.reg_a = value;
                    println!("LDH ({}), A", offset);
                }
                0xE1 => {
                    self.sp = self.sp + 1;
                    self.reg_h = self.interconnect[self.sp];
                    self.interconnect[self.sp] = 0;
                    self.sp = self.sp + 1;
                    self.reg_l = self.interconnect[self.sp];
                    self.interconnect[self.sp] = 0;
                    println!("POP HL");
                    self.pc = self.pc + 1;
                }
                0xC9 => {
                    self.ret();
                    println!("RET");
                }
                0xCB => {
                    println!("Z80 command prefix");
                    continue;
                }
                _ => panic!("unrecognized opcode {:0>2X}", opcode)
            }

            self.operations = self.operations + 1;
            println!("{}", self);
        }

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
        &self.reg_f & (1<<7) > 0
    }

    fn flag_subtract(&self) -> bool {
        &self.reg_f & (1<<6) > 0
    }

    fn flag_half_carry(&self) -> bool {
        &self.reg_f & (1<<5) > 0
    }

    fn flag_carry(&self) -> bool {
        &self.reg_f & (1<<4) > 0
    }

    fn print_stack(&self) -> fmt::Result {
        println!("stack: ");
        let height: usize = 0xF;
        let mut depth = 0;

        while depth < height {
            let byte = 0xFFFF - depth;
            let mut sp = " ";
            if byte == self.sp as usize {
                sp = "❯";
            }
            println!("{arrow} 0x{address:0>4X}: {value:0>2X}", arrow=sp, address=byte, value=self.interconnect[byte]);
            depth = depth + 1;
        }
        Ok(())
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f,
"cpu {{
  pc: {pc:0>4X} [{i0:0>2X} {i1:0>2X} {i2:0>2X} {i3:0>2X}]
  sp: {sp:0>4X}
  registers: {{ a: {a:0>2X}, f: {f:0>2X}, b: {b:0>2X}, c: {c:0>2X}, d: {d:0>2X}, e: {e:0>2X}, h: {h:0>2X}, l: {l:0>2X} }}
  flags: {{ zero: {zero}, sub: {sub}, half: {half}, carry: {carry} }}
  instructions: {{}}
}}",
    pc=self.pc, i0=self.interconnect[self.pc+0], i1=self.interconnect[self.pc+1], i2=self.interconnect[self.pc+2], i3=self.interconnect[self.pc+3],
    sp=self.sp,
    a=self.reg_a, f=self.reg_f, b=self.reg_b, c=self.reg_c, d=self.reg_d, e=self.reg_e, h=self.reg_h, l=self.reg_l,
    zero=self.flag_zero(), sub=self.flag_subtract(), half=self.flag_half_carry(), carry=self.flag_carry()));

    self.print_stack()
    }
}
