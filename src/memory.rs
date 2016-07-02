use gameboy;
use cart;
const WRAM_SIZE: usize = 0xDFFF - 0xC000 + 1;
const VRAM_SIZE: usize = 0x9FFF - 0x8000 + 1;
const XRAM_SIZE: usize = 0xBFFF - 0xA000 + 1;
const HRAM_SIZE: usize = 0xFFFE - 0xFF80 + 1;
const IO_SIZE: usize = 0xFF7F - 0xFF01 + 1;

use std::ops::{Index, IndexMut, Range};

pub struct Memory {
    count: u16,
    boot: [u8; gameboy::BOOTROM_SIZE],
    cart: cart::Cart,
    wram: [u8; WRAM_SIZE],
    vram: [u8; VRAM_SIZE],
    xram: [u8; XRAM_SIZE],
    input: [u8; 1],
    io: [u8; IO_SIZE],
    hram: [u8; HRAM_SIZE],
    interrupt: [u8; 1],
    zero: [u8; 1],
}

impl Memory {
    pub fn new(boot: [u8; gameboy::BOOTROM_SIZE], cart: cart::Cart) -> Memory {
        Memory {
            count: 0xFFFF,
            boot: boot,
            cart: cart,
            wram: [127; WRAM_SIZE],
            vram: [127; VRAM_SIZE],
            xram: [0; XRAM_SIZE],
            input: [0],
            io: [0; IO_SIZE],
            hram: [0; HRAM_SIZE],
            interrupt: [0],
            zero: [0],
        }
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        match index {
            0xFF00 => &self.input[0],
            0xFFFF => &self.interrupt[0],
            _ => &self[index..index + 1][0],
        }
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index..index + 1][0]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        &mut self[index as usize]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        match index {
            0x0000...0x00FF => &mut self.boot[index - 0x0000],
            0x0100...0x7FFF => &mut self.cart[index - 0x0100],
            0x8000...0x9FFF => &mut self.vram[index - 0x8000],
            0xA000...0xBFFF => &mut self.xram[index - 0xA000],
            0xC000...0xDFFF => &mut self.wram[index - 0xC000],
            0xE000...0xFDFF => &mut self.wram[index - 0xE000],
            0xFF00 => &mut self.input[index - 0xFF00],
            0xFF01...0xFF7F => &mut self.io[index - 0xFF01],
            0xFF80...0xFFFE => &mut self.hram[index - 0xFF80],
            0xFFFF => &mut self.interrupt[index - 0xFFFF],
            _ => panic!("Address {:0>2X} has no known mapping!", index),
        }
    }
}

impl Index<Range<u16>> for Memory {
    type Output = [u8];
    fn index(&self, range: Range<u16>) -> &Self::Output {
        let usize_range = (range.start as usize)..(range.end as usize);
        &self[usize_range]
    }
}

impl Index<Range<usize>> for Memory {
    type Output = [u8];
    fn index(&self, range: Range<usize>) -> &Self::Output {
        let end = if range.end.wrapping_sub(range.start) == 1 {
            range.start
        } else {
            range.end
        };

        match (range.start, end) {
            (0x0000...0x00FF, 0x0000...0x00FF) => {
                &self.boot[(range.start - 0x0000)..(range.end - 0x0000)]
            }
            (0x0100...0x7FFF, 0x0100...0x7FFF) => {
                &self.cart[(range.start - 0x0100)..(range.end - 0x0100)]
            }
            (0x8000...0x9FFF, 0x8000...0x9FFF) => {
                &self.vram[(range.start - 0x8000)..(range.end - 0x8000)]
            }
            (0xA000...0xBFFF, 0xA000...0xBFFF) => {
                &self.xram[(range.start - 0xA000)..(range.end - 0xA000)]
            }
            (0xC000...0xDFFF, 0xC000...0xDFFF) => {
                &self.wram[(range.start - 0xC000)..(range.end - 0xC000)]
            }
            (0xE000...0xFDFF, 0xE000...0xFDFF) => {
                &self.wram[(range.start - 0xE000)..(range.end - 0xE000)]
            }
            (0xFE00...0xFEFF, 0xFE00...0xFEFF) => &self.zero[..],
            (0xFF00, 0xFF00) => &self.input[..],
            (0xFF01...0xFF7F, 0xFF01...0xFF7F) => {
                &self.io[(range.start - 0xFF01)..(range.end - 0xFF01)]
            }
            (0xFF80...0xFFFF, 0xFF80...0xFFFE) => {
                &self.hram[(range.start - 0xFF80)..(range.end - 0xFF80)]
            }
            (0xFFFF, 0x0000) => &self.interrupt[..],
            _ => {
                panic!("Address {:0>4X}..{:0>4X} has no known mapping!",
                       range.start,
                       end)
            }
        }
    }
}

impl Iterator for Memory {
    // we will be counting with usize
    type Item = u8;

    // next() is the only required method
    fn next(&mut self) -> Option<u8> {
        // increment our count. This is why we started at zero.
        self.count -= 1;

        // check to see if we've finished counting or not.
        if self.count > 0 {
            Some(self[self.count])
        } else {
            None
        }
    }
}
