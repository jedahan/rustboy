use std::sync::{Arc, RwLock};

use cpu;
use lcd;
use cart;
use memory;

pub const BOOTROM_SIZE: usize = 256;

pub fn run(boot: [u8; BOOTROM_SIZE], cart: cart::Cart) {
    let memory = Arc::new(RwLock::new(memory::Memory::new(boot, cart)));

    let mut cpu = cpu::Cpu::new(memory.clone());
    let mut lcd = lcd::LcdScreen::new(160, 144, memory.clone());

    loop {
        cpu.step();
        lcd.step();
    }
}
