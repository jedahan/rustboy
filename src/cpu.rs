use std::fmt;

use screen;
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
    screen: screen::Screen,
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

    pub fn new(memory: memory::Memory, screen: screen::Screen) -> Cpu {
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
                    0x06 => { self.ld_b_d8() }
                    0x0C => { self.inc_c() }
                    0x0E => { self.ldd_c_d8() }
                    0x11 => { self.ld_de_d16() }
                    0x1A => { self.ld_a_de() }
                    0x20 => { self.jr_nz() }
                    0x21 => { self.ld_hl_d16() }
                    0x31 => { self.ld_sp_d16() }
                    0x32 => { self.ldd_d16_a() }
                    0x3E => { self.ldd_a_d8() }
                    0x4F => { self.ld_c_a() }
                    0x77 => { self.ld_hl_a() }
                    0xAF => { self.xor_a() }
                    0xC3 => { self.jmp_a16() }
                    0xC5 => { self.push_bc() }
                    0xCD => { self.call() }
                    0xE0 => { self.ldh_a8_a() }
                    0xE1 => { self.pop_hl() }
                    0xE2 => { self.load_relative_c_a() }
                    0xC9 => { self.ret() }
                    _ => {
                        self.screen.debug(&self.memory);
                        0
                        //self.crash(format!("unrecognized opcode {:0>2X}", opcode))
                    }
                }
            }
        }
    }

    pub fn get(&self, flag: Flag) -> bool { self.reg_f & flag.to_u8() != 0 }
    pub fn set(&mut self, flag: Flag) { self.reg_f |= flag.to_u8(); }
    pub fn unset(&mut self, flag: Flag) { self.reg_f &= ! flag.to_u8(); }

    fn rl_c(&mut self) -> u16 {
        let size = 2;
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

    fn ld_b_d8(&mut self) -> u16 {
        let size = 2;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD B, 0x{:0>2X}", value), size);
        self.reg_b = value;
        size
    }

    fn ld_c_a(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("LD C, A"), size);
        self.reg_c = self.reg_a;
        size
    }

    fn call(&mut self) -> u16 {
        let size = 3;
        let address = self.read_word(self.pc + 1);
        self.print_disassembly(format!("CALL ${:0>4X}", address), size);

        self.sp -= 1;
        self.memory[self.sp] = (self.pc >> 8) as u8 & 0xFF;
        self.sp -= 1;
        self.memory[self.pc] = (self.pc >> 0) as u8 & 0xFF;

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

    fn inc_c(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("INC C"), size);
        self.reg_c = self.reg_c.wrapping_add(1);
        size
    }

    fn load_relative_c_a(&mut self) -> u16 {
        let size = 1;
        let value = 0xFF00 + self.reg_c as u16;
        self.print_disassembly(format!("LD +${:0>2X}, {:0>2X}", self.reg_c, self.reg_a), size);
        self.memory[value] = self.reg_a;
        size
    }

    fn ldd_a_d8(&mut self) -> u16 {
        let size = 2;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD A,${:0>2X}", value), size);
        self.reg_a = value;
        size
    }

    fn ldd_c_d8(&mut self) -> u16 {
        let size = 2;
        let value = self.memory[self.pc + 1];
        self.print_disassembly(format!("LD C,${:0>2X}", value), size);
        self.reg_c = value;
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
        self.pc = self.sp;
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


    // load reg into the address (HL), then decrement hl
    // TODO: check that we are using the zero flag correctly
    fn ldd_d16_a(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("LD (HL-) ({:0>2X}{:0>2X}), {:?}", self.reg_h, self.reg_l, self.reg_a), size);

        let mut address = self.hl();
        self.memory[address] = self.reg_a;

        if address - 1 == 0x0000 {
            self.set(Flag::ZERO);
        } else {
            address -= 1;
        }
        self.reg_l = (address >> 0) as u8;
        self.reg_h = (address >> 8) as u8;
        size
    }

    fn nop(&self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("NOP"), size);
        size
    }

    fn jr_nz(&mut self) -> u16 {
        let size = 2;
        if ! self.get(Flag::ZERO) {
            let offset = self.memory[self.pc + 1] as i8;
            let address = self.pc.wrapping_add(offset as u16);
            self.print_disassembly(format!("JR NZ, $+{:0>2X} ; 0x{:0>4X}", offset, address + 1), size);
            self.pc = address;

            return 2
        }
        size
    }

    fn bit_h(&mut self, bit: u8) -> u16 {
        let size = 1;
        self.unset(Flag::SUBTRACT);
        self.set(Flag::HALFCARRY);

        if self.reg_h & (1<<bit) == 0 {
            self.set(Flag::ZERO);
        } else {
            self.unset(Flag::ZERO);
        }

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

    fn ldh_a8_a(&mut self) -> u16{
        let size = 2;
        let offset = self.memory[self.pc + 1];
        self.print_disassembly(format!("LDH (${:0>2X}), A", offset), size);

        let address = 0xFF00 + offset as u16;
        let value = self.memory[address];
        self.reg_a = value;

        size
    }

    fn pop_hl(&mut self) -> u16 {
        println!("POP HL");
        self.sp = self.sp + 1;
        self.reg_h = self.memory[self.sp];
        self.memory[self.sp] = 0;
        self.sp = self.sp + 1;
        self.reg_l = self.memory[self.sp];
        self.memory[self.sp] = 0;
        1
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
