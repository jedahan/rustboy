use gameboy;
use cart;
const RAM_SIZE: usize = 0x0200;
const VRAM_SIZE: usize = 0x2000;
const XRAM_SIZE: usize = 0x1FFF;
const HRAM_SIZE: usize = 0x007F;

use std::ops::{Index, IndexMut};

pub struct Interconnect {
    boot: [u8; gameboy::BOOTROM_SIZE],
    cart: cart::Cart,
    wram: [u8; RAM_SIZE],
    vram: [u8; VRAM_SIZE],
    xram: [u8; XRAM_SIZE],
    input: [u8; 1],
    hram: [u8; HRAM_SIZE],
    interrupt: [u8; 1]
}

impl Interconnect {
    pub fn new(boot: [u8; gameboy::BOOTROM_SIZE], cart: cart::Cart) -> Interconnect {
        Interconnect {
            boot: boot,
            cart: cart,
            wram: [0; RAM_SIZE],
            vram: [0; VRAM_SIZE],
            xram: [0; XRAM_SIZE],
            input: [0],
            hram: [0; HRAM_SIZE],
            interrupt: [0]
        }
    }
}

impl Index<u16> for Interconnect {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<usize> for Interconnect {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0x0000...0x00FF => &self.boot[index - 0x0000],
            0x0100...0x7FFF => &self.cart[index - 0x0100],
            0x8000...0x9FFF => &self.vram[index - 0x8000],
            0xA000...0xBFFF => &self.xram[index - 0xA000],
            0xC000...0xDFFF => &self.wram[index - 0xC000],
            0xE000...0xFDFF => &self.wram[index - 0xE000],
            0xFF00 => &self.input[index - 0xFF00],
            0xFF80...0xFFFE => &self.hram[index - 0xFF80],
            0xFFFF => &self.interrupt[index - 0xFFFF],
            _ => panic!("Address {:0>2X} has no known mapping!")
        }
    }
}

impl IndexMut<u16> for Interconnect {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        &mut self[index as usize]
    }
}

impl IndexMut<usize> for Interconnect {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        match index {
            0x0000...0x00FF => &mut self.boot[index - 0x0000],
            0x0100...0x7FFF => &mut self.cart[index - 0x0100],
            0x8000...0x9FFF => &mut self.vram[index - 0x8000],
            0xA000...0xBFFF => &mut self.xram[index - 0xA000],
            0xC000...0xDFFF => &mut self.wram[index - 0xC000],
            0xE000...0xFDFF => &mut self.wram[index - 0xE000],
            0xFF00 => &mut self.input[index - 0xFF00],
            0xFF80...0xFFFE => &mut self.hram[index - 0xFF80],
            0xFFFF => &mut self.interrupt[index - 0xFFFF],
            _ => panic!("Address {:0>2X} has no known mapping!")
        }
    }
}
