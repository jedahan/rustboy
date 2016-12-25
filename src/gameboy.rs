use std::sync::{Arc, RwLock};

use cpu;
use cart;
use memory;

pub const BOOTROM_SIZE: usize = 256;

pub fn run(boot: [u8; BOOTROM_SIZE], cart: cart::Cart) {
    let memory = Arc::new(RwLock::new(memory::Memory::new(boot, cart)));

    let mut cpu = cpu::Cpu::new(memory);

    cpu.run();
}
