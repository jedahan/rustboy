use gameboy;
use cart;
const RAM_SIZE: usize = 8 * 1024;
const XRAM_SIZE: usize = 0x1FFF;
const HRAM_SIZE: usize = 0x007E;

use std::ops::Index;
//use std::ops::Range;
use std::collections::HashMap;

pub struct Interconnect {
    boot: [u8; gameboy::BOOTROM_SIZE],
    cart: cart::Cart,
    wram: [u8; RAM_SIZE],
    vram: [u8; RAM_SIZE],
    xram: [u8; XRAM_SIZE],
    input: u8,
    hram: [u8; HRAM_SIZE],
    interrupt: bool
}

impl Interconnect {
    pub fn new(boot: [u8; gameboy::BOOTROM_SIZE], cart: cart::Cart) -> Interconnect {
        Interconnect {
            boot: boot,
            cart: cart,
            wram: [0; RAM_SIZE],
            vram: [0; RAM_SIZE],
            xram: [0; XRAM_SIZE],
            input: 0,
            hram: [0; HRAM_SIZE],
            interrupt: false
        }
    }
}

impl Index<u16> for Interconnect {
    type Output = u8;

    fn index<'a>(&'a self, index: u16) -> &Self::Output {
        if index < 0x0100 {
            &self.boot[index as usize]
        } else {
            &self.cart[index]
        }
    }
}

/*
    pub fn read(&self, index: u16) -> u8 {
        if index < 0x0100 {
            self.boot[index as usize]
        } else {
            self.cart[(index - 0x0100) as usize]
        }
        /*
        map.insert(0x0000..0x0100, &self.boot);
        map.insert(0x0100..0x7FFF, &self.cart);
        map.insert(0x8000..0x9FFF, &self.vram);
        map.insert(0xA000..0xBFFF, &self.xram);
        map.insert(0xC000..0xDFFF, &self.wram );
        map.insert(0xE000..0xFDFF, &self.wram);
        map.insert(0xFF00..0xFF00, &self.input);
        map.insert(0xFF80..0xFFFE, &self.hram);
        map.insert(0xFFFF..0xFFFF, &self.interrupt);
        */

    }
        */
