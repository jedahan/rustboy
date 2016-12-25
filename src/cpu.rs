use std::fmt;

use lcd;
use memory;
use std::sync::{Arc, RwLock};
use std::fmt::Write;
use window::Drawable;

#[repr(u8)]
pub enum Flag {
    ZERO = 1 << 7,
    SUBTRACT = 1 << 6,
    HALFCARRY = 1 << 5,
    CARRY = 1 << 4,
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
    ime: bool,
    memory: Arc<RwLock<memory::Memory>>,
    screen: lcd::LcdScreen,
    operations: usize,
    running: bool
}

impl Cpu {
    fn print_stack_and_vram(&self, height: usize) {
        println!("mem {{\n  stack:\tvram:");
        for depth in 0..height {
            let byte = 0xFFFF - depth as u16;
            let arrow = if self.sp == byte {
                "❯"
            } else {
                " "
            };

            let memory = self.memory.read().unwrap();
            println!("{}   0x{:0>4X}: {:0>2X} \t  0x{:0>4X}: {:0>2X} \t\t",
                     arrow,
                     byte,
                     memory[byte],
                     byte - 0x6000,
                     memory[byte - 0x6000])
        }
        println!("}}");

    }

    fn crash(&mut self, message: String) -> u16 {
        println!("{:0>4X}: {}", self.operations, self);
        self.print_stack_and_vram(0xFF);
        println!("{}", message);
        self.running = false;
        0
    }

    /**
     * Should I set this on new() or run()?
     * AF 01B0h
     * BC 0013h
     * DE 00D8h
     * HL 014Dh
     * SP FFFEh
     * PC 0100h
     **/
    pub fn new(memory: Arc<RwLock<memory::Memory>>) -> Cpu {
        let memory_ref = memory.clone();
        let screen = lcd::LcdScreen::new(160, 144, memory_ref);
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

            ime: false,

            running: false,
            operations: 0,

            memory: memory,
            screen: screen,
        }
    }

    /** Interrupt master enable.
     * This flag is not mapped to memory and can't be read by any means.
     * The meaning of the flag is not to enable or disable interrupts.
     * In fact, what it does is enable the jump to the interrupt vectors.
     * IME can only be set to '1' by the instructions EI and RETI,
     * and can only be set to '0' by DI (and the CPU when jumping to an interrupt vector).
     *
     * Note that EI doesn't enable the interrupts the same cycle it is executed, but the next cycle!
     *
     *   di
     *   ld  a,IEF_TIMER
     *   ld  [rIE],a
     *   ld  [rIF],a
     *   ei
     *   inc a ; This is still executed before jumping to the interrupt vector.
     *   inc a ; This is executed after returning.
     *   ld   [hl+],a
     **/
    fn ime(&mut self, enable: bool) -> bool {
        self.ime = enable;
        self.ime
    }


    pub fn service(&mut self, vector: u16) {
        println!("In where we service vector 0x{:0>4X}", vector);
    }


    // Not sure if this is little-endian or big-endian
    fn read_word(&self, address: u16) -> u16 {
        let memory = self.memory.read().unwrap();
        (memory[address + 1] as u16) << 8 | (memory[address + 0] as u16)
    }

    // It takes 20 clocks to dispatch an interrupt.
    // If CPU is in HALT mode, another extra 4 clocks are needed.
    // These timings are the same in every Game Boy model or in double/single speeds in CGB/AGB/AGS.
    //
    // 1. Two wait states are executed (2 machine cycles pass while nothing occurs, presumably the CPU is executing NOPs during this time).
    // 2. The current PC is pushed onto the stack, this process consumes 2 more machine cycles.
    // 3. The high byte of the PC is set to 0, the low byte is set to the address of the handler
    // ($40,$48,$50,$58,$60). This consumes one last machine cycle.
    // The entire ISR should consume a total of 5 machine cycles. This has yet to be tested, but is what the Z80 datasheet implies.
    pub fn service_interrupts(&mut self) {
        // If any IF flag and the corresponding IE flag are both '1' and IME is set to '1' too, the CPU will push the current PC into the stack, will jump to the corresponding interrupt vector and set IME to '0'. If IME is '0', this won't happen.
        // TODO: what does 'this' refer to?
        // If IME='0' and CPU is halted, when any interrupt is triggered by setting any IF flag to '1' with the corresponding bit in IE set to '1', it takes 4 clocks to exit halt mode, even if the CPU doesn't jump to the interrupt vector.
        if self.ime {
            self.sp = self.pc;
            self.sp += 1;

            // service the lowest bit (highest priority) interrupt
            let interrupts: u8 = self.interrupt_enable() & self.interrupt_flag();
            // TCAGBD 2.2
            self.service(0x0040 + (8 * (interrupts.trailing_zeros() as u16)));
        }
    }

    // interrupt flag / interrupt enable
    //
    // Bit 4 – Joypad Interrupt Requested
    // Bit 3 – Serial Interrupt Requested
    // Bit 2 – Timer Interrupt Requested
    // Bit 1 – LCD STAT Interrupt Requested
    // Bit 0 – Vertical Blank Interrupt Requested (1=Requested)
    fn interrupt_flag(&self) -> u8 {
        self.memory.read().unwrap()[0xFFFFu16] & 0b00011111
    }

    fn interrupt_enable(&self) -> u8 {
        self.memory.read().unwrap()[0xFF0Fu16] & 0b00011111
    }

    pub fn run(&mut self) {
        println!("Cpu::run");
        self.running = true;

        while self.running {
            // interrupts are serviced before fetching the next instruction
            self.service_interrupts();

            let instruction = self.fetch();
            let advance = self.execute(instruction);
            self.pc += advance;
            self.operations += 1;
            self.screen.update();

            match self.operations {
                0x0001 => {
                    self.screen.run();
                    println!("MAIN SCREEN TURN ON");
                },
                0x7090 => {
                    self.running = false;
                    println!("{}", self);
                },
                _ => ()
            }

            match self.pc {
                0x0000 => println!("START CLEAR VRAM"),
                0x000C => println!("END CLEAR VRAM\n  START AUDIO"),
                0x001D => println!("END AUDIO\n  START LOGO"),
                0x00E0 => println!("END LOGO\n  START CHECKSUM"),
                _ => ()
            }
        }
    }

    fn fetch(&mut self) -> (u8, u8) {
        let memory = self.memory.read().unwrap();
        let opcode = memory[self.pc];
        if opcode == 0xCB {
            self.pc += 1;

            return (opcode, memory[self.pc]);
        }
        (0, opcode)
    }

    fn execute(&mut self, instruction: (u8, u8)) -> u16 {
        match instruction {
            // z80prefix
            (0xCB, opcode) => {
                match opcode {
                    0x7C => self.bit_h(7),
                    0x11 => self.rl_c(),
                    _ => self.crash(format!("unrecognized z80 opcode {:0>2X}", opcode)),
                }
            }
            (_, opcode) => {
                match opcode {
                    0x00 => self.nop(),

                    // math
                    0x0C => self.inc_c(),
                    0x1C => self.inc_e(),
                    0x2C => self.inc_l(),
                    0x3C => self.inc_a(),
                    0x04 => self.inc_b(),
                    0x14 => self.inc_d(),
                    0x24 => self.inc_h(),

                    0x13 => self.inc_de(),
                    0x23 => self.inc_hl(),

                    0x0D => self.dec_c(),
                    0x1D => self.dec_e(),
                    0x2D => self.dec_l(),
                    0x3D => self.dec_a(),
                    0x05 => self.dec_b(),
                    0x15 => self.dec_d(),
                    0x25 => self.dec_h(),

                    0xAF => self.xor_a(),

                    0x17 => self.rla(),

                    // flow
                    0x18 => self.jr_r8(),
                    0x28 => self.jr(Flag::ZERO, true),
                    0x20 => self.jr(Flag::ZERO, false),
                    0xC3 => self.jmp_a16(),

                    0xCD => self.call(),

                    0xC9 => self.ret(),

                    // stack
                    0xFE => self.cp_d8(),

                    0xC5 => self.push_bc(),

                    0xC1 => self.pop_bc(),
                    0xE1 => self.pop_hl(),

                    // loading
                    // TODO: replace with macro
                    // ld_a
                    0x7F => self.ld("a", "a"),
                    0x78 => self.ld("a", "b"),
                    0x79 => self.ld("a", "c"),
                    0x7A => self.ld("a", "d"),
                    0x7B => self.ld("a", "e"),
                    0x7C => self.ld("a", "h"),
                    0x7D => self.ld("a", "l"),
                    // ld_b
                    0x47 => self.ld("b", "a"),
                    0x40 => self.ld("b", "b"),
                    0x41 => self.ld("b", "c"),
                    0x42 => self.ld("b", "d"),
                    0x43 => self.ld("b", "e"),
                    0x44 => self.ld("b", "h"),
                    0x45 => self.ld("b", "l"),
                    // ld_c
                    0x4F => self.ld("c", "a"),
                    0x48 => self.ld("c", "b"),
                    0x49 => self.ld("c", "c"),
                    0x4A => self.ld("c", "d"),
                    0x4B => self.ld("c", "e"),
                    0x4c => self.ld("c", "h"),
                    0x4D => self.ld("c", "l"),
                    // ld_d
                    0x57 => self.ld("d", "a"),
                    0x50 => self.ld("d", "b"),
                    0x51 => self.ld("d", "c"),
                    0x52 => self.ld("d", "d"),
                    0x53 => self.ld("d", "e"),
                    0x54 => self.ld("d", "h"),
                    0x55 => self.ld("d", "l"),
                    // ld_e
                    0x5F => self.ld("e", "a"),
                    0x58 => self.ld("e", "b"),
                    0x59 => self.ld("e", "c"),
                    0x5A => self.ld("e", "d"),
                    0x5B => self.ld("e", "e"),
                    0x5C => self.ld("e", "h"),
                    0x5D => self.ld("e", "l"),
                    // ld_h
                    0x67 => self.ld("h", "a"),
                    0x60 => self.ld("h", "b"),
                    0x61 => self.ld("h", "c"),
                    0x62 => self.ld("h", "d"),
                    0x63 => self.ld("h", "e"),
                    0x64 => self.ld("h", "h"),
                    0x65 => self.ld("h", "l"),
                    // ld_l
                    0x6F => self.ld("l", "a"),
                    0x68 => self.ld("l", "b"),
                    0x69 => self.ld("l", "c"),
                    0x6A => self.ld("l", "d"),
                    0x6B => self.ld("l", "e"),
                    0x6C => self.ld("l", "h"),
                    0x6D => self.ld("l", "l"),

                    0x1A => self.ld_a_de(),

                    0x3E => self.ld_a_d8(),
                    0x06 => self.ld_b_d8(),
                    0x0E => self.ld_c_d8(),
                    0x16 => self.ld_d_d8(),
                    0x1E => self.ld_e_d8(),
                    0x26 => self.ld_h_d8(),
                    0x2E => self.ld_l_d8(),

                    0x77 => self.ld_hl_a(),

                    0x11 => self.ld_de_d16(),
                    0x21 => self.ld_hl_d16(),
                    0x31 => self.ld_sp_d16(),

                    0x22 => self.ldi_hl_a(),

                    0x32 => self.ldd_hl_a(),

                    0xE0 => self.ldh_a8_a(),
                    0xF0 => self.ldh_a_a8(),

                    0xE2 => self.ldr_c_a(),
                    0xEA => self.ld_a16_a(),
                    _ => {
                        println!("unrecognized opcode {:0>2X}", opcode);
                        0
                    }
                }
            }
        }
    }

    pub fn get(&self, flag: Flag) -> bool {
        self.reg_f & flag as u8 != 0
    }

    pub fn set(&mut self, flag: Flag, set: bool) {
        if set {
            self.reg_f |= flag as u8;
        } else {
            self.reg_f &= !(flag as u8);
        }
    }

    fn rla(&mut self) -> u16 {
        let size = 1;
        let amount = { self.memory.read().unwrap()[self.pc + 1] };
        self.print_disassembly(format!("RLA ({})", amount), size);
        if self.reg_a & 0b10000000 != 0 {
            self.set(Flag::CARRY, true)
        }
        self.reg_a = self.reg_a.rotate_left(amount as u32);
        size
    }

    fn rl_c(&mut self) -> u16 {
        let size = 1;
        let amount = { self.memory.read().unwrap()[self.pc + 1] };
        self.print_disassembly(format!("RL C ({})", amount), size);
        self.reg_c.rotate_left(amount as u32);
        size
    }

    fn push_bc(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("PUSH BC ${:0>2X}{:0>2X}", self.reg_b, self.reg_c),
                               size);
        self.sp -= 1;
        let mut memory = self.memory.write().unwrap();
        memory[self.sp] = self.reg_b;
        self.sp -= 1;
        memory[self.sp] = self.reg_c;
        size
    }

    fn ld_a_d8(&mut self) -> u16 {
        let size = 2;
        let memory = self.memory.read().unwrap();
        let value = memory[self.pc + 1];
        self.print_disassembly(format!("LD A, 0x{:0>2X}", value), size);
        self.reg_a = value;
        size
    }

    fn ld_b_d8(&mut self) -> u16 {
        let size = 2;
        let memory = self.memory.read().unwrap();
        let value = memory[self.pc + 1];
        self.print_disassembly(format!("LD B, 0x{:0>2X}", value), size);
        self.reg_b = value;
        size
    }

    // TODO: replace with macro!
    fn ld(&mut self, to: &'static str, from: &'static str) -> u16 {
        let size = 1;
        let value = match from {
            "a" => self.reg_a,
            "b" => self.reg_b,
            "c" => self.reg_c,
            "d" => self.reg_d,
            "e" => self.reg_e,
            "h" => self.reg_h,
            "l" => self.reg_l,
            _ => panic!("'{}' does not match a register", from)
        };

        match to {
            "a" => self.reg_a = value,
            "b" => self.reg_b = value,
            "c" => self.reg_c = value,
            "d" => self.reg_d = value,
            "e" => self.reg_e = value,
            "h" => self.reg_h = value,
            "l" => self.reg_l = value,
            _ => panic!("'{}' does not match a register", to)
        };

        self.print_disassembly(format!("LD {}, {}; {:0>2X}", from, to, value), size);

        size
    }

    fn ld_c_d8(&mut self) -> u16 {
        let size = 2;
        let memory = self.memory.read().unwrap();
        let value = memory[self.pc + 1];
        self.print_disassembly(format!("LD C, 0x{:0>2X}", value), size);
        self.reg_c = value;
        size
    }

    fn ld_d_d8(&mut self) -> u16 {
        let size = 2;
        let memory = self.memory.read().unwrap();
        let value = memory[self.pc + 1];
        self.print_disassembly(format!("LD D, 0x{:0>2X}", value), size);
        self.reg_d = value;
        size
    }

    fn ld_e_d8(&mut self) -> u16 {
        let size = 2;
        let memory = self.memory.read().unwrap();
        let value = memory[self.pc + 1];
        self.print_disassembly(format!("LD E, 0x{:0>2X}", value), size);
        self.reg_e = value;
        size
    }

    fn ld_h_d8(&mut self) -> u16 {
        let size = 2;
        let memory = self.memory.read().unwrap();
        let value = memory[self.pc + 1];
        self.print_disassembly(format!("LD H, ? ; 0x{:0>2X}", value), size);
        self.reg_h = value;
        size
    }

    fn ld_l_d8(&mut self) -> u16 {
        let size = 2;
        let memory = self.memory.read().unwrap();
        let value = memory[self.pc + 1];
        self.print_disassembly(format!("LD L, 0x{:0>2X}", value), size);
        self.reg_l = value;
        size
    }

    fn a(&self) -> u8 {
        self.reg_a
    }
    fn b(&self) -> u8 {
        self.reg_b
    }
    fn c(&self) -> u8 {
        self.reg_c
    }
    fn d(&self) -> u8 {
        self.reg_d
    }
    fn e(&self) -> u8 {
        self.reg_e
    }
    fn h(&self) -> u8 {
        self.reg_h
    }
    fn l(&self) -> u8 {
        self.reg_l
    }

    fn call(&mut self) -> u16 {
        let size = 3;
        let address = self.read_word(self.pc + 1);
        let return_address = self.pc + size;

        let return_address_high = (return_address >> 8) as u8 & 0xFF;
        let return_address_low = (return_address >> 0) as u8 & 0xFF;

        self.print_disassembly(format!("CALL ${:0>4X} (from {:0>2X}{:0>2X})",
                                       address,
                                       return_address_high,
                                       return_address_low),
                               size);

        let mut memory = self.memory.write().unwrap();
        memory[self.sp] = return_address_low;
        self.sp = self.sp.wrapping_sub(1);
        memory[self.sp] = return_address_high;
        self.sp = self.sp.wrapping_sub(1);

        self.pc = address;
        0
    }

    fn ld_a_de(&mut self) -> u16 {
        let size = 1;
        let address = self.de();
        self.print_disassembly(format!("LD A, DE ; DE=${:0>4X}", address), size);
        let memory = self.memory.read().unwrap();
        self.reg_a = memory[address];
        size
    }

    fn ld_de_d16(&mut self) -> u16 {
        let size = 3;
        let memory = self.memory.read().unwrap();
        self.reg_d = memory[self.pc + 2];
        self.reg_e = memory[self.pc + 1];
        self.print_disassembly(format!("LD DE,${:0>2X}{:0>2X}", self.reg_d, self.reg_e),
                               size);
        size
    }

    fn ld_hl_a(&mut self) -> u16 {
        let size = 1;
        let address = self.hl();
        self.print_disassembly(format!("LD [HL], A ; HL=0x{:0>4X}, A={:0>2X}", address, self.reg_a), size);
        let mut memory = self.memory.write().unwrap();
        memory[address] = self.reg_a;
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
        self.print_disassembly(format!("LD [C],A; C=+${:0>2X}, A={:0>2X}", self.reg_c, self.reg_a),
                               size);
        let mut memory = self.memory.write().unwrap();
        memory[address] = self.reg_a;
        size
    }

    fn ld_a16_a(&mut self) -> u16 {
        let size = 3;
        let address = self.read_word(self.pc + 1);
        self.print_disassembly(format!("LD ${:0>4X}, {:0>2X}", address, self.reg_a), size);
        let mut memory = self.memory.write().unwrap();
        memory[address] = self.reg_a;
        size
    }

    fn print_disassembly(&self, instruction: String, num_bytes: u16) {
        let mut s = String::new();

        let memory = self.memory.read().unwrap();
        for &byte in &memory[self.pc..self.pc + num_bytes] {
            write!(&mut s, "0x{:0>2X} ", byte).unwrap();
        }
        println!("[0x{:0>8X}] {:<15} {:<32} {:>16X}",
                 self.pc,
                 s,
                 instruction,
                 self.operations)
    }

    // OPERATIONS START HERE

    fn ret(&mut self) -> u16 {
        let memory = { self.memory.read().unwrap() };
        let addr_h = memory[self.sp.wrapping_add(1)];
        let addr_l = memory[self.sp.wrapping_add(2)];
        let return_address = (addr_h as u16) << 8 | addr_l as u16;
        self.print_disassembly(format!("RET ({:0>4X})", return_address), 1);

        self.pc = return_address;

        // move back "up" the stack, zeroing out
        self.sp = self.sp.wrapping_add(1);
        self.sp = 0x00;
        self.sp = self.sp.wrapping_add(1);
        self.sp = 0x00;

        0
    }

    // when we jump to a new address, make sure to save the current program counter
    // address to the bottom of the stack, so when we can return to the current address
    //
    fn jmp_a16(&mut self) -> u16 {
        let address = self.read_word(self.pc.wrapping_add(1));
        self.print_disassembly(format!("JMP {:0>4X}", address), 3);
        self.pc = address;
        0
    }

    fn ld_hl_d16(&mut self) -> u16 {
        let size = 3;
        let memory = self.memory.read().unwrap();
        self.reg_l = memory[self.pc + 1];
        self.reg_h = memory[self.pc + 2];
        self.print_disassembly(format!("LD HL,${:0>2X}{:0>2X}", self.reg_h, self.reg_l),
                               size);
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
        self.print_disassembly(format!("LD (HL+) ({:0>2X}{:0>2X}), {:?}",
                                       self.reg_h,
                                       self.reg_l,
                                       self.reg_a),
                               size);
        let address = self.hl();
        self.memory.write().unwrap()[address] = self.reg_a;
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
        self.print_disassembly(format!("LD [HLD],A ; HL=({:0>2X}{:0>2X}), A={:?}",
                                       self.reg_h,
                                       self.reg_l,
                                       self.reg_a),
                               size);

        let address = self.hl();
        self.memory.write().unwrap()[address] = self.reg_a;

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
        let memory = self.memory.read().unwrap();
        let offset = memory[self.pc + 1] as i8;
        let address = self.pc.wrapping_add(offset as u16);
        self.print_disassembly(format!("JR $+{:0>2X} ; 0x{:0>4X}", offset, address + 1),
                               size);

        self.pc = address;
        size
    }

    fn jr(&mut self, flag: Flag, zero: bool) -> u16 {
        let size = 2;
        let memory = self.memory.read().unwrap();
        let offset = memory[self.pc + 1] as i8;
        let address = self.pc.wrapping_add(offset as u16);
        let n = if zero {
            ""
        } else {
            "N"
        };
        self.print_disassembly(format!("JR {}Z, $+{:0>2X} ; 0x{:0>4X}", n, offset, address + 1),
                               size);

        if self.get(flag) == zero {
            self.pc = address;
        }
        size
    }

    fn bit_h(&mut self, bit: u8) -> u16 {
        let size = 1;
        let h = self.reg_h;
        let zero = h & (1 << bit) == 0;

        self.set(Flag::ZERO, zero);
        self.set(Flag::SUBTRACT, false);
        self.set(Flag::HALFCARRY, true);

        self.print_disassembly(format!("BIT {}, H; {}", bit, zero), size);
        size
    }

    // Thank you https://realboyemulator.wordpress.com/2013/01/03/a-look-at-the-game-boy-bootstrap-let-the-fun-begin/comment-page-1/
    fn xor_a(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("XOR A"), size);
        self.reg_a ^= self.reg_a;
        size
    }

    fn ldh_a_a8(&mut self) -> u16 {
        let size = 2;
        let memory = self.memory.read().unwrap();
        let offset = memory[self.pc + 1];
        self.print_disassembly(format!("LDH A, (${:0>2X})", offset), size);

        let address = 0xFF00 + offset as u16;
        let mut memory = self.memory.write().unwrap();
        memory[address] = self.reg_a;

        size
    }

    fn ldh_a8_a(&mut self) -> u16 {
        let size = 2;
        let memory = self.memory.read().unwrap();
        let offset = memory[self.pc + 1];
        self.print_disassembly(format!("LDH (${:0>2X}), A", offset), size);

        let address = 0xFF00 + offset as u16;
        let value = memory[address];
        self.reg_a = value;

        size
    }

    fn pop_bc(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("POP BC"), size);
        self.sp += 1;
        let mut memory = self.memory.write().unwrap();
        self.reg_b = memory[self.sp];
        memory[self.sp] = 0;

        self.sp += 1;
        self.reg_c = memory[self.sp];
        memory[self.sp] = 0;
        size
    }

    fn pop_hl(&mut self) -> u16 {
        let size = 1;
        self.print_disassembly(format!("POP HL"), size);
        self.sp += 1;
        let mut memory = self.memory.write().unwrap();
        self.reg_h = memory[self.sp];
        memory[self.sp] = 0;
        self.sp += 1;
        self.reg_l = memory[self.sp];
        memory[self.sp] = 0;
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
        let value = self.memory.read().unwrap()[self.pc + 1];
        self.print_disassembly(format!("CP 0x{:0>2X}", value), size);
        self.set(Flag::ZERO, a == value);
        self.set(Flag::SUBTRACT, true);
        self.set(Flag::HALFCARRY, (a << 4) < (value << 4));
        self.set(Flag::CARRY, a < value);
        size
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let memory = self.memory.read().unwrap();
        try!(writeln!(f,
                      "cpu {{\n\tpc: {pc:0>4X} [{i0:0>2X} {i1:0>2X} {i2:0>2X} {i3:0>2X}]\n\tsp: \
                       {sp:0>4X}\n\tregisters: {{ a: {a:0>2X}, f: {f:0>2X}, b: {b:0>2X}, c: \
                       {c:0>2X}, d: {d:0>2X}, e: {e:0>2X}, h: {h:0>2X}, l: {l:0>2X} \
                       }}\n\tflags: {{ zero: {zero}, sub: {sub}, half: {half}, carry: {carry} \
                   }}\n}}
                       \nscreen {{ \
                       \n\t{screen}\
                   }}
            ",
                      pc = self.pc,
                      i0 = memory[self.pc + 0],
                      i1 = memory[self.pc + 1],
                      i2 = memory[self.pc + 2],
                      i3 = memory[self.pc + 3],
                      sp = self.sp,
                      a = self.reg_a,
                      f = self.reg_f,
                      b = self.reg_b,
                      c = self.reg_c,
                      d = self.reg_d,
                      e = self.reg_e,
                      h = self.reg_h,
                      l = self.reg_l,
                      zero = self.get(Flag::ZERO),
                      sub = self.get(Flag::SUBTRACT),
                      half = self.get(Flag::HALFCARRY),
                      carry = self.get(Flag::CARRY),
                      screen = self.screen));

        self.print_stack_and_vram(8);
        Ok(())
    }
}
