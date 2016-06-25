use std::fmt;

use debug;
use memory;
use std::fmt::Write;
use std::env;
use std::time::{Duration, Instant};

pub enum Flag {ZERO, SUBTRACT, HALFCARRY, CARRY}

impl Flag {
    fn to_u8(self) -> u8 { 1 << (7 - self as u8) }
}

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
    memory: memory::Memory,
    screen: debug::Screen,
    operations: usize,
    debug: bool,
}

impl Cpu {

    fn print_stack_and_vram(&self, height: usize) {
        println!("mem {{\n  stack:\tvram:");
        for depth in 0..height {
            let byte = 0xFFFF - depth as u16;
            let arrow = if self.sp == byte { "❯" } else { " " };

            println!("{}   0x{:0>4X}: {:0>2X} \t  0x{:0>4X}: {:0>2X} \t\t",
                arrow, byte, self.memory[byte], byte-0x6000, self.memory[byte-0x6000]
            )
        }
        println!("}}");

    }

    fn crash(&self, message: String) {
        println!("{:0>4X}: {}", self.operations, self);
        self.print_stack_and_vram(0xFF);
        panic!(message);
    }

    pub fn new(memory: memory::Memory, screen: debug::Screen) -> Cpu {
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

            screen: screen,

            operations: 0,
            debug: debug,

            memory: memory
        }
    }

    // Not sure if this is little-endian or big-endian
    fn read_word(&self, address: u16) -> u16 {
        (self.memory[address + 1] as u16) << 8 |
        (self.memory[address + 0] as u16)
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
        let frame_duration = Duration::from_millis(16);
        let mut previous_draw = Instant::now();

        println!("rustboy is running");
        loop {
            let instruction = self.fetch();
            let advance = self.execute(instruction);
            self.pc += advance;
            self.operations += 1;

            if self.debug {
                let now = Instant::now();
                if now - previous_draw > frame_duration {
                    self.screen.draw(&self.memory);
                    previous_draw = now;
                }
                println!("{:0>4X}: {}", self.operations, self);
            }
        }
    }

    fn fetch(&mut self) -> (u8, u8) {
        let opcode = self.memory[self.pc];
        if opcode == 0xCB {
            self.pc += 1;

            return (opcode, self.memory[self.pc])
        }
        (0, opcode)
    }

    fn execute(&mut self, instruction: (u8, u8)) -> u16 {
        match instruction {
            // z80prefix
            (0xCB, opcode) => {
                match opcode {
                    0x7C => { self.bit_h(7) }
                    0x11 => { self.rl_c() }
                    _ => { self.crash(format!("unrecognized z80 opcode {:0>2X}", opcode)); unreachable!() }
                }
            }
            (_, opcode) => {
                match opcode {
                    0x00 => { self.nop() }

                    /* math */
                    0x0C => { self.inc_c() }
                    0x1C => { self.inc_e() }
                    0x2C => { self.inc_l() }
                    0x3C => { self.inc_a() }
                    0x04 => { self.inc_b() }
                    0x14 => { self.inc_d() }
                    0x24 => { self.inc_h() }

                    0x13 => { self.inc_de() }
                    0x23 => { self.inc_hl() }

                    0x0D => { self.dec_c() }
                    0x1D => { self.dec_e() }
                    0x2D => { self.dec_l() }
                    0x3D => { self.dec_a() }
                    0x05 => { self.dec_b() }
                    0x15 => { self.dec_d() }
                    0x25 => { self.dec_h() }

                    0xAF => { self.xor_a() }

                    0x17 => { self.rla() }

                    /* flow */
                    0x18 => { self.jr_r8() }
                    0x28 => { self.jr(Flag::ZERO, true) }
                    0x20 => { self.jr(Flag::ZERO, false) }
                    0xC3 => { self.jmp_a16() }

                    0xCD => { self.call() }

                    0xC9 => { self.ret() }

                    /* stack */
                    0xFE => { self.cp_d8() }

                    0xC5 => { self.push_bc() }

                    0xC1 => { self.pop_bc() }
                    0xE1 => { self.pop_hl() }

                    /* loading */
                    0x7F => { unborrow!(self.ld_a(self.a())) }
                    0x78 => { unborrow!(self.ld_a(self.b())) }
                    0x79 => { unborrow!(self.ld_a(self.c())) }
                    0x7A => { unborrow!(self.ld_a(self.d())) }
                    0x7B => { unborrow!(self.ld_a(self.e())) }
                    0x7C => { unborrow!(self.ld_a(self.h())) }
                    0x7D => { unborrow!(self.ld_a(self.l())) }

                    0x47 => { unborrow!(self.ld_b(self.a())) }
                    0x40 => { unborrow!(self.ld_b(self.b())) }
                    0x41 => { unborrow!(self.ld_b(self.c())) }
                    0x42 => { unborrow!(self.ld_b(self.d())) }
                    0x43 => { unborrow!(self.ld_b(self.e())) }
                    0x44 => { unborrow!(self.ld_b(self.h())) }
                    0x45 => { unborrow!(self.ld_b(self.l())) }

                    0x4F => { unborrow!(self.ld_c(self.a())) }
                    0x48 => { unborrow!(self.ld_c(self.b())) }
                    0x49 => { unborrow!(self.ld_c(self.c())) }
                    0x4A => { unborrow!(self.ld_c(self.d())) }
                    0x4B => { unborrow!(self.ld_c(self.e())) }
                    0x4C => { unborrow!(self.ld_c(self.h())) }
                    0x4D => { unborrow!(self.ld_c(self.l())) }

                    0x57 => { unborrow!(self.ld_d(self.a())) }
                    0x50 => { unborrow!(self.ld_d(self.b())) }
                    0x51 => { unborrow!(self.ld_d(self.c())) }
                    0x52 => { unborrow!(self.ld_d(self.d())) }
                    0x53 => { unborrow!(self.ld_d(self.e())) }
                    0x54 => { unborrow!(self.ld_d(self.h())) }
                    0x55 => { unborrow!(self.ld_d(self.l())) }

                    0x5F => { unborrow!(self.ld_e(self.a())) }
                    0x58 => { unborrow!(self.ld_e(self.b())) }
                    0x59 => { unborrow!(self.ld_e(self.c())) }
                    0x5A => { unborrow!(self.ld_e(self.d())) }
                    0x5B => { unborrow!(self.ld_e(self.e())) }
                    0x5C => { unborrow!(self.ld_e(self.h())) }
                    0x5D => { unborrow!(self.ld_e(self.l())) }

                    0x67 => { unborrow!(self.ld_h(self.a())) }
                    0x60 => { unborrow!(self.ld_h(self.b())) }
                    0x61 => { unborrow!(self.ld_h(self.c())) }
                    0x62 => { unborrow!(self.ld_h(self.d())) }
                    0x63 => { unborrow!(self.ld_h(self.e())) }
                    0x64 => { unborrow!(self.ld_h(self.h())) }
                    0x65 => { unborrow!(self.ld_h(self.l())) }

                    0x6F => { unborrow!(self.ld_l(self.a())) }
                    0x68 => { unborrow!(self.ld_l(self.b())) }
                    0x69 => { unborrow!(self.ld_l(self.c())) }
                    0x6A => { unborrow!(self.ld_l(self.d())) }
                    0x6B => { unborrow!(self.ld_l(self.e())) }
                    0x6C => { unborrow!(self.ld_l(self.h())) }
                    0x6D => { unborrow!(self.ld_l(self.l())) }

                    0x1A => { self.ld_a_de() }

                    0x3E => { self.ld_a_d8() }
                    0x06 => { self.ld_b_d8() }
                    0x0E => { self.ld_c_d8() }
                    0x16 => { self.ld_d_d8() }
                    0x1E => { self.ld_e_d8() }
                    0x26 => { self.ld_h_d8() }
                    0x2E => { self.ld_l_d8() }

                    0x77 => { self.ld_hl_a() }

                    0x11 => { self.ld_de_d16() }
                    0x21 => { self.ld_hl_d16() }
                    0x31 => { self.ld_sp_d16() }

                    0x22 => { self.ldi_hl_a() }

                    0x32 => { self.ldd_hl_a() }

                    0xE0 => { self.ldh_a8_a() }
                    0xF0 => { self.ldh_a_a8() }

                    0xE2 => { self.ldr_c_a() }
                    0xEA => { self.ld_a16_a() }
                    _ => {
                        println!("unrecognized opcode {:0>2X}", opcode);
                        self.screen.debug(&self.memory);
                        0
                    }
                }
            }
        }
    }

    pub fn get(&self, flag: Flag) -> bool { self.reg_f & flag.to_u8() != 0 }

    pub fn set(&mut self, flag: Flag, set: bool) {
        if set {
            self.reg_f |= flag.to_u8();
        } else {
            self.reg_f &= ! flag.to_u8();
        }
    }

    fn rla(&mut self) -> u16 {
        let size = 1;
        let amount = self.memory[self.pc + 1];
        self.print_disassembly(format!("RLA ({})", amount), size);
        if self.reg_a & 0b10000000 != 0 {
            self.set(Flag::CARRY, true)
        }
        self.reg_a <<= amount as u32;
        size
    }

    fn rl_c(&mut self) -> u16 {
        let size = 1;
        let amount = self.memory[self.pc + 1];
        self.print_disassembly(format!("RL C ({})", amount), size);
        self.reg_c.rotate_left(amount as u32);
        size
    }

    fn push_bc(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("PUSH BC ${:0>2X}{:0>2X}", self.reg_b, self.reg_c), size);
        self.sp -= 1;
        self.memory[self.sp] = self.reg_b;
        self.sp -= 1;
        self.memory[self.sp] = self.reg_c;
        size
    }

    fn ld_a_d8(&mut self) -> u16 {
        let size = 2;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD A, 0x{:0>2X}", value), size);
        self.reg_a = value;
        size
    }

    fn ld_b_d8(&mut self) -> u16 {
        let size = 2;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD B, 0x{:0>2X}", value), size);
        self.reg_b = value;
        size
    }

    fn ld_c_d8(&mut self) -> u16 {
        let size = 2;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD C, 0x{:0>2X}", value), size);
        self.reg_c = value;
        size
    }

    fn ld_d_d8(&mut self) -> u16 {
        let size = 2;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD D, 0x{:0>2X}", value), size);
        self.reg_d = value;
        size
    }

    fn ld_e_d8(&mut self) -> u16 {
        let size = 2;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD E, 0x{:0>2X}", value), size);
        self.reg_e = value;
        size
    }

    fn ld_h_d8(&mut self) -> u16 {
        let size = 2;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD H, ? ; 0x{:0>2X}", value), size);
        self.reg_h = value;
        size
    }

    fn ld_l_d8(&mut self) -> u16 {
        let size = 2;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD L, 0x{:0>2X}", value), size);
        self.reg_l = value;
        size
    }

    fn a(&self) -> u8 { self.reg_a }
    fn b(&self) -> u8 { self.reg_b }
    fn c(&self) -> u8 { self.reg_c }
    fn d(&self) -> u8 { self.reg_d }
    fn e(&self) -> u8 { self.reg_e }
    fn h(&self) -> u8 { self.reg_h }
    fn l(&self) -> u8 { self.reg_l }

    fn ld_a(&mut self, value: u8) -> u16 {
        let size = 1;
        self.reg_a = value;
        self.print_disassembly(format!("LD A, ?; {:0>2X}", value), size);
        size
    }

    fn ld_b(&mut self, value: u8) -> u16 {
        let size = 1;
        self.reg_b = value;
        self.print_disassembly(format!("LD B, ?; {:0>2X}", value), size);
        size
    }

    fn ld_c(&mut self, value: u8) -> u16 {
        let size = 1;
        self.reg_c = value;
        self.print_disassembly(format!("LD C, ?; {:0>2X}", value), size);
        size
    }

    fn ld_d(&mut self, value: u8) -> u16 {
        let size = 1;
        self.reg_d = value;
        self.print_disassembly(format!("LD D, ?; {:0>2X}", value), size);
        size
    }

    fn ld_e(&mut self, value: u8) -> u16 {
        let size = 1;
        self.reg_e = value;
        self.print_disassembly(format!("LD E, ?; {:0>2X}", value), size);
        size
    }

    fn ld_h(&mut self, value: u8) -> u16 {
        let size = 1;
        self.reg_h = value;
        self.print_disassembly(format!("LD H, ?; {:0>2X}", value), size);
        size
    }

    fn ld_l(&mut self, value: u8) -> u16 {
        let size = 1;
        self.reg_l = value;
        self.print_disassembly(format!("LD L, ?; {:0>2X}", value), size);
        size
    }

    fn call(&mut self) -> u16 {
        let size = 3;
        let address = self.read_word(self.pc + 1);
        let return_address = self.pc + size;

        let return_address_high = (return_address >> 8) as u8 & 0xFF;
        let return_address_low = (return_address >> 0) as u8 & 0xFF;

        self.print_disassembly(format!(
            "CALL ${:0>4X} (from {:0>2X}{:0>2X})",
            address, return_address_high, return_address_low
        ), size);

        self.memory[self.sp] = return_address_low;
        self.sp -= 1;
        self.memory[self.sp] = return_address_high;
        self.sp -= 1;

        self.pc = address;
        0
    }

    fn ld_a_de(&mut self) -> u16 {
        let size = 1;
        let address = self.de();
        self.print_disassembly(format!("LD A,${:0>4X}", address), size);
        self.reg_a = self.memory[address];
        size
    }

    fn ld_de_d16(&mut self) -> u16 {
        let size = 3;
        self.reg_d = self.memory[self.pc + 2];
        self.reg_e = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD DE,${:0>2X}{:0>2X}", self.reg_d, self.reg_e), size);
        size
    }

    fn ld_hl_a(&mut self) -> u16 {
        let size = 1;
        let address = self.hl();
        self.print_disassembly(format!("LD 0x{:0>4X}, A", address), size);
        self.memory[address] = self.reg_a;
        size
    }

    fn inc_a(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("INC A"), size);
        self.reg_a = self.reg_a.wrapping_add(1);
        size
    }

    fn inc_b(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("INC B"), size);
        self.reg_b = self.reg_b.wrapping_add(1);
        size
    }

    fn inc_c(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("INC C"), size);
        self.reg_c = self.reg_c.wrapping_add(1);
        size
    }

    fn inc_d(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("INC D"), size);
        self.reg_d = self.reg_d.wrapping_add(1);
        size
    }

    fn inc_e(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("INC E"), size);
        self.reg_e = self.reg_e.wrapping_add(1);
        size
    }

    fn inc_h(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("INC H"), size);
        self.reg_h = self.reg_h.wrapping_add(1);
        size
    }

    fn inc_l(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("INC L"), size);
        self.reg_l = self.reg_l.wrapping_add(1);
        size
    }

    fn ldr_c_a(&mut self) -> u16 {
        let size = 1;
        let address = 0xFF00 + self.reg_c as u16;
        self.print_disassembly(format!("LD +${:0>2X}, {:0>2X}", self.reg_c, self.reg_a), size);
        self.memory[address] = self.reg_a;
        size
    }

    fn ld_a16_a(&mut self) -> u16 {
        let size = 3;
        let address = self.read_word(self.pc+1);
        self.print_disassembly(format!("LD ${:0>4X}, {:0>2X}", address, self.reg_a), size);
        self.memory[address] = self.reg_a;
        size
    }

    fn print_disassembly(&self, instruction: String, num_bytes: u16) {
        let mut s = String::new();
        for &byte in &self.memory[self.pc..self.pc + num_bytes] {
            write!(&mut s, "0x{:0>2X} ", byte).unwrap();
        }
        println!("[0x{:0>8X}] {:<15} {:<32} {:>16X}", self.pc, s, instruction, self.operations)
    }

    // OPERATIONS START HERE

    fn ret(&mut self) -> u16 {
        let addr_h = self.memory[self.sp+1];
        let addr_l = self.memory[self.sp+2];
        let return_address = (addr_h as u16) << 8 | addr_l as u16;
        self.print_disassembly(format!("RET ({:0>4X})", return_address), 1);

        self.pc = return_address;

         // move back "up" the stack, zeroing out
        self.sp += 1;
        self.sp = 0x00;
        self.sp += 1;
        self.sp = 0x00;

        0
    }

    /* when we jump to a new address, make sure to save the current program counter
     * address to the bottom of the stack, so when we can return to the current address
     */
    fn jmp_a16(&mut self) -> u16 {
        let address = self.read_word(self.pc + 1);
        self.print_disassembly(format!("JMP {:0>4X}", address), 3);
        self.pc = address;
        0
    }

    fn ld_hl_d16(&mut self) -> u16 {
        let size = 3;
        self.reg_l = self.memory[self.pc+1];
        self.reg_h = self.memory[self.pc+2];
        self.print_disassembly(format!("LD HL,${:0>2X}{:0>2X}", self.reg_h, self.reg_l), size);
        size
    }

    fn ld_sp_d16(&mut self) -> u16 {
        let size = 3;
        let address = self.read_word(self.pc + 1);
        self.print_disassembly(format!("LD SP,${:0>4X}", address), size);
        self.sp = address;
        size
    }

    fn de(&self) -> u16 {
        ((self.reg_d as u16) << 8) + ((self.reg_e as u16) << 0)
    }

    fn hl(&self) -> u16 {
        ((self.reg_h as u16) << 8) + ((self.reg_l as u16) << 0)
    }

    fn ldi_hl_a(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("LD (HL+) ({:0>2X}{:0>2X}), {:?}", self.reg_h, self.reg_l, self.reg_a), size);
        let address = self.hl();
        self.memory[address] = self.reg_a;
        self.store_hl(address.wrapping_add(1));
        size
    }

    fn store_hl(&mut self, address: u16) {
        self.reg_l = (address >> 0) as u8;
        self.reg_h = (address >> 8) as u8;
    }

    fn store_de(&mut self, address: u16) {
        self.reg_e = (address >> 0) as u8;
        self.reg_d = (address >> 8) as u8;
    }

    fn ldd_hl_a(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("LD (HL-) ({:0>2X}{:0>2X}), {:?}", self.reg_h, self.reg_l, self.reg_a), size);

        let address = self.hl();
        self.memory[address] = self.reg_a;

        self.store_hl(address.wrapping_sub(1));
        size
    }

    fn nop(&self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("NOP"), size);
        size
    }

    fn jr_r8(&mut self) -> u16 {
        let size = 2;
        let offset = self.memory[self.pc + 1] as i8;
        let address = self.pc.wrapping_add(offset as u16);
        self.print_disassembly(format!("JR $+{:0>2X} ; 0x{:0>4X}", offset, address + 1), size);

        self.pc = address;
        size
    }

    fn jr(&mut self, flag: Flag, zero: bool) -> u16 {
        let size = 2;
        let offset = self.memory[self.pc + 1] as i8;
        let address = self.pc.wrapping_add(offset as u16);
        let n = if zero { "" } else { "N" };
        self.print_disassembly(format!("JR {}Z, $+{:0>2X} ; 0x{:0>4X}", n, offset, address + 1), size);

        if self.get(flag) == zero {
            self.pc = address;
        }
        size
    }

    fn bit_h(&mut self, bit: u8) -> u16 {
        let size = 1;
        let h = self.reg_h;
        self.set(Flag::ZERO, h & (1<<bit) == 0);
        self.set(Flag::SUBTRACT, false);
        self.set(Flag::HALFCARRY, true);

        self.print_disassembly(format!("BIT {}, H", bit), size);
        size
    }

    // Thank you https://realboyemulator.wordpress.com/2013/01/03/a-look-at-the-game-boy-bootstrap-let-the-fun-begin/comment-page-1/
    fn xor_a(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("XOR A"), size);
        self.reg_a ^= self.reg_a;
        size
    }

    fn ldh_a_a8(&mut self) -> u16{
        let size = 2;
        let offset = self.memory[self.pc + 1];
        self.print_disassembly(format!("LDH A, (${:0>2X})", offset), size);

        let address = 0xFF00 + offset as u16;
        self.memory[address] = self.reg_a;

        size
    }

    fn ldh_a8_a(&mut self) -> u16{
        let size = 2;
        let offset = self.memory[self.pc + 1];
        self.print_disassembly(format!("LDH (${:0>2X}), A", offset), size);

        let address = 0xFF00 + offset as u16;
        let value = self.memory[address];
        self.reg_a = value;

        size
    }

    fn pop_bc(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("POP BC"), size);
        self.sp += 1;
        self.reg_b = self.memory[self.sp];
        self.memory[self.sp] = 0;

        self.sp += 1;
        self.reg_c = self.memory[self.sp];
        self.memory[self.sp] = 0;
        size
    }

    fn pop_hl(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("POP HL"), size);
        self.sp = self.sp + 1;
        self.reg_h = self.memory[self.sp];
        self.memory[self.sp] = 0;
        self.sp = self.sp + 1;
        self.reg_l = self.memory[self.sp];
        self.memory[self.sp] = 0;
        size
    }

    fn dec_a(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("DEC A"), size);

        let half = self.reg_a == 0;
        self.set(Flag::HALFCARRY, half);
        self.reg_a = self.reg_a.wrapping_sub(1);

        let zero = self.reg_a == 0;
        self.set(Flag::ZERO, zero);
        self.set(Flag::SUBTRACT, true);
        size
    }

    fn dec_b(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("DEC B"), size);

        let half = self.reg_b == 0;
        self.set(Flag::HALFCARRY, half);
        self.reg_b = self.reg_b.wrapping_sub(1);

        let zero = self.reg_b == 0;
        self.set(Flag::ZERO, zero);
        self.set(Flag::SUBTRACT, true);
        size
    }

    fn dec_c(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("DEC C"), size);

        let half = self.reg_c == 0;
        self.set(Flag::HALFCARRY, half);
        self.reg_c = self.reg_c.wrapping_sub(1);

        let zero = self.reg_c == 0;
        self.set(Flag::ZERO, zero);
        self.set(Flag::SUBTRACT, true);
        size
    }

    fn dec_d(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("DEC D"), size);

        let half = self.reg_d == 0;
        self.set(Flag::HALFCARRY, half);
        self.reg_d = self.reg_d.wrapping_sub(1);

        let zero = self.reg_d == 0;
        self.set(Flag::ZERO, zero);
        self.set(Flag::SUBTRACT, true);
        size
    }

    fn dec_e(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("DEC E"), size);

        let half = self.reg_e == 0;
        self.set(Flag::HALFCARRY, half);
        self.reg_e = self.reg_e.wrapping_sub(1);

        let zero = self.reg_e == 0;
        self.set(Flag::ZERO, zero);
        self.set(Flag::SUBTRACT, true);
        size
    }

    fn dec_h(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("DEC H"), size);

        let half = self.reg_h == 0;
        self.set(Flag::HALFCARRY, half);
        self.reg_h = self.reg_h.wrapping_sub(1);

        let zero = self.reg_h == 0;
        self.set(Flag::ZERO, zero);
        self.set(Flag::SUBTRACT, true);
        size
    }

    fn dec_l(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("DEC L"), size);

        let half = self.reg_l == 0;
        self.set(Flag::HALFCARRY, half);
        self.reg_l = self.reg_l.wrapping_sub(1);

        let zero = self.reg_l == 0;
        self.set(Flag::ZERO, zero);
        self.set(Flag::SUBTRACT, true);
        size
    }

    fn inc_de(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("INC DE"), size);
        let value = self.de();
        self.store_de(value.wrapping_add(1));
        size
    }

    fn inc_hl(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("INC HL"), size);
        let value = self.hl();
        self.store_hl(value.wrapping_add(1));
        size
    }

    fn cp_d8(&mut self) -> u16 {
        let size = 2;
        let a = self.reg_a;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("CP 0x{:0>2X}", value), size);
        self.set(Flag::ZERO, a == value);
        self.set(Flag::SUBTRACT, true);
        self.set(Flag::HALFCARRY, (a << 4) < (value << 4));
        self.set(Flag::CARRY, a < value);
        if value==0x90 {
            self.crash(format!("HEY WAT IS GOING ON"))
        }
        size
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f,
            "cpu {{\
            \n\tpc: {pc:0>4X} [{i0:0>2X} {i1:0>2X} {i2:0>2X} {i3:0>2X}]\
            \n\tsp: {sp:0>4X}\
            \n\tregisters: {{ a: {a:0>2X}, f: {f:0>2X}, b: {b:0>2X}, c: {c:0>2X}, d: {d:0>2X}, e: {e:0>2X}, h: {h:0>2X}, l: {l:0>2X} }}\
            \n\tflags: {{ zero: {zero}, sub: {sub}, half: {half}, carry: {carry} }}
            \n}}
            ",
                pc=self.pc, i0=self.memory[self.pc+0], i1=self.memory[self.pc+1], i2=self.memory[self.pc+2], i3=self.memory[self.pc+3],
                sp=self.sp,
                a=self.reg_a, f=self.reg_f, b=self.reg_b, c=self.reg_c, d=self.reg_d, e=self.reg_e, h=self.reg_h, l=self.reg_l,
                zero=self.get(Flag::ZERO), sub=self.get(Flag::SUBTRACT), half=self.get(Flag::HALFCARRY), carry=self.get(Flag::CARRY)

        ));

        self.print_stack_and_vram(8);
        Ok(())
    }
}
